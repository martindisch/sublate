use eyre::Result;
use regex::Regex;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::srt::{Subtitle, Subtitles};

pub fn translate(
    subtitles: &[Subtitle],
    source_language: &str,
    target_language: &str,
    client: &Client,
) -> Result<Subtitles> {
    let texts: Vec<_> = subtitles.iter().map(Subtitle::to_html).collect();
    let request = TranslationRequest {
        q: &texts,
        format: "html",
        source: source_language,
        target: target_language,
    };

    let response: TranslationResponse = client
        .post("https://translation.googleapis.com/language/translate/v2")
        .json(&request)
        .send()?
        .json()?;

    let translated_subs = response
        .data
        .translations
        .iter()
        .zip(subtitles)
        .map(|(translation, original)| {
            Subtitle::from_html(
                original.counter,
                original.timestamp.clone(),
                &html_escape::decode_html_entities(
                    &translation.translated_text,
                )
                .to_string(),
            )
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Subtitles(translated_subs))
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

impl Subtitle {
    fn to_html(&self) -> String {
        self.lines
            .iter()
            .map(|s| format!("<span>{}</span>", s))
            .collect()
    }

    fn from_html(counter: u32, timestamp: String, html: &str) -> Result<Self> {
        let regex = Regex::new(r"<span>(.+?)</span>")?;
        let lines: Vec<_> = regex
            .captures_iter(html)
            .map(|c| c[1].to_string())
            .collect();

        Ok(Self {
            counter,
            timestamp,
            lines,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_single_span() {
        let subtitle = Subtitle::from_html(
            1,
            "whatever".into(),
            "<span>We buy a used one!</span>",
        )
        .unwrap();

        assert_eq!(&["We buy a used one!"], &subtitle.lines[..]);
    }

    #[test]
    fn from_two_spans() {
        let subtitle = Subtitle::from_html(
            1,
            "whatever".into(),
            "<span>Yes, ok.</span> <span>What should we be called, then?</span>",
        )
        .unwrap();

        assert_eq!(
            &["Yes, ok.", "What should we be called, then?"],
            &subtitle.lines[..]
        );
    }
}
