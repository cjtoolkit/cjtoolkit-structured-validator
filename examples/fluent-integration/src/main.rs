use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::common::locale::{LocaleData, LocaleValue, ValidateErrorStore};
use cjtoolkit_structured_validator::types::description::{Description, DescriptionError};
use cjtoolkit_structured_validator::types::name::name_alias::{Title, TitleError};
use fluent::{FluentArgs, FluentBundle, FluentResource};
use std::borrow::Borrow;
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

trait FluentBundleForLocaleData {
    fn get_translation<R: Borrow<FluentResource>>(
        &self,
        bundle: &FluentBundle<R>,
        original: String,
    ) -> String;
}

impl FluentBundleForLocaleData for LocaleData {
    fn get_translation<R: Borrow<FluentResource>>(
        &self,
        bundle: &FluentBundle<R>,
        original: String,
    ) -> String {
        let mut args: Option<FluentArgs> = None;
        if !self.args.is_empty() {
            let mut values = FluentArgs::new();
            for (key, value) in self.args.iter() {
                match value {
                    LocaleValue::String(string) => {
                        values.set::<String, String>(key.clone(), string.clone());
                    }
                    LocaleValue::Uint(uint) => {
                        values.set::<String, usize>(key.clone(), *uint);
                    }
                    LocaleValue::Int(int) => {
                        values.set::<String, isize>(key.clone(), *int);
                    }
                    LocaleValue::Float(float) => {
                        values.set::<String, f64>(key.clone(), *float);
                    }
                }
            }
            args = Some(values);
        }

        let mut errors = vec![];
        bundle
            .get_message(self.name.as_str())
            .map(|f| {
                let pattern = f.value();
                match pattern {
                    None => original.clone(),
                    Some(pattern) => {
                        let value = bundle.format_pattern(pattern, args.as_ref(), &mut errors);
                        value.to_string()
                    }
                }
            })
            .unwrap_or(original)
    }
}

pub trait FluentBundleForStore {
    fn as_translated_messages<R: Borrow<FluentResource>>(
        &self,
        bundle: &FluentBundle<R>,
    ) -> Vec<String>;

    fn as_translated_messages_arc<R: Borrow<FluentResource>>(
        &self,
        bundle: &FluentBundle<R>,
    ) -> Arc<[String]> {
        self.as_translated_messages(bundle).into()
    }
}

impl FluentBundleForStore for ValidateErrorStore {
    fn as_translated_messages<R: Borrow<FluentResource>>(
        &self,
        bundle: &FluentBundle<R>,
    ) -> Vec<String> {
        self.0
            .iter()
            .map(|e| e.1.get_locale_data().get_translation(bundle, e.0.clone()))
            .collect()
    }
}

fn build_english_bundle() -> FluentBundle<FluentResource> {
    let ftl_string = String::from(include_str!("_locale/english.ftl"));

    let resource = FluentResource::try_new(ftl_string).expect("Failed to build resource");
    let lang_id: LanguageIdentifier = "en-US".parse().expect("Failed to parse lang id");
    let mut bundle = FluentBundle::new(vec![lang_id]);
    bundle
        .add_resource(resource)
        .expect("Failed to add resource");

    bundle
}

fn build_french_bundle() -> FluentBundle<FluentResource> {
    let ftl_string = String::from(include_str!("_locale/french.ftl"));

    let resource = FluentResource::try_new(ftl_string).expect("Failed to build resource");
    let lang_id: LanguageIdentifier = "fr-FR".parse().expect("Failed to parse lang id");
    let mut bundle = FluentBundle::new(vec![lang_id]);
    bundle
        .add_resource(resource)
        .expect("Failed to add resource");
    bundle
}

struct Subject {
    title: String,
    description: String,
}

impl Subject {
    fn as_validated(&self) -> Result<SubjectValidated, SubjectError> {
        let mut flag = FlagCounter::new();

        let title = flag.check(Title::parse(Some(self.title.clone().as_str())));
        let description = flag.check(Description::parse(Some(self.description.clone().as_str())));

        if flag.is_flagged() {
            return Err(SubjectError { title, description });
        }
        Ok(SubjectValidated {
            title: title.expect("Expected title to be valid"),
            description: description.expect("Expected description to be valid"),
        })
    }
}

#[allow(dead_code)]
struct SubjectValidated {
    title: Title,
    description: Description,
}

struct SubjectError {
    title: Result<Title, TitleError>,
    description: Result<Description, DescriptionError>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct SubjectMessage {
    title: Arc<[String]>,
    description: Arc<[String]>,
}

impl<R: Borrow<FluentResource>> From<(&SubjectError, &FluentBundle<R>)> for SubjectMessage {
    fn from((error, bundle): (&SubjectError, &FluentBundle<R>)) -> Self {
        Self {
            title: error
                .title
                .as_ref()
                .err()
                .map(|e| e.0.as_translated_messages_arc(bundle))
                .unwrap_or_default(),
            description: error
                .description
                .as_ref()
                .err()
                .map(|e| e.0.as_translated_messages_arc(bundle))
                .unwrap_or_default(),
        }
    }
}

fn main() {
    let subject = Subject {
        title: "AA".to_string(),
        description: "".to_string(),
    };

    let subject_error = subject.as_validated().err().expect("Expected error");

    let english_bundle = build_english_bundle();
    let french_bundle = build_french_bundle();

    let english_message = SubjectMessage::from((&subject_error, &english_bundle));
    let french_message = SubjectMessage::from((&subject_error, &french_bundle));

    println!("{:?}", english_message);
    println!("{:?}", french_message);
}
