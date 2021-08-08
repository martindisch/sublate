use eyre::{eyre, Result};
use itertools::Itertools;
use reqwest::blocking::Client;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
};

mod srt;
mod translation;

pub fn translate_subs(
    file: impl AsRef<Path>,
    source_language: &str,
    target_language: &str,
    client: &Client,
) -> Result<()> {
    let original_sub_file = extract_subtitle(file.as_ref(), source_language)?;
    let original_subs = srt::subtitles(&original_sub_file)?;
    let chunks_to_translate = original_subs.chunks(128);

    let translated_sub_file =
        File::create(build_subtitle_path(file.as_ref(), target_language)?)?;
    let mut translated_sub_writer = BufWriter::new(translated_sub_file);

    for chunk in &chunks_to_translate {
        let original_chunk: Vec<_> = chunk.collect();
        let translated_chunk = translation::translate(
            &original_chunk,
            source_language,
            target_language,
            client,
        )?;

        writeln!(translated_sub_writer, "{}", translated_chunk)?;
    }
    translated_sub_writer.flush()?;

    Ok(())
}

fn extract_subtitle(file: &Path, source_language: &str) -> Result<PathBuf> {
    let subtitle = build_subtitle_path(file, source_language)?;

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

fn build_subtitle_path(file: &Path, language: &str) -> Result<PathBuf> {
    let file_stem = file
        .file_stem()
        .ok_or_else(|| eyre!("Could not extract file stem from {:?}", file))?
        .to_str()
        .ok_or_else(|| eyre!("Could not convert OsStr to str"))?;

    let mut subtitle = env::temp_dir();
    subtitle.push(format!("{}_{}.srt", file_stem, language));

    Ok(subtitle)
}
