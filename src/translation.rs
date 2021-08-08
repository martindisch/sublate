use eyre::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub fn translate(
    texts: &[String],
    source_language: &str,
    target_language: &str,
    client: &Client,
) -> Result<Vec<String>> {
    let request = TranslationRequest {
        q: texts,
        format: "html",
        source: source_language,
        target: target_language,
    };

    let response: TranslationResponse = client
        .post("https://translation.googleapis.com/language/translate/v2")
        .json(&request)
        .send()?
        .json()?;

    Ok(response
        .data
        .translations
        .into_iter()
        .map(|t| {
            html_escape::decode_html_entities(&t.translated_text).to_string()
        })
        .collect())
}

#[derive(Debug, Serialize)]
struct TranslationRequest<'a, T>
where
    T: AsRef<str>,
{
    q: &'a [T],
    format: &'a str,
    source: &'a str,
    target: &'a str,
}

#[derive(Debug, Deserialize)]
struct TranslationResponse {
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    translations: Vec<Translation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Translation {
    translated_text: String,
}
