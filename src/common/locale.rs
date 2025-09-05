use blake3::Hash;
use std::collections::HashMap;
use std::fmt::Debug;
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
 * and associated arguments or values used for localization purposes.
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
    /// Returns a new instance of the struct, where the `name` field is initialized
    /// with the provided value and the `args` field is set to its default value.
    ///
    /// # Example
    /// ```
    /// use cjtoolkit_structured_validator::common::locale::LocaleData;
    /// let instance = LocaleData::new("example");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            args: Default::default(),
        }
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
    /// A new instance of the structure populated with the provided `name` and `args`.
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
    pub fn new_with_vec(name: &str, args: Vec<(String, LocaleValue)>) -> Self {
        Self {
            name: name.to_string(),
            args: args.into_iter().collect(),
        }
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
/// use cjtoolkit_structured_validator::common::locale::{LocaleMessage, LocaleData};
///
/// struct MyLocaleMessage;
///
/// impl LocaleMessage for MyLocaleMessage {
///     fn get_locale_data(&self) -> LocaleData {
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
    fn get_locale_data(&self) -> LocaleData;
}

/// `ValidateErrorStore` is a structure used to store validation errors, where each error consists
/// of a `String` key and an associated `Box<dyn LocaleMessage>` value. The key represents
/// an identifier (e.g., field name or error code), while the `LocaleMessage` represents
/// a localizable message for the associated validation error.
///
/// This structure is designed to be `Default` and makes use of an `Arc<[]>` to share ownership
/// of the data, enabling efficient cloning and concurrent usage in multi-threaded contexts.
///
/// # Fields
/// - `0`: A reference-counted array (`Arc<[]>`) of tuples containing:
///   - `String`: The identifier for the validation error.
///   - `Box<dyn LocaleMessage>`: A boxed trait object to represent a localizable message dynamically.
///
/// # Traits
/// The struct derives the `Default` trait so it can be initialized with an empty error store.
///
#[derive(Default)]
pub struct ValidateErrorStore(pub Arc<[(String, Box<dyn LocaleMessage>)]>);

impl Debug for ValidateErrorStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, error) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", error.0)?;
        }
        Ok(())
    }
}

impl PartialEq for ValidateErrorStore {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

impl Clone for ValidateErrorStore {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl ValidateErrorStore {
    /// Converts the internal vector representation of the original message
    /// into an `Arc<[String]>`, a thread-safe, shared, and reference-counted slice of strings.
    ///
    /// # Returns
    /// An `Arc<[String]>` containing the original message as a shared reference. The
    /// `Arc` allows multiple threads to safely share ownership of the message data without
    /// requiring additional cloning or copying of the underlying strings.
    ///
    /// # Usage
    /// This method can be used when you need a thread-safe, immutable reference
    /// to the original message structure, allowing for efficient sharing of data
    /// across threads.
    ///
    /// # Implementation
    /// Internally, the method calls `as_original_message_vec()` to retrieve the original
    /// message as a `Vec<String>` and then converts it to an `Arc<[String]>`.
    ///
    pub fn as_original_message(&self) -> Arc<[String]> {
        self.as_original_message_vec().into()
    }

    /// Converts the current collection into a `Vec<String>` containing the original messages.
    ///
    /// # Description
    /// This method iterates over the elements of the internal collection, extracts the original
    /// message (assumed to be stored as the first element of each component, `e.0`), clones it,
    /// and collects all the cloned messages into a new `Vec<String>`.
    ///
    /// # Returns
    /// A `Vec<String>` containing the cloned original messages from each element in the collection.
    ///
    /// # Notes
    /// - The internal structure `self.0` must be iterable, and each element must have a
    ///   `0` field containing a `String`-like value.
    /// - The result is a completely new vector and does not modify the internal state of the
    ///   current collection.
    ///
    /// # Complexity
    /// This method has a time complexity of O(n), where n is the number of elements in
    /// the internal collection `self.0`.
    pub fn as_original_message_vec(&self) -> Vec<String> {
        self.0.iter().map(|e| e.0.clone()).collect()
    }

    fn hash(&self) -> Hash {
        let mut hasher = blake3::Hasher::new();
        for error in self.0.iter() {
            hasher.update(error.0.as_bytes());
        }
        hasher.finalize()
    }
}

/// A struct for collecting validation errors in a list.
///
/// `ValidateErrorCollector` is used to gather validation errors that can be
/// associated with a specific field or key. Each error is stored as a tuple containing:
/// - A `String` representing the field or key name where the error occurred.
/// - A `Box<dyn LocaleMessage>` representing a localized error message.
///
/// # Fields
/// - `0`: A vector of tuples, each tuple containing a field name as `String` and a
///   localized error message as `Box<dyn LocaleMessage>`.
///
/// Note: The `LocaleMessage` trait is used to encapsulate errors with localization support.
/// Implementations of `LocaleMessage` should provide mechanisms for translating error messages
/// to various locales.
pub struct ValidateErrorCollector(pub Vec<(String, Box<dyn LocaleMessage>)>);

impl Into<ValidateErrorStore> for ValidateErrorCollector {
    fn into(self) -> ValidateErrorStore {
        ValidateErrorStore(self.0.into())
    }
}

impl ValidateErrorCollector {
    /// Creates a new instance of the struct with an empty `Vec`.
    ///
    /// # Returns
    /// A new instance of the struct containing an empty `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cjtoolkit_structured_validator::common::locale::ValidateErrorCollector;
    /// let instance = ValidateErrorCollector::new();
    /// assert!(instance.0.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Checks whether the container is empty.
    ///
    /// This method returns `true` if the container has no elements, and `false` otherwise.
    ///
    /// # Returns
    /// * `true` - If the container is empty.
    /// * `false` - If the container contains one or more elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use cjtoolkit_structured_validator::common::locale::ValidateErrorCollector;
    /// let container = ValidateErrorCollector::new();
    /// assert!(container.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    /// Adds an error item to the collection.
    ///
    /// # Parameters
    /// - `error`: A tuple containing:
    ///   - A `String` representing the error message or identifier.
    ///   - A `Box<dyn LocaleMessage>` which encapsulates a trait object implementing `LocaleMessage`.
    ///     This provides localized details for the error.
    ///
    /// # Behavior
    /// Appends the given `error` tuple to the internal vector storing errors.
    ///
    pub fn push(&mut self, error: (String, Box<dyn LocaleMessage>)) {
        self.0.push(error);
    }

    /// Returns the number of elements in the collection.
    ///
    /// This method provides the length of the underlying collection by
    /// delegating the call to the `.len()` method of the inner data structure.
    ///
    /// # Returns
    /// * `usize` - The number of elements currently contained in the collection.
    ///
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
