use blake3::Hash;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

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

pub struct LocaleData {
    pub name: String,
    pub args: HashMap<String, LocaleValue>,
}

impl LocaleData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            args: Default::default(),
        }
    }

    pub fn new_with_vec(name: &str, args: Vec<(String, LocaleValue)>) -> Self {
        Self {
            name: name.to_string(),
            args: args.into_iter().collect(),
        }
    }
}

pub trait LocaleMessage: Send + Sync {
    fn get_locale_data(&self) -> LocaleData;
}

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
    fn hash(&self) -> Hash {
        let mut hasher = blake3::Hasher::new();
        for error in self.0.iter() {
            hasher.update(error.0.as_bytes());
        }
        hasher.finalize()
    }

    pub fn as_original_message(&self) -> Arc<[String]> {
        self.as_original_message_vec().into()
    }

    pub fn as_original_message_vec(&self) -> Vec<String> {
        self.0.iter().map(|e| e.0.clone()).collect()
    }
}

pub struct ValidateErrorCollector(pub Vec<(String, Box<dyn LocaleMessage>)>);

impl Into<ValidateErrorStore> for ValidateErrorCollector {
    fn into(self) -> ValidateErrorStore {
        ValidateErrorStore(self.0.into())
    }
}

impl ValidateErrorCollector {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, error: (String, Box<dyn LocaleMessage>)) {
        self.0.push(error);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
