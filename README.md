# Structured Validator Library for Rust

A library for validating structured data.

# Description

A Validation library designed to be extendable with the use
of [Rust's extension trait](http://xion.io/post/code/rust-extension-traits.html),
for example, you can bring your own i18n by implementing extension trait against `LocaleData` and `ValidateErrorStore`

Here are the examples of where Poem's i18n and Dioxus's i18n are being used.

https://github.com/CJ-Jackson/animal_api_again/blob/main/backend_api/src/common/locale/mod.rs  
https://github.com/CJ-Jackson/animal_api_again/blob/main/ui/src/common/locale/mod.rs

You can also extend `Name` and hook in your own RegExp of your choosing, or you could use your
own Post Code Validator; the possibility is endless.

# Installation

```sh
cargo add cjtoolkit-structured-validator --git https://github.com/cjtoolkit/cjtoolkit-structured-validator
```

# Locale Template

Fluent Template Language (FTL)

```ftl
# Validation
validate-cannot-be-empty = Cannot be empty

validate-min-length =
    Must be at least { $min ->
        [one] a character
        *[other] { $min } characters
    }
validate-max-length =
    Must be at most { $max ->
        [one] a character
        *[other] { $max } characters
    }

validate-must-have-special-chars = Must contain at least one special character
validate-must-have-uppercase-and-lowercase = Must contain at least one uppercase and lowercase letter
validate-must-have-uppercase = Must contain at least one uppercase letter
validate-must-have-lowercase = Must contain at least one lowercase letter
validate-must-have-digit = Must contain at least one digit

validate-password-does-not-match = Does not match
validate-username-taken = Already taken

validate-invalid-url = URL is not valid

validate-email-invalid = Email is not valid
validate-email-does-not-match = Email does not match

validate-number-min-value = Must be at least { $min }
validate-number-max-value = Must be at most { $max }
```

# Features Flags

- `full` - Enables all features.
- `url` - Enables URL support.
- `email` - Enables Email support.