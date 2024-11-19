use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail};
use log::{info, warn};
use quick_protobuf::MessageWrite;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream,
};
use tokio::task::JoinHandle;
use tokio::time::interval;

use crate::pb::message as proto;
use crate::pb::util::{decode_msg, encode_msg};
use crate::util::timestamp_us;

use proto::mod_ClientMessage::OneOfmsg;

pub async fn start(addr: SocketAddr) -> anyhow::Result<()> {
    let conn = TcpStream::connect(addr).await?;
    let (read_half, write_half) = conn.into_split();
    let running = Arc::new(AtomicBool::new(true));

    let read_running = running.clone();
    let read_task: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        tokio::task::unconstrained(handle_read(read_half, read_running)).await
    });
    let write_running = running.clone();
    let write_task: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        tokio::task::unconstrained(handle_write(write_half, write_running)).await
    });
    let (result, _, _) = futures::future::select_all([read_task, write_task]).await;
    if let Ok(Err(err)) = result {
        warn!("Client({addr}) finished with: {err}");
        running.store(false, Relaxed);
    }
    Ok(())
}

async fn handle_read(
    mut read_half: OwnedReadHalf,
    read_running: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let mut len_buf = [0u8; 4];
    let mut buf = [0u8; 4096];
    while read_running.load(Relaxed) {
        read_half
            .read_exact(&mut len_buf)
            .await
            .map_err(|err| anyhow!("Read len error: {err}"))?;
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > buf.len() {
            bail!(
                "Agent send message length({}) larger than read buffer length({})",
                len,
                buf.len()
            );
        }
        read_half
            .read_exact(&mut buf[0..len])
            .await
            .map_err(|err| anyhow!("Read message body error: {err}"))?;
        let msg: proto::ClientMessage = decode_msg(&buf[0..len])
            .map_err(|err| anyhow!("Decode agent message error: {err}, len={len}"))?;
        match msg.msg {
            OneOfmsg::pong(msg) => {
                let now_us = timestamp_us();
                let delta_1 = msg.ts_us - msg.recv_ts_us;
                let delta_2 = now_us - msg.ts_us;
                let delta_total = delta_1 + delta_2;
                info!(
                    "Received pong message: {msg:?}, d1={}us, d2={}us, dA={}us",
                    delta_1, delta_2, delta_total,
                );
            }
            _ => todo!(),
        }
    }
    info!("Read finished!");
    Ok(())
}

async fn handle_write(
    mut write_half: OwnedWriteHalf,
    write_running: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let mut buf = [0u8; 4096];
    let mut interval = interval(Duration::from_secs(5));
    while write_running.load(Relaxed) {
        interval.tick().await;

        let ts_us = timestamp_us();
        let ping = proto::Ping { ts_us };
        let ping_msg = proto::ClientMessage {
            msg: OneOfmsg::ping(ping.clone()),
        };
        let len = ping_msg.get_size();
        let len_bytes = (len as u32).to_be_bytes();
        encode_msg(&ping_msg, &mut buf[..])?;
        let t1 = timestamp_us();
        write_half.write_all(&len_bytes[..]).await?;
        write_half.write_all(&buf[0..len]).await?;
        let t2 = timestamp_us();
        write_half.flush().await?;
        let t3 = timestamp_us();
        info!(
            "encode cost: {}us, write cost: {}us, flush cost: {}us",
            t1 - ts_us,
            t2 - t1,
            t3 - t2
        );
        info!("Write ping message {ping:?}, len={len}");
    }
    info!("Write finished!");
    Ok(())
}
