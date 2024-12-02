use std::{fs, path::Path};

use itertools::Itertools;

pub struct Report {
    levels: Vec<i32>,
}

impl Report {
    pub fn new(levels: Vec<i32>) -> Self {
        Self { levels }
    }

    pub fn is_safe(&self, eval_with_dampener: bool) -> (bool, bool) {
        let mut sort_direction: Option<u8> = Option::None;
        let mut linear = true;
        let mut correctly_spaced = true;
        let mut safe_dampened: bool = false;
        for (l, r) in self.levels.iter().tuple_windows() {
            let diff = l.abs_diff(*r);
            correctly_spaced = diff >= 1 && diff <= 3;

            match sort_direction {
                Some(direction) => {
                    if (l < r && direction == 0) || (l > r && direction == 1) {
                        linear = false;
                    }
                }
                None => {
                    if l < r {
                        sort_direction = Option::Some(1); // ASC
                    } else {
                        sort_direction = Option::Some(0); // DESC
                    }
                }
            }

            if !(correctly_spaced && linear) {
                if eval_with_dampener {
                    for i in 0..self.levels.len() {
                        let mut permutation = self.levels.clone();
                        permutation.remove(i);
                        safe_dampened = Report { levels: permutation }.is_safe(false).0;

                        if safe_dampened {
                            break;
                        }
                    }
                    
                }
                break;
            }
        }

        (linear && correctly_spaced, safe_dampened)
    }
}

pub struct ReportCollection {
    reports: Vec<Report>,
}

impl ReportCollection {
    pub fn count_safe(&self) -> usize {
        self.reports.iter().fold(0, |acc, report| {
            let (safe, _n_failures) = report.is_safe(false);
            if safe {
                acc + 1
            } else {
                acc
            }
        })
    }

    pub fn count_safe_dampened(&self) -> usize {
        self.reports.iter().fold(0, |acc, report| {
            let (safe, safe_dampened) = report.is_safe(true);
            if safe || safe_dampened {
                acc + 1
            } else {
                acc
            }
        })
    }
}

impl<P: AsRef<Path>> From<P> for ReportCollection {
    fn from(file: P) -> Self {
        let raw_content = fs::read_to_string(file).expect("Could not read file");
        let mut reports: Vec<Report> = vec![];

        raw_content.lines().for_each(|line| {
            reports.push(Report::new(
                line.split(" ")
                    .map(|s| s.parse::<i32>().expect("Invalid level, not a number"))
                    .collect(),
            ));
        });

        Self { reports }
    }
}
