use clap::Parser;
use std::{
    io::{self, Write},
    str::FromStr,
};

use lib::{HELP_MAX, HELP_SEPARATOR, HELP_THEME};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
enum Theme {
    Blocks,
    Shade,
    Wave,
    Weird,
}

const THEME_BLOCKS: &str = "▁▂▃▄▅▆▇█";
const THEME_SHADE: &str = "░▒▓█";
const THEME_WAVE: &str = "⎽⎼⎻⎺";
const THEME_WEIRD: &str = "▖▘▝▗▚▞█";

#[allow(dead_code)]
impl Theme {
    fn literal(self) -> &'static str {
        match self {
            Self::Blocks => THEME_BLOCKS,
            Self::Shade => THEME_SHADE,
            Self::Wave => THEME_WAVE,
            Self::Weird => THEME_WEIRD,
        }
    }

    fn parts(self) -> impl Iterator<Item = char> {
        self.literal().chars()
    }

    fn len(self) -> usize {
        self.parts().count()
    }

    fn default(self) -> char {
        self.parts().next().unwrap()
    }
}

#[derive(Parser)]
struct Args {
    #[arg(
        long,
        value_name = "MAX",
        value_parser = Args::parse_base,
        help = HELP_MAX
    )]
    max: Option<usize>,

    #[arg(
        long,
        value_name = "CHAR",
        default_value = ";",
        help = HELP_SEPARATOR
    )]
    separator: char,

    #[arg(
        long,
        value_name = "THEME",
        value_parser = Args::parse_theme,
        default_value = "blocks",
        help = HELP_THEME
    )]
    theme: Theme,
}

impl Args {
    fn parse_base(s: &str) -> Result<usize, String> {
        let value = s.parse::<usize>().map_err(|err| err.to_string())?;

        if value < 1 {
            return Err(String::from("base must be >= 1"));
        }

        Ok(value)
    }

    fn parse_theme(s: &str) -> Result<Theme, String> {
        Theme::from_str(s).map_err(|err| err.to_string())
    }
}

fn main() {
    let args = Args::parse();

    let stdin = io::stdin();
    let lines = stdin.lines().filter_map(Result::ok);

    for line in lines {
        eprintln!("{:?}", line);
        let parts = line
            .trim()
            .split(args.separator)
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        if parts.is_empty() {
            return;
        }

        let max = args
            .max
            .unwrap_or_else(|| parts.iter().copied().max().unwrap_or(1));

        let chars = args.theme.parts().collect::<Vec<_>>();
        let len = chars.len();

        let wave = parts
            .iter()
            .map(|value| {
                let normalized = (*value as f32 / max as f32).min(1.0);
                let scaled = (normalized * (len - 1) as f32).round();
                chars
                    .get(scaled as usize)
                    .cloned()
                    .unwrap_or(args.theme.default())
            })
            .collect::<String>();

        println!("{}", wave);
        std::io::stdout().flush().unwrap();
    }
}
