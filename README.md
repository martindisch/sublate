# sublate

This tool extracts subtitles from a video file and translates them with
Google's Cloud Translation API. It creates new sub tracks for a target
language, or even a combined track showing both the source and target language
at the same time, which is particularly helpful when learning a language.

## Usage

Requirements:
* `ffmpeg` needs to be installed locally.
* To use the Cloud Translation API you need to obtain an API key as described
  in the [documentation](https://cloud.google.com/translate/docs/setup). Next
  you have to generate the access token (valid for 60 minutes) with
  `GOOGLE_APPLICATION_CREDENTIALS=<PATH_TO_JSON> gcloud auth
  application-default print-access-token`.

Then you can use the application as such:
```text
sublate 0.1.0
Martin Disch <martindisch@gmail.com>
Translates and combine video subtitles.

USAGE:
    sublate <FILE>... --access-token <token> --source <lang> --target <lang>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --access-token <token>    Access token for the Cloud Translation API
    -s, --source <lang>           Source language ISO 639-1 code
    -t, --target <lang>           Target language ISO 639-1 code

ARGS:
    <FILE>...    The original video file(s)
```

## License
Licensed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
