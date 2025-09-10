# Structured Validator Library for Rust

A library for validating structured data.

[![crates.io](https://img.shields.io/crates/v/cjtoolkit-structured-validator.svg)](https://crates.io/crates/cjtoolkit-structured-validator)
[![docs.rs](https://img.shields.io/docsrs/cjtoolkit-structured-validator)](https://docs.rs/cjtoolkit-structured-validator)

# Description

A Validation library designed to be extendable with the use
of [Rust's extension trait](http://xion.io/post/code/rust-extension-traits.html).

Here are some examples of what you can do with this library with Trait Extensions:

* [Integration with Fluent Translation](https://github.com/cjtoolkit/cjtoolkit-structured-validator/blob/main/examples/fluent-integration/src/main.rs)
* [Multiple Choice Validation](https://github.com/cjtoolkit/cjtoolkit-structured-validator/blob/main/examples/multiple-choice/src/main.rs)
* [Validate with RegExp](https://github.com/cjtoolkit/cjtoolkit-structured-validator/blob/main/examples/validate-with-regexp/src/main.rs)

The trait extensions will let you create your own validation rules and integrate them with the library; the
possibilities are endless. The examples also demonstrate the fact that it stays out of the way when it comes to writing
testable code.

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

validate-date-min = Must be after { $min }
validate-date-time-min = Must be after { DATETIME($min) }
validate-date-time-naive-min = Must be after { $min }
validate-time-min = Must be after { $min } }

validate-date-max = Must be before { $max }
validate-date-time-max = Must be before { DATETIME($max) }
validate-date-time-naive-max = Must be before { $max }
validate-time-max = Must be before { $max } }
```

Note: this should contain all the validation messages you want to use.