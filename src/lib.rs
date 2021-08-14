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

pub fn translate_subtitle(
    video: impl AsRef<Path>,
    source_language: &str,
    target_language: &str,
    client: &Client,
) -> Result<()> {
    let filename = get_file_stem(video.as_ref())?;
    let output_dir = env::temp_dir();

    let original_sub =
        build_subtitle_path(&output_dir, &filename, source_language);
    ffmpeg::extract_subtitle(video.as_ref(), &original_sub)?;

    let original_subtitles_iter = srt::subtitles(&original_sub)?;
    let chunks_to_translate = original_subtitles_iter.chunks(128);

    let (translated_sub, mut translated_sub_writer) =
        build_subtitle_writer(&output_dir, &filename, target_language)?;
    let (combined_sub, mut combined_sub_writer) = build_subtitle_writer(
        &output_dir,
        &filename,
        &format!("{}-{}", source_language, target_language),
    )?;

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
        video.as_ref(),
        &translated_sub,
        &combined_sub,
        &output_dir,
    )?;

    Ok(())
}

fn get_file_stem(path: &Path) -> Result<String> {
    Ok(path
        .file_stem()
        .ok_or_else(|| eyre!("Could not extract file stem from {:?}", path))?
        .to_str()
        .ok_or_else(|| eyre!("Could not convert OsStr for {:?}", path))?
        .to_string())
}

fn build_subtitle_path(
    directory: &Path,
    filename: &str,
    suffix: &str,
) -> PathBuf {
    let mut path = directory.to_path_buf();
    path.push(format!("{}_{}.srt", filename, suffix));

    path
}

fn build_subtitle_writer(
    directory: &Path,
    filename: &str,
    suffix: &str,
) -> Result<(PathBuf, BufWriter<File>)> {
    let path = build_subtitle_path(directory, filename, suffix);
    let file = File::create(&path)?;

    Ok((path, BufWriter::new(file)))
}
