use clap::{App, Arg};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about("Translates and combine video subtitles.")
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .value_name("language")
                .takes_value(true)
                .required(true)
                .help("The source language ISO 639-1 code"),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("language")
                .takes_value(true)
                .required(true)
                .help("The target language ISO 639-1 code"),
        )
        .arg(
            Arg::with_name("access-token")
                .short("a")
                .long("access-token")
                .value_name("token")
                .takes_value(true)
                .required(true)
                .help("Your access token for the Cloud Translation API"),
        )
        .arg(
            Arg::with_name("INPUT")
                .value_name("FILE")
                .multiple(true)
                .required(true)
                .help("The original video file(s)"),
        )
        .get_matches();

    // If we get here, unwrap is safe on mandatory arguments
    let source_language = matches.value_of("source").unwrap();
    let target_language = matches.value_of("target").unwrap();
    let access_token = matches.value_of("access-token").unwrap();
    let files: Vec<_> = matches.values_of("INPUT").unwrap().collect();

    println!(
        "{}, {}, {}, {}",
        source_language,
        target_language,
        access_token,
        files.join(",")
    );
}
