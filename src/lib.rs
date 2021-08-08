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

use srt::Subtitles;

pub fn translate_subs(
    file: impl AsRef<Path>,
    source_language: &str,
    target_language: &str,
    client: &Client,
) -> Result<()> {
    let original_sub_file = extract_subtitle(file.as_ref(), source_language)?;
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
        let original_chunk: Vec<_> = chunk.collect();
        let translated_chunk = translation::translate(
            &original_chunk,
            source_language,
            target_language,
            client,
        )?;

        writeln!(translated_sub_writer, "{}", translated_chunk)?;

        let combined_chunks = Subtitles(original_chunk) + translated_chunk;
        writeln!(combined_sub_writer, "{}", combined_chunks)?;
    }

    translated_sub_writer.flush()?;
    combined_sub_writer.flush()?;

    combine_files(file.as_ref(), &translated_sub_path, &combined_sub_path)?;

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

fn combine_files(
    video: &Path,
    target_sub: &Path,
    combined_sub: &Path,
) -> Result<()> {
    let file_name = video.file_name().ok_or_else(|| {
        eyre!("Could not extract file name from {:?}", video)
    })?;

    let mut output_video = env::temp_dir();
    output_video.push(file_name);

    let output = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i",
            video
                .to_str()
                .ok_or_else(|| eyre!("Could not convert video path to str"))?,
            "-i",
            target_sub.to_str().ok_or_else(|| {
                eyre!("Could not convert target sub path to str")
            })?,
            "-i",
            combined_sub.to_str().ok_or_else(|| {
                eyre!("Could not convert combined sub path to str")
            })?,
            "-map",
            "0",
            "-map",
            "1",
            "-map",
            "2",
            "-c",
            "copy",
            // TODO: support MKV files (-c:s srt instead of mov_text)
            "-c:s",
            "mov_text",
            output_video.to_str().ok_or_else(|| {
                eyre!("Could not convert output path to str")
            })?,
        ])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        std::io::stdout().write_all(&output.stderr)?;
        Err(eyre!("Could not write output to {:?}", output_video))
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
