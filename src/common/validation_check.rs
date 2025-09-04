use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};

pub trait ValidationCheck: Sized {
    fn validate_new(messages: ValidateErrorStore) -> Self;

    fn validate_check(messages: ValidateErrorCollector) -> Result<(), Self> {
        if messages.is_empty() {
            Ok(())
        } else {
            Err(Self::validate_new(messages.into()))
        }
    }
}
