use eyre::{eyre, Result};
use reqwest::Client;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

mod srt;

pub fn translate_subs(
    file: impl AsRef<Path>,
    source_language: &str,
    target_language: &str,
    client: &Client,
) -> Result<()> {
    let original_sub = extract_subtitle(file.as_ref())?;

    Ok(())
}

fn extract_subtitle(file: &Path) -> Result<PathBuf> {
    let file_stem = file
        .file_stem()
        .ok_or_else(|| eyre!("Could not extract file stem from {:?}", file))?;

    let mut subtitle = env::temp_dir();
    subtitle.push(file_stem);
    subtitle.set_extension("srt");

    let output = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i",
            file.to_str()
                .ok_or_else(|| eyre!("Could not convert input path to str"))?,
            subtitle
                .to_str()
                .ok_or_else(|| eyre!("Could not convert sub path to str"))?,
        ])
        .output()?;

    if output.status.success() {
        Ok(subtitle)
    } else {
        Err(eyre!("Could not extract subtitle from {:?}", file))
    }
}
