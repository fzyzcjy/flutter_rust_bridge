// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `chrono_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

pub async fn datetime_utc_twin_rust_async(
    d: chrono::DateTime<chrono::Utc>,
) -> chrono::DateTime<chrono::Utc> {
    use chrono::Datelike;
    use chrono::Timelike;
    assert_eq!(&d.year(), &2022);
    assert_eq!(&d.month(), &9);
    assert_eq!(&d.day(), &10);
    assert_eq!(&d.hour(), &20);
    assert_eq!(&d.minute(), &48);
    assert_eq!(&d.second(), &53);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(&d.nanosecond(), &123_000_000);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(&d.nanosecond(), &123_456_000);
    d
}

pub async fn datetime_local_twin_rust_async(
    d: chrono::DateTime<chrono::Local>,
) -> chrono::DateTime<chrono::Local> {
    use chrono::Datelike;
    use chrono::Timelike;
    assert_eq!(&d.year(), &2022);
    assert_eq!(&d.month(), &9);
    assert_eq!(&d.day(), &10);
    if cfg!(target_arch = "wasm32") {
        assert_eq!(&d.nanosecond(), &123_000_000);
    } else {
        assert_eq!(&d.hour(), &20);
        assert_eq!(&d.nanosecond(), &123_456_000);
    }
    assert_eq!(&d.minute(), &48);
    assert_eq!(&d.second(), &53);
    d
}

pub async fn naivedatetime_twin_rust_async(d: chrono::NaiveDateTime) -> chrono::NaiveDateTime {
    use chrono::{Datelike, Timelike};
    assert_eq!(&d.year(), &2022);
    assert_eq!(&d.month(), &9);
    assert_eq!(&d.day(), &10);
    assert_eq!(&d.hour(), &20);
    assert_eq!(&d.minute(), &48);
    assert_eq!(&d.second(), &53);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(&d.nanosecond(), &123_000_000);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(&d.nanosecond(), &123_456_000);
    d
}

pub async fn optional_empty_datetime_utc_twin_rust_async(
    d: Option<chrono::DateTime<chrono::Utc>>,
) -> Option<chrono::DateTime<chrono::Utc>> {
    assert_eq!(&d, &None);
    d
}

pub async fn duration_twin_rust_async(d: chrono::Duration) -> chrono::Duration {
    assert_eq!(&d.num_hours(), &4);
    d
}

pub async fn handle_timestamps_twin_rust_async(
    timestamps: Vec<chrono::NaiveDateTime>,
    epoch: chrono::NaiveDateTime,
) -> Vec<chrono::Duration> {
    timestamps
        .into_iter()
        .map(|ts| epoch.signed_duration_since(ts))
        .collect()
}

pub async fn handle_durations_twin_rust_async(
    durations: Vec<chrono::Duration>,
    since: chrono::DateTime<chrono::Local>,
) -> Vec<chrono::DateTime<chrono::Local>> {
    durations.into_iter().map(|dur| since - dur).collect()
}

pub struct TestChronoTwinRustAsync {
    pub dt: Option<chrono::DateTime<chrono::Utc>>,
    pub dt2: Option<chrono::NaiveDateTime>,
    pub du: Option<chrono::Duration>,
}

pub async fn test_chrono_twin_rust_async() -> TestChronoTwinRustAsync {
    TestChronoTwinRustAsync {
        dt: Some(chrono::DateTime::from_naive_utc_and_offset(
            chrono::DateTime::from_timestamp(1631297333, 0)
                .unwrap()
                .naive_utc(),
            chrono::Utc,
        )),
        dt2: Some(
            chrono::DateTime::from_timestamp(1631297333, 0)
                .unwrap()
                .naive_utc(),
        ),
        du: Some(chrono::Duration::hours(4)),
    }
}

pub async fn test_precise_chrono_twin_rust_async() -> TestChronoTwinRustAsync {
    TestChronoTwinRustAsync {
        dt: Some(chrono::DateTime::from_naive_utc_and_offset(
            chrono::DateTime::from_timestamp(1014466435, 0)
                .unwrap()
                .naive_utc(),
            chrono::Utc,
        )),
        dt2: Some(
            chrono::DateTime::from_timestamp(-5362715015, 0)
                .unwrap()
                .naive_utc(),
        ),
        du: Some(chrono::Duration::hours(4)),
    }
}

#[derive(Debug, Clone)]
pub struct FeatureChronoTwinRustAsync {
    pub utc: chrono::DateTime<chrono::Utc>,
    pub local: chrono::DateTime<chrono::Local>,
    pub duration: chrono::Duration,
    pub naive: chrono::NaiveDateTime,
}

pub async fn how_long_does_it_take_twin_rust_async(
    mine: FeatureChronoTwinRustAsync,
) -> anyhow::Result<chrono::Duration> {
    use chrono::{Datelike, Timelike};
    let difference: chrono::Duration = chrono::Utc::now() - mine.utc;
    assert_eq!(&mine.duration.num_hours(), &4);
    assert_eq!(&mine.naive.year(), &2022);
    assert_eq!(&mine.naive.month(), &9);
    assert_eq!(&mine.naive.day(), &10);
    assert_eq!(&mine.naive.hour(), &20);
    assert_eq!(&mine.naive.minute(), &48);
    assert_eq!(&mine.naive.second(), &53);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(&mine.naive.nanosecond(), &123_000_000);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(&mine.naive.nanosecond(), &123_456_000);
    Ok(difference)
}
