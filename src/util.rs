use std::time::{SystemTime, UNIX_EPOCH};

// pub fn timestamp_ms() -> u64 {
//     SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .map(|d| d.as_millis() as u64)
//         .expect("system time before UNIX_EPOCH")
// }

pub fn timestamp_us() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as u64)
        .expect("system time before UNIX_EPOCH")
}
