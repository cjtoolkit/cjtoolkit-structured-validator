use cjtoolkit_structured_validator::common::locale::{LocaleData, LocaleMessage};
use cjtoolkit_structured_validator::common::validation_check::ValidationCheck;
use cjtoolkit_structured_validator::common::validation_collector::AsValidateErrorStore;
use cjtoolkit_structured_validator::types::AsStringOnResult;
use cjtoolkit_structured_validator::types::name::{Name, NameError, NameRules};
use regex::Regex;
use std::sync::{Arc, OnceLock};

static POSTCODE_REGEX_CACHE: OnceLock<Regex> = OnceLock::new();

fn validate_postcode(postcode: &str) -> bool {
    let regex = POSTCODE_REGEX_CACHE.get_or_init(|| {
        Regex::new(r"^[A-Z]{1,2}[0-9R][0-9A-Z]? [0-9][ABD-HJLNP-UW-Z]{2}$").expect("Invalid Regex")
    });
    regex.is_match(postcode)
}

struct PostcodeLocale;

impl LocaleMessage for PostcodeLocale {
    fn get_locale_data(&self) -> Arc<LocaleData> {
        LocaleData::new("validate-postcode")
    }
}

trait ParsePostCode {
    fn parse_postcode(s: Option<&str>) -> Result<Name, NameError>;
}

impl ParsePostCode for Name {
    fn parse_postcode(s: Option<&str>) -> Result<Name, NameError> {
        let postcode = Self::parse_custom(
            s,
            NameRules {
                is_mandatory: true,
                min_length: Some(7),
                max_length: Some(10),
            },
        );
        let mut messages = postcode.as_validate_error_collector();
        if !validate_postcode(postcode.as_string().as_str()) {
            messages.push(("Invalid Postcode".to_string(), Box::new(PostcodeLocale)));
        }
        NameError::validate_check(messages)?;
        postcode
    }
}

type Postcode = Name;

fn main() {
    let postcode = Postcode::parse_postcode(Some("SW1A 1AA"));
    println!("{:?}", postcode);

    let postcode = Postcode::parse_postcode(Some("SW1A 1A"));
    println!("{:?}", postcode);
}

#[test]
fn test_validate_postcode() {
    assert!(validate_postcode("SW1A 1AA"));
    assert!(!validate_postcode("SW1A 1A"));
}
