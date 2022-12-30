use std::fmt::Write;
use itertools::Itertools;
use crate::music::Note;
use colorize::{Colored, Color};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Fret {
    note: Note,
    interval: Option<usize>
}

impl Fret {
    fn to_string(&self) -> String {
        let bg = match self.interval {
            Some(i) => {
                match i {
                    1 => Color::BrightGreen,
                    3 => Color::BrightCyan,
                    5 => Color::BrightRed,
                    6 => Color::BrightYellow,
                    7 => Color::BrightBlue,
                    _ => Color::Cyan,
                }
            },
            None => Color::Default,
        };
        let fg = match self.interval {
            Some(_) => Color::BrightBlack,
            None    => Color::Default,
        };
        self.note.to_string().fg(fg).bg(bg)
    }
}

pub struct Guitar {
    frets: Vec<Vec<Fret>>,
}

impl Guitar {


    fn to_string(&self) -> Option<String> {
        let mut f = String::new();
        let count = self.frets[0].len() * 5 - 3;
        write!(f, "|{}|\n", "-".repeat(count)).ok().unwrap();
        for string in 0..self.frets.len() {
            let str = self.frets[string]
             .iter()
             .map(|x| x.to_string())
             .intersperse(" | ".to_string())
             .fold(String::new(), |str, s| str + &s);
            write!(f, "|{}|\n", str).ok().unwrap();
        }
        write!(f, "|{}|\n", "-".repeat(count)).ok().unwrap();
        let fret_markers = (0..self.frets[0].len()).into_iter()
         .map(|x| format!("{:<2}", x))
         .intersperse(" | ".to_string())
         .fold(String::new(), |str, s| str + &s);
        write!(f, "|{}|\n", fret_markers).ok().unwrap();
        write!(f, "|{}|\n", "-".repeat(count)).ok().unwrap();
        Some(f)
    }

    pub fn set_interval(&mut self, n: Note, interval: usize) {
        for i in 0..self.frets.len() {
            for j in 0..self.frets[0].len() {
                if self.frets[i][j].note == n {
                    self.frets[i][j].interval = Some(interval);
                }
            }
        }
    }
}

impl std::fmt::Debug for Guitar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_string() {
            Some(s) => write!(f, "{}", s),
            None            => Err(std::fmt::Error),
        }
    }
}

pub struct GuitarBuilder {
    frets: usize,
    tuning: Vec<Note>
}

impl GuitarBuilder {

    pub fn new() -> Self {
        GuitarBuilder { frets: 0, tuning: Vec::new() }
    }

    pub fn with_frets(&self, frets: usize) -> GuitarBuilder {
        GuitarBuilder { frets: frets, tuning: self.tuning.clone() }
    }

    pub fn with_tuning(&self, tuning: Vec<Note>) -> GuitarBuilder {
        GuitarBuilder { frets: self.frets, tuning: tuning }
    }

    pub fn build(&self) -> Guitar {
        let frets = self.tuning.iter().rev()
         .map(|x| x.into_iter()
            .take(self.frets)
            .map(|note| Fret{ note: note, interval: None })
            .collect::<Vec<_>>())
         .collect::<Vec<_>>();
        Guitar {
            frets: frets,
        }
    }
}