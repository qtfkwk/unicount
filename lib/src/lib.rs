#[doc = include_str!("../README.md")]
use {clap::ValueEnum, unicode_segmentation::UnicodeSegmentation};

fn split(s: &str) -> Vec<String> {
    UnicodeSegmentation::graphemes(s, true)
        .map(|x| x.to_string())
        .collect()
}

pub struct Counter {
    current: usize,
    alphabet: Vec<String>,
}

impl Counter {
    pub fn new(alphabet: &str, start: usize) -> Counter {
        Counter {
            current: start,
            alphabet: split(alphabet),
        }
    }

    fn get(&self, x: usize) -> String {
        let length = self.alphabet.len();
        let d = x / length;
        let r = x % length;
        let t = self.alphabet[r].clone();
        if d == 0 {
            t
        } else {
            format!("{}{t}", self.get(d - 1))
        }
    }
}

impl std::iter::Iterator for Counter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.to_string();
        self.current += 1;
        Some(r)
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get(self.current))
    }
}

#[derive(Clone, ValueEnum)]
pub enum Kind {
    EnglishUpper,
    EnglishLower,
}

impl Kind {
    pub fn counter(&self, start: usize) -> Counter {
        match self {
            Kind::EnglishUpper => Counter::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", start),
            Kind::EnglishLower => Counter::new("abcdefghijklmnopqrstuvwxyz", start),
        }
    }
}
