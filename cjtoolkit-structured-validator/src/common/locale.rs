//! This module contains structures and traits for working with locales and localization.

pub use crate::common::validation_collector::{ValidateErrorCollector, ValidateErrorStore};
use std::collections::HashMap;
use std::sync::Arc;

/// Represents various types of values associated with a locale.
///
/// `LocaleValue` is an enum that allows for storage and handling of multiple data types.
/// This can be useful in scenarios where locale-specific values need to support different kinds of data.
///
/// Variants:
/// - `String(String)`: Stores a UTF-8 encoded string value.
/// - `Uint(usize)`: Stores an unsigned integer value.
/// - `Int(isize)`: Stores a signed integer value.
/// - `Float(f64)`: Stores a floating-point number value.
///
/// The `Clone` trait is implemented for `LocaleValue`, allowing instances of this enum to be duplicated.
///
/// Example:
/// ```
/// use cjtoolkit_structured_validator::common::locale::LocaleValue;
/// let string_locale = LocaleValue::String(String::from("Hello"));
/// let unsigned_locale = LocaleValue::Uint(42);
/// let signed_locale = LocaleValue::Int(-7);
/// let float_locale = LocaleValue::Float(3.14);
///
/// match string_locale {
///     LocaleValue::String(s) => println!("String value: {}", s),
///     _ => println!("Not a string"),
/// }
/// ```
#[derive(Clone)]
pub enum LocaleValue {
    String(String),
    Uint(usize),
    Int(isize),
    Float(f64),
}

impl From<String> for LocaleValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for LocaleValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<usize> for LocaleValue {
    fn from(s: usize) -> Self {
        Self::Uint(s)
    }
}

impl From<isize> for LocaleValue {
    fn from(s: isize) -> Self {
        Self::Int(s)
    }
}

impl From<f64> for LocaleValue {
    fn from(s: f64) -> Self {
        Self::Float(s)
    }
}

/**
 * Represents the localization data for a specific locale.
 * This structure holds locale-specific information, such as the locale's name
 * and associated arguments or values used for localization.
 */
pub struct LocaleData {
    pub name: String,
    pub args: HashMap<String, LocaleValue>,
}

impl LocaleData {
    /// Creates a new instance of the struct with the provided name.
    ///
    /// # Parameters
    /// - `name`: A string slice that represents the name to associate with the instance.
    ///
    /// # Returns
    /// A new instance of the struct, where the `name` field is initialized
    /// with the provided value and the `args` field is set to its default value.
    ///
    /// # Example
    /// ```
    /// use cjtoolkit_structured_validator::common::locale::LocaleData;
    /// let instance = LocaleData::new("example");
    /// ```
    pub fn new(name: &str) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
            args: Default::default(),
        })
    }

    /// Creates a new instance of the structure using a name and a vector of key-value pairs.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice representing the name for the new instance.
    /// * `args` - A vector containing tuples, where each tuple consists of a `String` (key)
    ///   and a `LocaleValue` (value). These key-value pairs will be used to initialize
    ///   the args map of the structure.
    ///
    /// # Returns
    ///
    /// A new instance of the structure is populated with the provided `name` and `args`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::common::locale::LocaleData;
    /// use cjtoolkit_structured_validator::common::locale::LocaleValue;
    /// let instance = LocaleData::new_with_vec("example_name", vec![
    ///     ("key1".to_string(), LocaleValue::from("value1")),
    ///     ("key2".to_string(), LocaleValue::from("value2")),
    /// ]);
    /// assert_eq!(instance.name, "example_name");
    /// assert!(instance.args.contains_key("key1"));
    /// ```
    pub fn new_with_vec(name: &str, args: Vec<(String, LocaleValue)>) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
            args: args.into_iter().collect(),
        })
    }
}

/// A trait representing a localized message provider that offers locale-specific data.
///
/// Types that implement this trait are expected to provide a mechanism for retrieving
/// locale-specific data useful for internationalization or localization purposes. Implementers must
/// be `Send` and `Sync` to ensure safe usage in concurrent environments.
///
/// # Required Methods
///
/// - `get_locale_data`: Retrieves locale-specific information encapsulated in a `LocaleData` object.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use cjtoolkit_structured_validator::common::locale::{LocaleMessage, LocaleData};
///
/// struct MyLocaleMessage;
///
/// impl LocaleMessage for MyLocaleMessage {
///     fn get_locale_data(&self) -> Arc<LocaleData> {
///         LocaleData::new("validate-example")
///     }
/// }
///
/// let locale_message = MyLocaleMessage;
/// let locale_data = locale_message.get_locale_data();
/// ```
///
/// # Notes
///
/// This trait is designed to be used in scenarios where thread-safe and cross-thread access
/// to locale information is necessary.
pub trait LocaleMessage: Send + Sync {
    fn get_locale_data(&self) -> Arc<LocaleData>;
}

impl LocaleMessage for Arc<LocaleData> {
    fn get_locale_data(&self) -> Arc<LocaleData> {
        Arc::clone(self)
    }
}
