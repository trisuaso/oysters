use chrono::{TimeZone, Utc};

/// Get a [`usize`] timestamp from the given `year` epoch
pub fn epoch_timestamp(year: u16) -> usize {
    let now = Utc::now().timestamp_millis();
    let then = Utc
        .with_ymd_and_hms(year as i32, 1, 1, 0, 0, 0)
        .unwrap()
        .timestamp_millis();

    (now - then) as usize
}
