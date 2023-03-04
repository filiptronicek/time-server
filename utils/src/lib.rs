use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_unix_times() -> (u64, u64) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let unix = since_the_epoch.as_secs();
    let unix_ms = unix * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    (unix_ms, unix);
    return (unix_ms, unix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix() {
        let (unix_ms, unix) = get_unix_times();
        println!("unix_ms: {}", unix_ms);
        println!("unix: {}", unix);
        assert!(unix_ms > 0);
        assert!(unix > 0);
        assert!(unix_ms > unix);
    }
}
