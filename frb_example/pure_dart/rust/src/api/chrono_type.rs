pub fn datetime_utc(d: chrono::DateTime<chrono::Utc>) -> chrono::DateTime<chrono::Utc> {
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

pub fn datetime_local(d: chrono::DateTime<chrono::Local>) -> chrono::DateTime<chrono::Local> {
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

pub fn naivedatetime(d: chrono::NaiveDateTime) -> chrono::NaiveDateTime {
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

pub fn optional_empty_datetime_utc(
    d: Option<chrono::DateTime<chrono::Utc>>,
) -> Option<chrono::DateTime<chrono::Utc>> {
    assert_eq!(&d, &None);
    d
}

pub fn duration(d: chrono::Duration) -> chrono::Duration {
    assert_eq!(&d.num_hours(), &4);
    d
}

pub fn handle_timestamps(
    timestamps: Vec<chrono::NaiveDateTime>,
    epoch: chrono::NaiveDateTime,
) -> Vec<chrono::Duration> {
    timestamps
        .into_iter()
        .map(|ts| epoch.signed_duration_since(ts))
        .collect()
}

pub fn handle_durations(
    durations: Vec<chrono::Duration>,
    since: chrono::DateTime<chrono::Local>,
) -> Vec<chrono::DateTime<chrono::Local>> {
    durations.into_iter().map(|dur| since - dur).collect()
}

pub struct TestChrono {
    pub dt: Option<chrono::DateTime<chrono::Utc>>,
    pub dt2: Option<chrono::NaiveDateTime>,
    pub du: Option<chrono::Duration>,
}

pub fn test_chrono() -> TestChrono {
    TestChrono {
        dt: Some(chrono::DateTime::from_naive_utc_and_offset(
            chrono::NaiveDateTime::from_timestamp_opt(1631297333, 0).unwrap(),
            chrono::Utc,
        )),
        dt2: Some(chrono::NaiveDateTime::from_timestamp_opt(1631297333, 0).unwrap()),
        du: Some(chrono::Duration::hours(4)),
    }
}

pub fn test_precise_chrono() -> TestChrono {
    TestChrono {
        dt: Some(chrono::DateTime::from_naive_utc_and_offset(
            chrono::NaiveDateTime::from_timestamp_opt(1014466435, 0).unwrap(),
            chrono::Utc,
        )),
        dt2: Some(chrono::NaiveDateTime::from_timestamp_opt(-5362715015, 0).unwrap()),
        du: Some(chrono::Duration::hours(4)),
    }
}

#[derive(Debug, Clone)]
pub struct FeatureChrono {
    pub utc: chrono::DateTime<chrono::Utc>,
    pub local: chrono::DateTime<chrono::Local>,
    pub duration: chrono::Duration,
    pub naive: chrono::NaiveDateTime,
}

pub fn how_long_does_it_take(mine: FeatureChrono) -> anyhow::Result<chrono::Duration> {
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
