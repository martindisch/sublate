use eyre::Result;
use reqwest::Client;
use std::path::Path;

pub fn translate_subs(
    file: impl AsRef<Path>,
    source_language: &str,
    target_language: &str,
    client: &Client,
) -> Result<()> {
    Ok(())
}
