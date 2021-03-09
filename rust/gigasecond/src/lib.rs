use chrono::{DateTime, Duration, Utc};

const GIGASECOND: i64 = 1_000_000_000;

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    return start + Duration::seconds(GIGASECOND);
}
