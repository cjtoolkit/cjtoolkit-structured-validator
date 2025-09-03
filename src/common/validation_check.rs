use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};

pub trait ValidationCheck: Sized {
    fn validate_new(msgs: ValidateErrorStore) -> Self;

    fn validate_check(msgs: ValidateErrorCollector) -> Result<(), Self> {
        if msgs.is_empty() {
            Ok(())
        } else {
            Err(Self::validate_new(msgs.into()))
        }
    }
}
