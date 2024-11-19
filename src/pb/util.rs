use quick_protobuf::errors::Result;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer};

pub fn decode_msg<'a, T: MessageRead<'a>>(buf: &'a [u8]) -> Result<T> {
    let mut reader = BytesReader::from_bytes(buf);
    T::from_reader(&mut reader, buf)
}

pub fn encode_msg<T: MessageWrite>(msg: &T, buf: &mut [u8]) -> Result<()> {
    let mut writer = Writer::new(&mut buf[..]);
    msg.write_message(&mut writer)
}
