use unicode_segmentation::UnicodeSegmentation;

pub struct StringValidator<'a>(&'a str, usize);

impl<'a> StringValidator<'a> {
    const SPECIAL_CHARS: [char; 30] = [
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}',
        '\\', '|', ';', ':', '\'', '"', ',', '.', '<', '>', '/', '?',
    ];

    fn new(s: &'a str) -> Self {
        Self(s, s.graphemes(true).count())
    }

    pub fn count_graphemes(&self) -> usize {
        self.1
    }

    pub fn is_empty(&self) -> bool {
        self.1 == 0
    }

    pub fn has_special_chars(&self) -> bool {
        self.0.chars().any(|c| Self::SPECIAL_CHARS.contains(&c))
    }

    pub fn count_special_chars(&self) -> usize {
        self.0
            .chars()
            .filter(|c| Self::SPECIAL_CHARS.contains(c))
            .count()
    }

    pub fn has_ascii_uppercase(&self) -> bool {
        self.0.chars().any(|c| c.is_ascii_uppercase())
    }

    pub fn count_ascii_uppercase(&self) -> usize {
        self.0.chars().filter(|c| c.is_ascii_uppercase()).count()
    }

    pub fn has_ascii_lowercase(&self) -> bool {
        self.0.chars().any(|c| c.is_ascii_lowercase())
    }

    pub fn count_ascii_lowercase(&self) -> usize {
        self.0.chars().filter(|c| c.is_ascii_lowercase()).count()
    }

    pub fn has_ascii_uppercase_and_lowercase(&self) -> bool {
        self.has_ascii_uppercase() && self.has_ascii_lowercase()
    }
    pub fn count_ascii_uppercase_and_lowercase(&self) -> usize {
        self.count_ascii_uppercase() + self.count_ascii_lowercase()
    }

    pub fn has_ascii_digit(&self) -> bool {
        self.0.chars().any(|c| c.is_ascii_digit())
    }

    pub fn count_ascii_digit(&self) -> usize {
        self.0.chars().filter(|c| c.is_ascii_digit()).count()
    }

    pub fn has_ascii_alphanumeric(&self) -> bool {
        self.0.chars().any(|c| c.is_ascii_alphanumeric())
    }

    pub fn count_ascii_alphanumeric(&self) -> usize {
        self.0.chars().filter(|c| c.is_ascii_alphanumeric()).count()
    }
}

trait StrSealed {}

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
