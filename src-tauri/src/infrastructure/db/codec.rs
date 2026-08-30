use time::format_description::FormatItem;
use time::macros::format_description;
use time::{error::Parse, Date, PrimitiveDateTime};

const DATETIME_FORMAT: &[FormatItem] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
const DATE_FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day]");

pub fn format_datetime(dt: PrimitiveDateTime) -> String {
    dt.format(DATETIME_FORMAT)
        .expect("well-formed datetime format description")
}

pub fn parse_datetime(s: &str) -> Result<PrimitiveDateTime, Parse> {
    PrimitiveDateTime::parse(s, DATETIME_FORMAT)
}

pub fn format_date(d: Date) -> String {
    d.format(DATE_FORMAT)
        .expect("well-formed date format description")
}

pub fn parse_date(s: &str) -> Result<Date, Parse> {
    Date::parse(s, DATE_FORMAT)
}
