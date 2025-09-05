use unicode_segmentation::UnicodeSegmentation;

/// A structure for validating strings with specific constraints.
pub struct StringValidator<'a>(&'a str, usize);

impl<'a> StringValidator<'a> {
    /// A constant array that contains a predefined set of 30 special characters.
    ///
    /// These special characters are commonly used in programming, input validation,
    /// and applications where character-based operations are required, such as parsing,
    /// formatting, or password generation.
    ///
    /// # Characters included:
    /// - `!`, `@`, `#`, `$`, `%`, `^`, `&`, `*`, `(`, `)`, `-`, `_`, `=`, `+`
    /// - `[`, `]`, `{`, `}`, `\`, `|`, `;`, `:`, `'`, `"`
    /// - `,`, `.`, `<`, `>`, `/`, `?`
    ///
    /// # Notes
    /// This constant is immutable and should be used for read-only purposes.
    pub const SPECIAL_CHARS: [char; 30] = [
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}',
        '\\', '|', ';', ':', '\'', '"', ',', '.', '<', '>', '/', '?',
    ];

    fn new(s: &'a str) -> Self {
        Self(s, s.graphemes(true).count())
    }

    /// Returns the number of graphemes in the structure.
    ///
    /// This function retrieves the count of graphemes stored within the current instance.
    /// It assumes that the second element of the tuple (`self.1`) represents the grapheme count.
    ///
    /// # Returns
    ///
    /// * `usize` - The total number of graphemes.
    ///
    pub fn count_graphemes(&self) -> usize {
        self.1
    }

    /// Checks whether the current object is empty.
    ///
    /// # Returns
    /// - `true` if the internal value (`self.1`) is equal to 0, indicating that the object is empty.
    /// - `false` otherwise.
    ///
    pub fn is_empty(&self) -> bool {
        self.1 == 0
    }

    /// Checks if the string contains any special character from a predefined set.
    ///
    /// # Returns
    /// * `true` - If the string contains at least one special character.
    /// * `false` - If the string does not contain any special characters.
    ///
    /// # Implementation Details
    /// - Checks each character in the string to determine if it is part of the `SPECIAL_CHARS` set.
    ///
    pub fn has_special_chars(&self) -> bool {
        self.0.chars().any(|c| Self::SPECIAL_CHARS.contains(&c))
    }

    /// Counts the number of special characters in the string.
    ///
    /// This function iterates through the characters of the string and determines
    /// how many of them are considered special. A special character is defined as
    /// any character present in the `SPECIAL_CHARS` set.
    ///
    /// # Returns
    /// * `usize` - The number of special characters in the string.
    ///
    /// # Note
    /// * The `SPECIAL_CHARS` set is a pre-defined constant within the implementation
    ///   of this type, specifying what qualifies as a special character.
    pub fn count_special_chars(&self) -> usize {
        self.0
            .chars()
            .filter(|c| Self::SPECIAL_CHARS.contains(c))
            .count()
    }

    /// Checks if the string contains at least one ASCII uppercase character.
    ///
    /// # Returns
    ///
    /// * `true` - If the string contains at least one ASCII uppercase character.
    /// * `false` - If the string does not contain any ASCII uppercase characters.
    ///
    /// # Note
    ///
    /// This function iterates over the characters of the string
    /// and checks if any character is an ASCII uppercase letter
    /// (i.e., 'A' to 'Z').
    pub fn has_ascii_uppercase(&self) -> bool {
        self.0.chars().any(|c| c.is_ascii_uppercase())
    }

    /// Counts and returns the number of ASCII uppercase letters in the string.
    ///
    /// # Description
    /// This function iterates over the characters of the string, filters out
    /// those that are ASCII uppercase letters, and returns the total count.
    ///
    /// # Returns
    /// * `usize` - The number of ASCII uppercase letters in the string.
    ///
    pub fn count_ascii_uppercase(&self) -> usize {
        self.0.chars().filter(|c| c.is_ascii_uppercase()).count()
    }

    /// Checks if the string slice contains any ASCII lowercase characters.
    ///
    /// # Returns
    /// - `true` if the string contains at least one ASCII lowercase character (`'a'..='z'`).
    /// - `false` if no ASCII lowercase characters are found.
    ///
    /// This method iterates through all the characters of the inner string (`self.0`)
    /// and checks if any character satisfies the `is_ascii_lowercase` predicate.
    /// It returns as soon as a lowercase ASCII character is found.
    pub fn has_ascii_lowercase(&self) -> bool {
        self.0.chars().any(|c| c.is_ascii_lowercase())
    }

    /// Counts the number of ASCII lowercase characters in the string.
    ///
    /// This function iterates through each character of the string stored in the first field of the
    /// tuple (`self.0`), filters out only those that are lowercase ASCII alphabetic characters (a-z),
    /// and returns their count.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of ASCII lowercase characters in the string.
    ///
    /// Note: This function only checks for ASCII lowercase characters. Non-ASCII characters are ignored.
    pub fn count_ascii_lowercase(&self) -> usize {
        self.0.chars().filter(|c| c.is_ascii_lowercase()).count()
    }

    /// Checks if the current object contains both ASCII uppercase and lowercase characters.
    ///
    /// This method evaluates whether the instance has at least one ASCII uppercase letter
    /// (e.g., 'A'-'Z') and at least one ASCII lowercase letter (e.g., 'a'-'z') simultaneously.
    ///
    /// # Returns
    /// * `true` - If the instance contains both uppercase and lowercase ASCII characters.
    /// * `false` - If the instance does not contain both uppercase and lowercase ASCII characters.
    ///
    pub fn has_ascii_uppercase_and_lowercase(&self) -> bool {
        self.has_ascii_uppercase() && self.has_ascii_lowercase()
    }

    /// Counts the total number of ASCII uppercase and lowercase characters in the current instance.
    ///
    /// This method combines the results of `count_ascii_uppercase` and `count_ascii_lowercase`,
    /// returning their sum. It provides a convenient way to calculate the total count of ASCII alphabetic
    /// characters (both uppercase and lowercase).
    ///
    /// # Returns
    ///
    /// * `usize` - The sum of the ASCII uppercase and lowercase character counts.
    ///
    /// Note: The functionality of this method depends on the implementation of both
    /// `count_ascii_uppercase` and `count_ascii_lowercase` methods, which must be defined
    /// elsewhere in the same context.
    pub fn count_ascii_uppercase_and_lowercase(&self) -> usize {
        self.count_ascii_uppercase() + self.count_ascii_lowercase()
    }

    /// Checks whether the inner string contains at least one ASCII digit.
    ///
    /// # Returns
    /// - `true` if the string contains at least one ASCII digit (`0-9`).
    /// - `false` otherwise.
    ///
    /// This method works by iterating over the characters of the inner string
    /// and checking if any character is an ASCII digit.
    pub fn has_ascii_digit(&self) -> bool {
        self.0.chars().any(|c| c.is_ascii_digit())
    }

    /// Counts the number of ASCII digit characters in the string.
    ///
    /// This function iterates through the characters of the string, filters out
    /// the characters that are ASCII digits (`0-9`), and returns the count of those digits.
    ///
    /// # Returns
    ///
    /// * `usize` - The total number of ASCII digit characters in the string.
    ///
    /// Note: Non-ASCII digits (e.g., Arabic numerals) will not be counted.
    pub fn count_ascii_digit(&self) -> usize {
        self.0.chars().filter(|c| c.is_ascii_digit()).count()
    }

    /// Checks if the string contains any ASCII alphanumeric characters.
    ///
    /// This method checks if at least one character in the string is an ASCII alphanumeric character.
    /// ASCII alphanumeric characters are defined as 'a' to 'z', 'A' to 'Z', and '0' to '9'.
    ///
    /// # Returns
    ///
    /// * `true` - If the string contains at least one ASCII alphanumeric character.
    /// * `false` - If the string does not contain any ASCII alphanumeric characters.
    ///
    pub fn has_ascii_alphanumeric(&self) -> bool {
        self.0.chars().any(|c| c.is_ascii_alphanumeric())
    }

    /// Counts the number of ASCII alphanumeric characters in the string.
    ///
    /// This function iterates through the characters of the string, filters out
    /// those that are ASCII alphanumeric (letters and digits), and returns the count
    /// of such characters.
    ///
    /// # Returns
    /// * `usize` - The number of ASCII alphanumeric characters in the string.
    ///
    /// Note: This function will only count characters that are both ASCII and alphanumeric.
    /// Non-ASCII characters or symbols will not be included in the count.
    pub fn count_ascii_alphanumeric(&self) -> usize {
        self.0.chars().filter(|c| c.is_ascii_alphanumeric()).count()
    }
}

trait StrSealed {}

/// A trait providing an extension for string validation functionality. This trait is sealed and
/// cannot be implemented outside of the module where it is defined. It is designed to extend
/// the functionality of types implementing the private `StrSealed` trait.
///
/// # Required Methods
/// - `as_string_validator`: Converts the type implementing this trait into a
///   `StringValidator`, enabling validation operations on the string.
///
/// # Note
/// The `#[allow(private_bounds)]` attribute is used as this trait relies on the
/// private `StrSealed` trait as a bound to enforce its sealing.
///
/// # Associated Types
/// - This trait does not introduce any associated types.
///
/// # Methods
///
/// - `as_string_validator(&'_ self) -> StringValidator<'_>`
///   - Returns a `StringValidator` which allows performing validation operations
///     on the implementing type.
///   - This method borrows self for a lifetime (`'_`) and produces a
///     `StringValidator` tied to the lifetime of the reference.
///
#[allow(private_bounds)]
pub trait StrValidationExtension: StrSealed {
    fn as_string_validator(&'_ self) -> StringValidator<'_>;
}

impl StrSealed for &str {}

impl StrValidationExtension for &str {
    fn as_string_validator(&'_ self) -> StringValidator<'_> {
        StringValidator::new(self)
    }
}

impl StrSealed for String {}

impl StrValidationExtension for String {
    fn as_string_validator(&'_ self) -> StringValidator<'_> {
        StringValidator::new(self)
    }
}
