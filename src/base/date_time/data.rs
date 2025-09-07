use std::fmt::Display;

#[derive(Default, Clone)]
pub enum DateTimeKind {
    Date,
    #[default]
    DateTime,
    DateTimeNaive,
    Time,
}

#[derive(Default, Clone)]
pub struct DateTimeData {
    pub kind: DateTimeKind,
    pub date_formatted: String,
    pub timestamp_seconds_days: i64,
    pub subsec_nano: u32,
}

impl Display for DateTimeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.date_formatted)
    }
}

impl PartialEq for DateTimeData {
    fn eq(&self, other: &Self) -> bool {
        (self.timestamp_seconds_days, self.subsec_nano)
            == (other.timestamp_seconds_days, other.subsec_nano)
    }
}

impl PartialOrd for DateTimeData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.timestamp_seconds_days, self.subsec_nano)
            .partial_cmp(&(other.timestamp_seconds_days, other.subsec_nano))
    }
}

pub trait AsDateTimeData {
    fn as_date_time_data(&self) -> DateTimeData;
}

#[cfg(feature = "chrono")]
mod chrono_impl {
    use super::*;
    use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike};

    impl<Tz: TimeZone> AsDateTimeData for DateTime<Tz> {
        fn as_date_time_data(&self) -> DateTimeData {
            DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: self.to_rfc3339(),
                timestamp_seconds_days: self.timestamp(),
                subsec_nano: self.timestamp_subsec_nanos(),
            }
        }
    }

    impl AsDateTimeData for NaiveDate {
        fn as_date_time_data(&self) -> DateTimeData {
            DateTimeData {
                kind: DateTimeKind::Date,
                date_formatted: self.to_string(),
                timestamp_seconds_days: self.num_days_from_ce() as i64,
                subsec_nano: 0,
            }
        }
    }

    impl AsDateTimeData for (Option<&str>, &NaiveDate) {
        fn as_date_time_data(&self) -> DateTimeData {
            match self.0 {
                Some(format) => DateTimeData {
                    kind: DateTimeKind::Date,
                    date_formatted: self.1.format(format).to_string(),
                    timestamp_seconds_days: self.1.num_days_from_ce() as i64,
                    subsec_nano: 0,
                },
                None => self.1.as_date_time_data(),
            }
        }
    }

    impl AsDateTimeData for NaiveDateTime {
        fn as_date_time_data(&self) -> DateTimeData {
            let as_utc = self.and_utc();
            DateTimeData {
                kind: DateTimeKind::DateTimeNaive,
                date_formatted: as_utc.to_string(),
                timestamp_seconds_days: as_utc.timestamp(),
                subsec_nano: as_utc.timestamp_subsec_nanos(),
            }
        }
    }

    impl AsDateTimeData for (Option<&str>, &NaiveDateTime) {
        fn as_date_time_data(&self) -> DateTimeData {
            match self.0 {
                Some(format) => {
                    let as_utc = self.1.and_utc();
                    DateTimeData {
                        kind: DateTimeKind::DateTimeNaive,
                        date_formatted: self.1.format(format).to_string(),
                        timestamp_seconds_days: as_utc.timestamp(),
                        subsec_nano: as_utc.timestamp_subsec_nanos(),
                    }
                }
                None => self.1.as_date_time_data(),
            }
        }
    }

    impl AsDateTimeData for NaiveTime {
        fn as_date_time_data(&self) -> DateTimeData {
            DateTimeData {
                kind: DateTimeKind::Time,
                date_formatted: self.to_string(),
                timestamp_seconds_days: self.num_seconds_from_midnight() as i64,
                subsec_nano: self.nanosecond(),
            }
        }
    }

    impl AsDateTimeData for (Option<&str>, &NaiveTime) {
        fn as_date_time_data(&self) -> DateTimeData {
            match self.0 {
                Some(format) => DateTimeData {
                    kind: DateTimeKind::Time,
                    date_formatted: self.1.format(format).to_string(),
                    timestamp_seconds_days: self.1.num_seconds_from_midnight() as i64,
                    subsec_nano: self.1.nanosecond(),
                },
                None => self.1.as_date_time_data(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        let a = DateTimeData {
            kind: DateTimeKind::DateTime,
            date_formatted: "en".to_string(),
            timestamp_seconds_days: 1,
            subsec_nano: 1,
        };
        let b = DateTimeData {
            kind: DateTimeKind::DateTime,
            date_formatted: "en".to_string(),
            timestamp_seconds_days: 1,
            subsec_nano: 2,
        };

        assert!(a < b);
    }
}
