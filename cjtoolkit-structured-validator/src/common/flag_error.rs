//! This module contains a function for flagging an error in a `Result` type.
//!
//! # Use Case
//! - Propagating errors in a `Result` type.
//! - Tracking whether an error occurred in a `Result` type.

/// Flags an error while propagating the result of a `Result` type.
///
/// This function takes a mutable boolean reference (`flag`) and a `Result` value.
/// If the `Result` is an error (`Err`), it sets the `flag` to `true`, allowing the
/// caller to track if an error occurred without consuming the `Result`.
///
/// The original `Result` is then returned unchanged.
///
/// # Type Parameters
/// - `T`: The type contained in the `Ok` variant of the `Result`.
/// - `E`: The type contained in the `Err` variant of the `Result`.
///
/// # Arguments
/// - `flag`: A mutable reference to a boolean flag that will be set to `true`
///   if the `Result` contains an error.
/// - `result`: The `Result` to check for an error.
///
/// # Returns
/// - Returns the provided `Result` value for further use or propagation.
///
/// # Examples
/// ```rust
/// use cjtoolkit_structured_validator::common::flag_error::flag_error;
/// let mut error_occurred = false;
/// let result: Result<i32, &str> = Err("An error occurred");
///
/// let checked_result = flag_error(&mut error_occurred, result);
///
/// assert!(error_occurred); // Flag has been set to true.
/// assert!(checked_result.is_err()); // The result is returned unchanged.
/// ```
///
/// ```rust
/// use cjtoolkit_structured_validator::common::flag_error::flag_error;
/// let mut error_occurred = false;
/// let result: Result<i32, &str> = Ok(42);
///
/// let checked_result = flag_error(&mut error_occurred, result);
///
/// assert!(!error_occurred); // Flag remains false.
/// assert_eq!(checked_result, Ok(42)); // The result is returned unchanged.
/// ```
pub fn flag_error<T, E>(flag: &mut bool, result: Result<T, E>) -> Result<T, E> {
    if result.is_err() {
        *flag = true;
    }
    result
}

/// A structure that provides a counter for managing flags or similar use cases.
///
/// The `FlagCounter` struct is designed to keep track of an incrementable count,
/// typically to represent or manage a counter for specific operations or states.
///
/// # Fields
///
/// * `count`:
///   - The current value of the counter.
///   - Represented as a `usize` (unsigned integer).
pub struct FlagCounter {
    count: usize,
}

impl FlagCounter {
    /// Creates and returns a new instance of the struct with the `count` field initialized to 0.
    ///
    /// # Returns
    ///
    /// A new instance of the struct.
    pub fn new() -> Self {
        Self { count: 0 }
    }

    /// Checks the provided `Result`, increments an internal error count if it is `Err`, and returns the `Result` unchanged.
    ///
    /// # Type Parameters
    /// - `T`: The type of the value inside the `Ok` variant of the `Result`.
    /// - `E`: The type of the error inside the `Err` variant of the `Result`.
    ///
    /// # Arguments
    /// - `result`: A `Result` value to be checked for an error.
    ///
    /// # Behavior
    /// - If the provided `Result` is `Err`, the method increments the internal error count (`self.count`).
    /// - Regardless of whether it is `Ok` or `Err`, the original `Result` is returned unchanged.
    ///
    /// # Returns
    /// Returns the provided `Result` value as-is.
    pub fn check<T, E>(&mut self, result: Result<T, E>) -> Result<T, E> {
        if result.is_err() {
            self.count += 1;
        }
        result
    }

    /// Checks if the current object is flagged.
    ///
    /// # Returns
    /// * `true` - If the `count` property of the object is greater than 0.
    /// * `false` - If the `count` property of the object is 0 or less.
    ///
    /// # Usage
    /// This function acts as a flagging mechanism. For example,
    /// it can be used to determine if there are any active or positive counts that
    /// signify certain conditions.
    ///
    /// # Note
    /// The `count` field must be initialized appropriately before calling this function.
    pub fn is_flagged(&self) -> bool {
        self.count > 0
    }

    /// Returns the current value of the `count` field.
    ///
    /// # Returns
    /// * `usize` - The value of the `count` field.
    pub fn get_count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_error() {
        let mut flag = false;
        let result: Result<(), ()> = flag_error(&mut flag, Ok(()));
        assert!(result.is_ok());
        assert!(!flag);
    }

    #[test]
    fn test_flag_error_err() {
        let mut flag = false;
        let result: Result<(), ()> = flag_error(&mut flag, Err(()));
        assert!(result.is_err());
        assert!(flag);
    }
}
