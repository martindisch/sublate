use eyre::Result;
use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    ops,
    path::Path,
};

pub fn subtitles(file: &Path) -> Result<SubtitleIter> {
    let line_iter = file_lines(file)?;
    let sub_iter = SubtitleIter(Box::from(line_iter));

    Ok(sub_iter)
}

#[derive(Debug, PartialEq)]
pub struct Subtitle {
    pub counter: u32,
    pub timestamp: String,
    pub lines: Vec<String>,
}

impl fmt::Display for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n",
            self.counter,
            self.timestamp,
            self.lines.join("\n")
        )
    }
}

impl ops::Add for Subtitle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut lines = self.lines;
        lines.extend(other.lines);

        Self {
            counter: self.counter,
            timestamp: self.timestamp,
            lines,
        }
    }
}

#[derive(Debug)]
pub struct Subtitles(pub Vec<Subtitle>);

impl fmt::Display for Subtitles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = self.0.len();

        for subtitle in self.0.iter().take(count - 1) {
            subtitle.fmt(f)?;
            writeln!(f)?;
        }

        self.0[count - 1].fmt(f)
    }
}

impl ops::Add for Subtitles {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let subs_combined = self
            .0
            .into_iter()
            .zip(other.0.into_iter())
            .map(|(sub1, sub2)| sub1 + sub2)
            .collect();

        Self(subs_combined)
    }
}

pub struct SubtitleIter(Box<dyn Iterator<Item = String>>);

impl Iterator for SubtitleIter {
    type Item = Subtitle;

    fn next(&mut self) -> Option<Self::Item> {
        let counter = self.0.next().and_then(|s| s.parse::<u32>().ok())?;
        let timestamp = self.0.next()?;
        let mut lines: Vec<_> = Vec::with_capacity(2);
        for s in &mut self.0 {
            if s.is_empty() {
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

fn file_lines(path: &Path) -> Result<impl Iterator<Item = String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|r| r.expect("Fatal error while reading lines")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_lines() {
        let lines = vec![];
        let iter = SubtitleIter(Box::from(lines.into_iter()));
        let subtitles: Vec<_> = iter.collect();

        assert_eq!(0, subtitles.len());
    }

    #[test]
    fn empty_line() {
        let lines = vec!["".into()];
        let iter = SubtitleIter(Box::from(lines.into_iter()));
        let subtitles: Vec<_> = iter.collect();

        assert_eq!(0, subtitles.len());
    }

    #[test]
    fn empty_lines() {
        let lines = vec!["".into(), "".into()];
        let iter = SubtitleIter(Box::from(lines.into_iter()));
        let subtitles: Vec<_> = iter.collect();

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
        let subtitles: Vec<_> = iter.collect();

        assert_eq!(
            &[Subtitle {
                counter: 1,
                timestamp: "00:00:14,600 --> 00:00:20,000".into(),
                lines: vec![
                    "Hvis vi jobber rundt ...".into(),
                    "Her er vannet dypere.".into()
                ],
            }],
            &subtitles[..]
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
        let subtitles: Vec<_> = iter.collect();

        assert_eq!(
            &[
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
            &subtitles[..]
        );
    }

    #[test]
    fn add_subtitle() {
        let sub1 = Subtitle {
            counter: 1,
            timestamp: "00:00:14,600 --> 00:00:20,000".into(),
            lines: vec![
                "Hvis vi jobber rundt ...".into(),
                "Her er vannet dypere.".into(),
            ],
        };
        let sub2 = Subtitle {
            counter: 2,
            timestamp: "00:00:21,280 --> 00:00:26,960".into(),
            lines: vec![
                "If we work around ...".into(),
                "Here the water is deeper.".into(),
            ],
        };

        let sub_combined = sub1 + sub2;

        assert_eq!(
            &[
                "Hvis vi jobber rundt ...",
                "Her er vannet dypere.",
                "If we work around ...",
                "Here the water is deeper."
            ],
            &sub_combined.lines[..]
        );
    }

    #[test]
    fn add_subtitles() {
        let subs1 = Subtitles(vec![Subtitle {
            counter: 1,
            timestamp: "00:00:14,600 --> 00:00:20,000".into(),
            lines: vec![
                "Hvis vi jobber rundt ...".into(),
                "Her er vannet dypere.".into(),
            ],
        }]);
        let subs2 = Subtitles(vec![Subtitle {
            counter: 2,
            timestamp: "00:00:21,280 --> 00:00:26,960".into(),
            lines: vec![
                "If we work around ...".into(),
                "Here the water is deeper.".into(),
            ],
        }]);

        let subs_combined = subs1 + subs2;

        assert_eq!(
            &[
                "Hvis vi jobber rundt ...",
                "Her er vannet dypere.",
                "If we work around ...",
                "Here the water is deeper."
            ],
            &subs_combined.0[0].lines[..]
        );
    }
}
