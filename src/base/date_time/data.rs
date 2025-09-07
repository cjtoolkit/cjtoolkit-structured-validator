use std::fmt::Display;

/// `DateTimeKind` is an enumeration that represents different kinds of date and time representations.
/// It is marked with the `#[derive(Default, Clone)]` attribute, allowing instances of the enum
/// to be cloned and providing a default value.
///
/// # Variants
///
/// * `Date` - Represents a date-only value (e.g., year, month, and day).
/// * `DateTime` - Represents a date and time value with timezone awareness. This is the default variant.
/// * `DateTimeNaive` - Represents a date and time value without timezone awareness.
/// * `Time` - Represents a time-only value (e.g., hours, minutes, seconds).
///
/// # Default
///
/// The `DateTime` variant is the default variant of this enum, as specified by the `#[default]` attribute.
#[derive(Default, Clone)]
pub enum DateTimeKind {
    Date,
    #[default]
    DateTime,
    DateTimeNaive,
    Time,
}

/// A data structure representing detailed information about a specific date and time.
/// This struct is primarily used to encapsulate information such as the date type, a formatted string,
/// timestamp details in seconds, and sub-second precision in nanoseconds.
///
/// # Fields
///
/// * `kind` - Defines the kind of the date-time.
///   It is represented using the `DateTimeKind` enum.
/// * `date_formatted` - A `String` containing the formatted representation of the date and time.
///   This is useful for display or logging purposes.
/// * `timestamp_seconds_days` - An `i64` value representing the number of seconds since or until
///   a reference time (typically the UNIX epoch, 1970-01-01 00:00:00 UTC).
///   This could also be used to store the number of seconds in terms of days for time calculations.
/// * `subsec_nano` - A `u32` representing the nanosecond portion of the timestamp,
///   providing sub-second precision.
///
/// # Notes
///
/// * This struct implements the `Default` trait, providing a convenient way to create an instance
///   with default values.
/// * It also implements the `Clone` trait, allowing the struct to be copied efficiently.
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

/// A trait that provides a method to convert or extract an implementing type into a `DateTimeData`.
///
/// Types that implement this trait must define how they can transform or expose
/// their data in the form of a `DateTimeData` structure.
///
/// # Required Methods
///
/// * `fn as_date_time_data(&self) -> DateTimeData`
///
///   This method is expected to return a `DateTimeData` value that represents
///   the internal datetime-related data of the implementing type.
///
/// # Use Cases
///
/// This trait is useful when multiple types need to standardize their access to
/// datetime data in the form of a common abstraction (`DateTimeData`).
/// Common scenarios could include logging, scheduling, or serialization work.
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

#[cfg(feature = "humantime")]
mod humantime_impl {
    use super::*;
    use humantime::Timestamp;
    use std::time::SystemTime;

    impl AsDateTimeData for Timestamp {
        fn as_date_time_data(&self) -> DateTimeData {
            let system_time: SystemTime = self.clone().into();
            let duration_from_unix = system_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default();
            DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: humantime::format_rfc3339(system_time).to_string(),
                timestamp_seconds_days: duration_from_unix.as_secs() as i64,
                subsec_nano: duration_from_unix.subsec_nanos(),
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
