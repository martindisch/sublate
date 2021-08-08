use eyre::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    path::Path,
};

#[derive(Debug, PartialEq)]
pub struct Subtitle {
    counter: u32,
    timestamp: String,
    lines: Vec<String>,
}

pub struct SubtitleIter(Box<dyn Iterator<Item = String>>);

impl Iterator for SubtitleIter {
    type Item = Subtitle;

    fn next(&mut self) -> Option<Self::Item> {
        let counter = self.0.next().and_then(|s| s.parse::<u32>().ok())?;
        let timestamp = self.0.next()?;
        let mut lines: Vec<String> = Vec::with_capacity(2);
        while let Some(s) = self.0.next() {
            if s == "" {
                break;
            }
            lines.push(s);
        }

        Some(Subtitle {
            counter,
            timestamp,
            lines,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_lines() {
        let lines = vec![];
        let iter = SubtitleIter(Box::from(lines.into_iter()));
        let subtitles: Vec<Subtitle> = iter.collect();

        assert_eq!(0, subtitles.len());
    }

    #[test]
    fn empty_line() {
        let lines = vec!["".into()];
        let iter = SubtitleIter(Box::from(lines.into_iter()));
        let subtitles: Vec<Subtitle> = iter.collect();

        assert_eq!(0, subtitles.len());
    }

    #[test]
    fn empty_lines() {
        let lines = vec!["".into(), "".into()];
        let iter = SubtitleIter(Box::from(lines.into_iter()));
        let subtitles: Vec<Subtitle> = iter.collect();

        assert_eq!(0, subtitles.len());
    }

    #[test]
    fn one_sub() {
        let lines = vec![
            "1".into(),
            "00:00:14,600 --> 00:00:20,000".into(),
            "Hvis vi jobber rundt ...".into(),
            "Her er vannet dypere.".into(),
        ];
        let iter = SubtitleIter(Box::from(lines.into_iter()));
        let subtitles: Vec<Subtitle> = iter.collect();

        assert_eq!(
            vec![Subtitle {
                counter: 1,
                timestamp: "00:00:14,600 --> 00:00:20,000".into(),
                lines: vec![
                    "Hvis vi jobber rundt ...".into(),
                    "Her er vannet dypere.".into()
                ],
            }],
            subtitles
        );
    }

    #[test]
    fn two_subs() {
        let lines = vec![
            "1".into(),
            "00:00:14,600 --> 00:00:20,000".into(),
            "Hvis vi jobber rundt ...".into(),
            "Her er vannet dypere.".into(),
            "".into(),
            "2".into(),
            "00:00:21,280 --> 00:00:26,960".into(),
            "Hvis vi ser på alternativ 1 først, Jåttå-".into(),
            "vågen, der er det et par problemer.".into(),
            "".into(),
        ];
        let iter = SubtitleIter(Box::from(lines.into_iter()));
        let subtitles: Vec<Subtitle> = iter.collect();

        assert_eq!(
            vec![
                Subtitle {
                    counter: 1,
                    timestamp: "00:00:14,600 --> 00:00:20,000".into(),
                    lines: vec![
                        "Hvis vi jobber rundt ...".into(),
                        "Her er vannet dypere.".into()
                    ],
                },
                Subtitle {
                    counter: 2,
                    timestamp: "00:00:21,280 --> 00:00:26,960".into(),
                    lines: vec![
                        "Hvis vi ser på alternativ 1 først, Jåttå-".into(),
                        "vågen, der er det et par problemer.".into()
                    ],
                }
            ],
            subtitles
        );
    }
}
