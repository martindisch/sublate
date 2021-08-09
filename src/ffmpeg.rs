use eyre::{eyre, Result};
use std::{
    env,
    io::{self, Write},
    path::Path,
    process::Command,
};

pub fn extract_subtitle(file: &Path, output: &Path) -> Result<()> {
    let ffmpeg_output = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i",
            file.to_str()
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
        Err(eyre!("Could not extract subtitle from {:?}", file))
    }
}

pub fn combine_files(
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
        io::stdout().write_all(&output.stderr)?;
        Err(eyre!("Could not write output to {:?}", output_video))
    }
}