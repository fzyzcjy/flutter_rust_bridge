#[cfg(feature = "chrono")]
#[inline]
pub fn wire2api_timestamp(ts: i64) -> Timestamp {
    #[cfg(wasm)]
    const PRECISION: i64 = 1_000;
    #[cfg(not(wasm))]
    const PRECISION: i64 = 1_000_000;

    let s = ts / PRECISION;
    let ns = (ts.rem_euclid(PRECISION) * (1_000_000_000 / PRECISION)) as u32;
    Timestamp { s, ns }
}

/// a timestamp with microseconds precision on native and milliseconds precision on web
#[cfg(feature = "chrono")]
pub struct Timestamp {
    /// seconds
    pub s: i64,
    /// nanoseconds
    pub ns: u32,
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod tests {
    #[test]
    fn test_wire2api() {
        #[cfg(not(wasm))]
        {
            // input in microseconds
            let input: i64 = 3_496_567_123;
            let super::Timestamp { s, ns } = super::wire2api_timestamp(input);
            assert_eq!(s, 3_496);
            assert_eq!(ns, 567_123_000);
        }

        #[cfg(wasm)]
        {
            // input in milliseconds
            let input: i64 = 3_496_567;
            let super::Timestamp { s, ns } = super::wire2api_timestamp(input);
            assert_eq!(s, 3_496);
            assert_eq!(ns, 567_000_000);
        }
    }
}

