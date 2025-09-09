//! A library for validating structured data.
//!
//! # Description
//!
//! A Validation library designed to be extendable with the use of Rust's extension trait, for example,
//! you can bring your own i18n by implementing extension trait against `LocaleData` and `ValidateErrorStore`
//!
//! Here are the examples of where Poem's i18n and Dioxus's i18n are being used.
//!
//! <https://github.com/CJ-Jackson/animal_api_again/blob/main/backend_api/src/common/locale/mod.rs>  
//! <https://github.com/CJ-Jackson/animal_api_again/blob/main/ui/src/common/locale/mod.rs>
//!
//! You can also extend `Name` and hook in your own RegExp of your choosing, or you could use your
//! own Post Code Validator; the possibility is endless.

#![warn(clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod base;
pub mod common;
pub mod types;
