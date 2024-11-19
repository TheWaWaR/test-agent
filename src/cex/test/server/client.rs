use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::Arc;

use anyhow::{anyhow, bail};
use log::{info, warn};
use quick_protobuf::MessageWrite;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpListener, TcpStream,
};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::pb::message as proto;
use crate::pb::util::{decode_msg, encode_msg};
use crate::util::timestamp_us;

use proto::mod_ClientMessage::OneOfmsg;

pub async fn handle_clients(bind: SocketAddr) -> anyhow::Result<()> {
    info!("Bind on: {bind}");
    let listener = TcpListener::bind(bind).await?;
    loop {
        let (sock, addr) = listener.accept().await?;
        info!("Accepted client {addr}");
        tokio::spawn(handle_client(sock, addr));
    }
}

async fn handle_client(sock: TcpStream, addr: SocketAddr) {
    let (read_half, write_half) = sock.into_split();
    let (ping_tx, ping_rx) = mpsc::channel::<(proto::Ping, u64)>(2);
    let running = Arc::new(AtomicBool::new(true));
    let read_running = running.clone();
    let read_task: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        tokio::task::unconstrained(handle_client_read(read_half, read_running, ping_tx, addr)).await
    });
    let write_running = running.clone();
    let write_task: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        tokio::task::unconstrained(handle_client_write(
            write_half,
            write_running,
            ping_rx,
            addr,
        ))
        .await
    });
    let (result, _, _) = futures::future::select_all([read_task, write_task]).await;
    if let Ok(Err(err)) = result {
        warn!("Client({addr}) finished with: {err}");
        running.store(false, Relaxed);
    }
}

async fn handle_client_read(
    mut read_half: OwnedReadHalf,
    read_running: Arc<AtomicBool>,
    ping_tx: mpsc::Sender<(proto::Ping, u64)>,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    info!("[{addr}] start read");
    let mut len_buf = [0u8; 4];
    let mut buf = [0u8; 4096];
    while read_running.load(Relaxed) {
        read_half.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > buf.len() {
            bail!(
                "Client send message length({}) larger than read buffer length({})",
                len,
                buf.len()
            );
        }
        read_half.read_exact(&mut buf[0..len]).await?;
        let msg: proto::ClientMessage = decode_msg(&buf[0..len])
            .map_err(|err| anyhow!("Decode client message error: {err}, len={len}"))?;
        match msg.msg {
            OneOfmsg::ping(msg) => {
                let now_us = timestamp_us();
                let delta = now_us - msg.ts_us;
                info!("Received ping message from {addr}: {msg:?}, d1={delta}");
                ping_tx.send((msg, now_us)).await?;
            }
            _ => todo!(),
        }
    }
    info!("[{addr}]Client read finished");
    Ok(())
}

async fn handle_client_write(
    mut write_half: OwnedWriteHalf,
    write_running: Arc<AtomicBool>,
    mut ping_rx: mpsc::Receiver<(proto::Ping, u64)>,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    info!("[{addr}] start write");
    let mut buf = [0u8; 4096];
    while write_running.load(Relaxed) {
        let (ping, ts_us) = ping_rx
            .recv()
            .await
            .ok_or_else(|| anyhow!("Ping sender closed!"))?;
        let now_us = timestamp_us();
        let pong = proto::Pong {
            ts_us: now_us,
            recv_ts_us: ping.ts_us,
        };
        let pong_msg = proto::ClientMessage {
            msg: OneOfmsg::pong(pong.clone()),
        };
        let len = pong_msg.get_size();
        let len_bytes = (len as u32).to_be_bytes();
        encode_msg(&pong_msg, &mut buf[..])?;
        write_half.write_all(&len_bytes[..]).await?;
        write_half.write_all(&buf[0..len]).await?;
        write_half.flush().await?;
        let delta_1 = ts_us - ping.ts_us;
        let delta_1p = now_us - ping.ts_us;
        let delta_internal = now_us - ts_us;
        info!(
            "Write pong message: {pong:?}, \
             len={len}, d1={delta_1}, d1'={delta_1p}, di={delta_internal}"
        );
    }
    info!("[{addr}]Client write finished");
    Ok(())
}
