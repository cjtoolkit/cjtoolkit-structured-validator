//! This module contains structures and traits for working with validation errors.

use crate::common::locale::LocaleMessage;
use blake3::Hash;
use std::fmt::Debug;
use std::sync::Arc;

/// `ValidateErrorStore` is a structure used to store validation errors, where each error consists
/// of a `String` key and an associated `Box<dyn LocaleMessage>` value. The key represents
/// an identifier (e.g., field name or error code), while the `LocaleMessage` represents
/// a localizable message for the associated validation error.
///
/// This structure is designed to be `Default` and makes use of an `Arc<[]>` to share ownership
/// of the data, enabling efficient cloning and concurrent usage in multithreaded contexts.
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

    /// Converts the current instance into a `ValidateErrorCollector`.
    ///
    /// This method takes the current object, clones it, and converts it into a
    /// `ValidateErrorCollector` instance. It is useful when you want to transform
    /// the current object to a `ValidateErrorCollector` for further processing or
    /// validation error handling.
    ///
    /// # Returns
    ///
    /// A `ValidateErrorCollector` created by cloning and converting the current object.
    pub fn as_validate_error_collector(&self) -> ValidateErrorCollector {
        self.clone().into()
    }

    fn hash(&self) -> Hash {
        let mut hasher = blake3::Hasher::new();
        for error in self.0.iter() {
            hasher.update(error.0.as_bytes());
        }
        hasher.finalize()
    }
}

impl Into<ValidateErrorCollector> for ValidateErrorStore {
    fn into(self) -> ValidateErrorCollector {
        let mut errors: Vec<(String, Box<dyn LocaleMessage>)> = vec![];
        for error in self.0.iter() {
            errors.push((error.0.clone(), Box::new(error.1.get_locale_data())));
        }
        ValidateErrorCollector(errors)
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
#[derive(Default)]
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
