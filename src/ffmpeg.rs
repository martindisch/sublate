use eyre::{eyre, Result};
use std::{
    io::{self, Write},
    path::Path,
    process::Command,
};

pub fn extract_subtitle(video: &Path, output: &Path) -> Result<()> {
    let ffmpeg_output = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i",
            video
                .to_str()
                .ok_or_else(|| eyre!("Could not convert input path to str"))?,
            output
                .to_str()
                .ok_or_else(|| eyre!("Could not convert sub path to str"))?,
        ])
        .output()?;

    if ffmpeg_output.status.success() {
        Ok(())
    } else {
        io::stdout().write_all(&ffmpeg_output.stderr)?;
        Err(eyre!("Could not extract subtitle from {:?}", video))
    }
}

pub fn combine_files(
    video: &Path,
    target_sub: &Path,
    combined_sub: &Path,
    output_dir: &Path,
) -> Result<()> {
    let filename = video.file_name().ok_or_else(|| {
        eyre!("Could not extract file name from {:?}", video)
    })?;
    let extension = video.extension().ok_or_else(|| {
        eyre!("Could not extract extension from {:?}", video)
    })?;
    let subtitle_arg = if extension == "mkv" {
        "srt"
    } else {
        "mov_text"
    };

    let mut output = output_dir.to_path_buf();
    output.push(filename);

    let ffmpeg_output = Command::new("ffmpeg")
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
            "-c:s",
            subtitle_arg,
            output.to_str().ok_or_else(|| {
                eyre!("Could not convert output path to str")
            })?,
        ])
        .output()?;

    if ffmpeg_output.status.success() {
        Ok(())
    } else {
        io::stdout().write_all(&ffmpeg_output.stderr)?;
        Err(eyre!("Could not write output to {:?}", output))
    }
}
