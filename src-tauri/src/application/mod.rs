pub mod compute_series;
pub mod today_summary;

use time::{Date, PrimitiveDateTime, Time};

pub(crate) fn day_range(date: Date) -> (PrimitiveDateTime, PrimitiveDateTime) {
    let start = PrimitiveDateTime::new(date, Time::MIDNIGHT);
    let end = PrimitiveDateTime::new(date, Time::from_hms(23, 59, 59).expect("valid time"));
    (start, end)
}
