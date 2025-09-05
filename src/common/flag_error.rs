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
