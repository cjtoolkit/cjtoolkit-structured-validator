pub mod description;
#[cfg(feature = "email")]
pub mod email;
pub mod name;
pub mod numbers;
pub mod password;
#[cfg(feature = "chrono")]
pub mod times_chrono;
#[cfg(feature = "humantime")]
pub mod times_humantime;
#[cfg(feature = "url")]
pub mod url;
pub mod username;

pub trait AsStringOnResult {
    fn as_string(&self) -> String;
}

impl<T, E> AsStringOnResult for Result<T, E>
where
    for<'a> &'a T: Into<String>,
{
    fn as_string(&self) -> String {
        self.as_ref().ok().map(|s| s.into()).unwrap_or_default()
    }
}
