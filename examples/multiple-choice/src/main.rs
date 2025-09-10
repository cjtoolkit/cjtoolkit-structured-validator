use cjtoolkit_structured_validator::common::locale::{LocaleData, LocaleMessage};
use cjtoolkit_structured_validator::common::validation_check::ValidationCheck;
use cjtoolkit_structured_validator::types::name::{Name, NameError, NameRules};
use std::sync::Arc;

const CHOICES_OF_FRUITTS: [&str; 3] = ["apple", "banana", "orange"];

fn check_fruit_choice(choice: &str) -> bool {
    CHOICES_OF_FRUITTS.contains(&choice)
}

struct FruitChoiceLocale;

impl LocaleMessage for FruitChoiceLocale {
    fn get_locale_data(&self) -> Arc<LocaleData> {
        LocaleData::new("validate-fruit-choice")
    }
}

trait FruitChoice {
    fn parse_fruit_choice(s: Option<&str>) -> Result<Name, NameError>;
}

impl FruitChoice for Name {
    fn parse_fruit_choice(s: Option<&str>) -> Result<Name, NameError> {
        let subject = Self::parse_custom(
            s,
            NameRules {
                is_mandatory: true,
                min_length: None,
                max_length: None,
            },
        );
        let mut messages = subject
            .as_ref()
            .err()
            .map(|e| e.0.as_validate_error_collector())
            .unwrap_or_default();
        if !check_fruit_choice(
            subject
                .as_ref()
                .ok()
                .map(|v| v.as_str())
                .unwrap_or_default(),
        ) {
            messages.push((
                r#"Invalid Fruit, valid fruit are 'apple', 'banana' and 'orange'"#.to_string(),
                Box::new(FruitChoiceLocale),
            ));
        }
        NameError::validate_check(messages)?;
        subject
    }
}

type Fruit = Name;

fn main() {
    let fruit = Fruit::parse_fruit_choice(Some("apple"));
    println!("{:?}", fruit);

    let fruit = Fruit::parse_fruit_choice(Some("pear"));
    println!("{:?}", fruit);
}

#[test]
fn test_check_fruit_choice() {
    assert!(check_fruit_choice("apple"));
    assert!(check_fruit_choice("banana"));
    assert!(check_fruit_choice("orange"));
    assert!(!check_fruit_choice("pear"));
}
