pub use super::DartAbi;
pub use super::MessagePort;
pub use allo_isolate::*;

#[cfg(feature = "chrono")]
#[inline]
pub fn wire2api_timestamp(ts: i64) -> (i64, u32) {
    let s = (ts / 1_000_000) as i64;
    let ns = (ts.rem_euclid(1_000_000) * 1_000) as u32;
    (s, ns)
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod tests {
    #[test]
    fn wire2api() {
        // input in microseconds
        let input: i64 = 3_496_567_123;
        let (s, ns) = super::wire2api_timestamp(input);
        assert_eq!(s, 3_496);
        assert_eq!(ns, 567_123_000);
    }
}
