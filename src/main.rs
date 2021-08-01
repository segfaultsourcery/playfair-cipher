use std::fmt::Display;

use color_eyre::Report;
use structopt::StructOpt;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Report> {
    let opt = Opt::from_args();
    setup(&opt)?;
    debug!("{:#?}", opt);

    let result = match opt.method {
        Method::Encipher { key, plaintext } => {
            let playfair = Playfair::new(key, opt.ignore_char);
            debug!("Playfair key:\n{}", playfair);
            playfair.encipher(&plaintext)?
        }
        Method::Decipher { key, ciphertext } => {
            let playfair = Playfair::new(key, opt.ignore_char);
            debug!("Playfair key:\n{}", playfair);
            playfair.decipher(&ciphertext)?
        }
    };

    info!("result = {:?}", result.pairs_to_string());

    Ok(())
}

fn setup(opt: &Opt) -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if opt.debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "playfair", about = "A Playfair cipher implementation.")]
struct Opt {
    /// Debug logging.
    #[structopt(short, long)]
    debug: bool,

    /// Char to ignore. Default is Q.
    #[structopt(short, long, default_value = "Q")]
    ignore_char: char,

    /// Encipher or decipher.
    #[structopt(subcommand)]
    method: Method,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Method")]
enum Method {
    Encipher {
        /// The key
        #[structopt(short, long)]
        key: String,

        /// Plaintext to hide.
        #[structopt()]
        plaintext: String,
    },
    Decipher {
        /// The key
        #[structopt(short, long)]
        key: String,

        /// Hidden text to reveal.
        #[structopt()]
        ciphertext: String,
    },
}

struct Playfair {
    key: Vec<char>,
}

impl Playfair {
    pub fn new(key: String, ignore_char: char) -> Self {
        let mut result = vec![];

        for c in key.to_lowercase().chars() {
            if (c >= 'a' && c <= 'z') && !result.contains(&c) && c != ignore_char {
                result.push(c)
            }
        }

        for c in 'a'..='z' {
            if !result.contains(&c) && c != ignore_char {
                result.push(c)
            }
        }

        Self { key: result }
    }

    pub fn encipher(&self, text: &str) -> Result<Vec<(char, char)>, Report> {
        self.transform_with_shapes(text, 1)
    }

    pub fn decipher(&self, text: &str) -> Result<Vec<(char, char)>, Report> {
        self.transform_with_shapes(text, 4)
    }

    fn transform_with_shapes(
        &self,
        text: &str,
        step_size: usize,
    ) -> Result<Vec<(char, char)>, Report> {
        let mut agg = vec![];

        for pair in self.chunkify(text) {
            let shape = self.find(&pair)?;

            let cipher = match shape {
                Shape::Rectangle { x1, y1, x2, y2 } => (self.at(x2, y1), self.at(x1, y2)),
                Shape::VerticalLine { x, y1, y2 } => {
                    (self.at(x, y1 + step_size), self.at(x, y2 + step_size))
                }
                Shape::HorizontalLine { y, x1, x2 } => {
                    (self.at(x1 + step_size, y), self.at(x2 + step_size, y))
                }
            };

            debug!("{} :: {:?} -> {:?}", &shape.name(), &pair, &cipher);

            agg.push(cipher);
        }

        Ok(agg)
    }

    fn find(&self, pair: &(char, char)) -> Result<Shape, Report> {
        let pos1 = self.find_single(&pair.0)?;
        let pos2 = self.find_single(&pair.1)?;

        let x1 = pos1 % 5;
        let y1 = pos1 / 5;
        let x2 = pos2 % 5;
        let y2 = pos2 / 5;

        match (x1, y1, x2, y2) {
            (x1, y1, x2, y2) if x1 == x2 => Ok(Shape::VerticalLine { x: x1, y1, y2 }),
            (x1, y1, x2, y2) if y1 == y2 => Ok(Shape::HorizontalLine { y: y1, x1, x2 }),
            (x1, y1, x2, y2) => Ok(Shape::Rectangle { x1, y1, x2, y2 }),
        }
    }

    fn find_single(&self, char_to_find: &char) -> Result<usize, Report> {
        let pos = self
            .key
            .iter()
            .enumerate()
            .find(|(_, c)| *c == char_to_find)
            .map(|(pos, _)| pos);

        match pos {
            Some(pos) => Ok(pos),
            None => {
                Err(PlayfairError::MissingCharacter(char_to_find.to_uppercase().to_string()).into())
            }
        }
    }

    fn chunkify(&self, text: &str) -> Vec<(char, char)> {
        let text: Vec<char> = text
            .to_lowercase()
            .chars()
            .filter(|c| *c >= 'a' && *c <= 'z')
            .collect();

        let mut text = text.chunks(2).fold(vec![], |mut agg, current| {
            if current.len() == 1 {
                agg.push(current[0]);
            } else {
                let a = current[0];
                let b = current[1];

                if a == b {
                    debug!("Found pair of doubles, {:?}, inserting X.", &current);
                    agg.push(a);
                    agg.push('x');
                    agg.push(b);
                } else {
                    agg.push(a);
                    agg.push(b);
                }
            }

            agg
        });

        if text.len() & 1 == 1 {
            text.push('x');
        }

        let pairs = text.chunks(2);
        pairs.map(|pair| (pair[0], pair[1])).collect()
    }

    fn at(&self, x: usize, y: usize) -> char {
        let x = x % 5; // Include % 5 here to deal with wrap-around.
        let y = y % 5; // Include % 5 here to deal with wrap-around.

        self.key[y * 5 + x]
    }
}

impl Display for Playfair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.key.chunks(5) {
            f.write_fmt(format_args!(
                "| {} |\n",
                chunk
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Shape {
    Rectangle {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    },
    VerticalLine {
        x: usize,
        y1: usize,
        y2: usize,
    },
    HorizontalLine {
        y: usize,
        x1: usize,
        x2: usize,
    },
}

impl Shape {
    fn name(&self) -> &'static str {
        match self {
            Shape::Rectangle { .. } => "Rectangle",
            Shape::VerticalLine { .. } => "VerticalLine",
            Shape::HorizontalLine { .. } => "HorizontalLine",
        }
    }
}

trait PairsToString {
    fn pairs_to_string(&self) -> String;
}

impl PairsToString for Vec<(char, char)> {
    fn pairs_to_string(&self) -> String {
        self.iter()
            .map(|(a, b)| format!("{}{}", a, b))
            .collect::<Vec<_>>()
            .join(" ")
            .to_uppercase()
    }
}

#[derive(Debug, thiserror::Error)]
enum PlayfairError {
    #[error("An unavailable character was used: `{0}`")]
    MissingCharacter(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const CIPHER: &str = "ME IK QO TX CQ TE ZX CO MW QC TE HN FB IK ME HA KR QC UN GI KM AV";

    // "laboulaye lady will lead to cibola temples of gold".
    const PLAIN: &str = "LA BO UL AY EL AD YW IL LX LE AD TO CI BO LA TE MP LE SO FG OL DX";
    const KEY: &str = "death";

    #[test]
    fn test_encipher() {
        let playfair = Playfair::new(KEY.into(), 'j');
        assert!(playfair.encipher(&PLAIN).unwrap().pairs_to_string() == CIPHER);
    }

    #[test]
    fn test_decipher() {
        let playfair = Playfair::new(KEY.into(), 'j');
        assert!(playfair.decipher(&CIPHER).unwrap().pairs_to_string() == PLAIN);
    }

}
