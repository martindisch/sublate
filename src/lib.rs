use eyre::{eyre, Result};
use itertools::Itertools;
use reqwest::blocking::Client;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use srt::Subtitles;

mod ffmpeg;
mod srt;
mod translation;

pub fn translate_subs(
    file: impl AsRef<Path>,
    source_language: &str,
    target_language: &str,
    client: &Client,
) -> Result<()> {
    let original_sub_file =
        build_subtitle_path(file.as_ref(), source_language)?;
    ffmpeg::extract_subtitle(file.as_ref(), &original_sub_file)?;

    let original_subs = srt::subtitles(&original_sub_file)?;
    let chunks_to_translate = original_subs.chunks(128);

    let translated_sub_path =
        build_subtitle_path(file.as_ref(), target_language)?;
    let translated_sub_file = File::create(&translated_sub_path)?;
    let mut translated_sub_writer = BufWriter::new(translated_sub_file);
    let combined_sub_path = build_subtitle_path(
        file.as_ref(),
        &format!("{}-{}", source_language, target_language),
    )?;
    let combined_sub_file = File::create(&combined_sub_path)?;
    let mut combined_sub_writer = BufWriter::new(combined_sub_file);

    for chunk in &chunks_to_translate {
        let original_chunk = Subtitles(chunk.collect());
        let translated_chunk = translation::translate(
            &original_chunk,
            source_language,
            target_language,
            client,
        )?;

        writeln!(translated_sub_writer, "{}", translated_chunk)?;

        let combined_chunk = original_chunk + translated_chunk;
        writeln!(combined_sub_writer, "{}", combined_chunk)?;
    }

    translated_sub_writer.flush()?;
    combined_sub_writer.flush()?;

    ffmpeg::combine_files(
        file.as_ref(),
        &translated_sub_path,
        &combined_sub_path,
    )?;

    Ok(())
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
