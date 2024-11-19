// Automatically generated rust module for 'message.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ClientMessage {
    pub msg: mod_ClientMessage::OneOfmsg,
}

impl<'a> MessageRead<'a> for ClientMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg = mod_ClientMessage::OneOfmsg::ping(r.read_message::<Ping>(bytes)?),
                Ok(18) => msg.msg = mod_ClientMessage::OneOfmsg::pong(r.read_message::<Pong>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ClientMessage {
    fn get_size(&self) -> usize {
        0
        + match self.msg {
            mod_ClientMessage::OneOfmsg::ping(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ClientMessage::OneOfmsg::pong(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ClientMessage::OneOfmsg::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.msg {            mod_ClientMessage::OneOfmsg::ping(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_ClientMessage::OneOfmsg::pong(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ClientMessage::OneOfmsg::None => {},
    }        Ok(())
    }
}

pub mod mod_ClientMessage {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfmsg {
    ping(Ping),
    pong(Pong),
    None,
}

impl Default for OneOfmsg {
    fn default() -> Self {
        OneOfmsg::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ping {
    pub ts_us: u64,
}

impl<'a> MessageRead<'a> for Ping {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ts_us = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Ping {
    fn get_size(&self) -> usize {
        0
        + if self.ts_us == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ts_us) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ts_us != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.ts_us))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Pong {
    pub ts_us: u64,
    pub recv_ts_us: u64,
}

impl<'a> MessageRead<'a> for Pong {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ts_us = r.read_uint64(bytes)?,
                Ok(16) => msg.recv_ts_us = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Pong {
    fn get_size(&self) -> usize {
        0
        + if self.ts_us == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ts_us) as u64) }
        + if self.recv_ts_us == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.recv_ts_us) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ts_us != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.ts_us))?; }
        if self.recv_ts_us != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.recv_ts_us))?; }
        Ok(())
    }
}

