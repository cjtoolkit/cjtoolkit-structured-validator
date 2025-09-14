use cjtoolkit_structured_validator::common::locale::{LocaleData, LocaleValue, ValidateErrorStore};
use cjtoolkit_structured_validator::common::validation_collector::AsValidateErrorStore;
use cjtoolkit_structured_validator::types::name::Name;
use poem::error::I18NError;
use poem::http::header;
use poem::i18n::{I18NArgs, I18NResources, Locale};
use poem::test::TestClient;
use poem::{EndpointExt, Route, handler};
use std::sync::Arc;

fn build_resources() -> Result<I18NResources, I18NError> {
    let english = include_str!("_locale/english.ftl");
    let french = include_str!("_locale/french.ftl");

    I18NResources::builder()
        .add_ftl("en-GB", english)
        .add_ftl("en-US", english)
        .add_ftl("fr-FR", french)
        .build()
}

pub trait LocaleExtForData {
    fn get_translation(&self, locale: &Locale, original: String) -> String;
}

impl LocaleExtForData for LocaleData {
    fn get_translation(&self, locale: &Locale, original: String) -> String {
        if !self.args.is_empty() {
            let mut values = I18NArgs::default();
            for (key, value) in self.args.iter() {
                match value {
                    LocaleValue::String(string) => {
                        values = values.set::<String, String>(key.clone(), string.clone());
                    }
                    LocaleValue::Uint(unit) => {
                        values = values.set::<String, usize>(key.clone(), *unit);
                    }
                    LocaleValue::Int(int) => {
                        values = values.set::<String, isize>(key.clone(), *int);
                    }
                    LocaleValue::Float(float) => {
                        values = values.set::<String, f64>(key.clone(), *float);
                    }
                }
            }
            locale
                .text_with_args(self.name.clone(), values)
                .unwrap_or(original)
        } else {
            locale.text(self.name.clone()).unwrap_or(original)
        }
    }
}

pub trait LocaleExtForStore {
    fn as_translated_messages(&self, locale: &Locale) -> Vec<String>;

    fn as_translated_messages_arc(&self, locale: &Locale) -> Arc<[String]> {
        self.as_translated_messages(locale).into()
    }
}

impl LocaleExtForStore for ValidateErrorStore {
    fn as_translated_messages(&self, locale: &Locale) -> Vec<String> {
        self.0
            .iter()
            .map(|e| e.1.get_locale_data().get_translation(locale, e.0.clone()))
            .collect()
    }
}

pub trait LocaleExtForResult: AsValidateErrorStore {
    fn as_translated_messages(&self, locale: &Locale) -> Vec<String>;

    fn as_translated_messages_arc(&self, locale: &Locale) -> Arc<[String]>;
}

impl<T, E> LocaleExtForResult for Result<T, E>
where
    for<'a> &'a E: Into<ValidateErrorStore>,
{
    fn as_translated_messages(&self, locale: &Locale) -> Vec<String> {
        self.as_validate_store().as_translated_messages(locale)
    }

    fn as_translated_messages_arc(&self, locale: &Locale) -> Arc<[String]> {
        self.as_validate_store().as_translated_messages_arc(locale)
    }
}

#[handler]
async fn index(locale: Locale) -> String {
    let value_result = Name::parse(Some("A"));

    let value_messages = value_result.as_translated_messages_arc(&locale);
    let mut str = String::new();
    for message in value_messages.iter() {
        str.push_str(message);
        str.push('\n');
    }
    str
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Route::new().at("/", index).data(build_resources()?);
    let cli = TestClient::new(app);

    let resp = cli
        .get("/")
        .header(header::ACCEPT_LANGUAGE, "en-GB")
        .send()
        .await;

    println!(
        "English Response Body: {}",
        resp.0.into_body().into_string().await.unwrap_or_default()
    );

    let resp = cli
        .get("/")
        .header(header::ACCEPT_LANGUAGE, "fr-FR")
        .send()
        .await;

    println!(
        "French Response Body: {}",
        resp.0.into_body().into_string().await.unwrap_or_default()
    );

    Ok(())
}
