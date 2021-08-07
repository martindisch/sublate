use clap::{App, Arg};
use eyre::Result;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};

fn main() -> Result<()> {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about("Translates and combine video subtitles.")
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .value_name("lang")
                .takes_value(true)
                .required(true)
                .help("Source language ISO 639-1 code"),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("lang")
                .takes_value(true)
                .required(true)
                .help("Target language ISO 639-1 code"),
        )
        .arg(
            Arg::with_name("access-token")
                .short("a")
                .long("access-token")
                .value_name("token")
                .takes_value(true)
                .required(true)
                .help("Access token for the Cloud Translation API"),
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

    let mut headers = HeaderMap::new();
    let mut auth = HeaderValue::from_str(&format!("Bearer {}", access_token))?;
    auth.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth);
    let client = Client::builder().default_headers(headers).build()?;

    // TODO: Parallelize and call for all input files
    sublate::translate_subs(
        files[0],
        source_language,
        target_language,
        &client,
    )?;

    Ok(())
}
