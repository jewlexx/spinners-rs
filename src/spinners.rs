use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;
use strum::{Display, EnumIter, EnumString};

struct Spinner {
    pub spinner: Spinners,
    pub frames: Vec<char>,
    pub interval: u64,
}

pub enum Error {
    UnknownSpinner(String),
}

impl Spinner {
    pub fn new(spinner: Spinners, interval: Option<u64>) -> Result<Self, Error> {
        let frames = SPINNER_MAP.get(&spinner.to_string());

        if let Some(frames) = frames {
            Ok(Self {
                spinner,
                frames: frames.chars().collect(),
                interval: interval.unwrap_or(100),
            })
        } else {
            Err(Error::UnknownSpinner(spinner.to_string()))
        }
    }
}

#[derive(Debug, Clone, EnumIter, Display, EnumString)]
pub enum Spinners {
    Dots,
    Dots2,
    Dots3,
    Dots4,
    Dots5,
    Dots6,
    Dots7,
    Dots8,
    Dots9,
    Dots10,
    Dots11,
    Pipe,
    Star,
    Star2,
    Flip,
    Hamburger,
    GrowVertical,
    GrowHorizontal,
    Balloon,
    Balloon2,
    Noise,
    Bounce,
    BoxBounce,
    BoxBounce2,
    Triangle,
    Arc,
    Circle,
    SquareCorners,
    CircleQuaters,
    CircleHalves,
    Squish,
    Toggle,
    Toggle2,
    Toggle3,
    Toggle4,
    Toggle5,
    Toggle6,
    Toggle7,
    Toggle8,
    Toggle9,
    Toggle10,
    Toggle11,
    Toggle12,
    Toggle13,
    Arrow,
}

lazy_static! {
    static ref SPINNER_MAP: HashMap<String, &'static str> = {
        hashmap! {
        "dots".into() => "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
        "dots2".into() => "⣾⣽⣻⢿⡿⣟⣯⣷",
        "dots3".into() => "⠋⠙⠚⠞⠖⠦⠴⠲⠳⠓",
        "dots4".into() => "⠄⠆⠇⠋⠙⠸⠰⠠⠰⠸⠙⠋⠇⠆",
        "dots5".into() => "⠋⠙⠚⠒⠂⠂⠒⠲⠴⠦⠖⠒⠐⠐⠒⠓⠋",
        "dots6".into() => "⠁⠉⠙⠚⠒⠂⠂⠒⠲⠴⠤⠄⠄⠤⠴⠲⠒⠂⠂⠒⠚⠙⠉⠁",
        "dots7".into() => "⠈⠉⠋⠓⠒⠐⠐⠒⠖⠦⠤⠠⠠⠤⠦⠖⠒⠐⠐⠒⠓⠋⠉⠈",
        "dots8".into() => "⠁⠁⠉⠙⠚⠒⠂⠂⠒⠲⠴⠤⠄⠄⠤⠠⠠⠤⠦⠖⠒⠐⠐⠒⠓⠋⠉⠈⠈",
        "dots9".into() => "⢹⢺⢼⣸⣇⡧⡗⡏",
        "dots10".into() => "⢄⢂⢁⡁⡈⡐⡠",
        "dots11".into() => "⠁⠂⠄⡀⢀⠠⠐⠈",
        "pipe".into() => "┤┘┴└├┌┬┐",
        "star".into() => "✶✸✹✺✹✷",
        "star2".into() => "+x*",
        "flip".into() => "___-``'´-___",
        "hamburger".into() => "☱☲☴",
        "growVertical".into() => "▁▃▄▅▆▇▆▅▄▃",
        "growHorizontal".into() => "▏▎▍▌▋▊▉▊▋▌▍▎",
        "balloon".into() => ".into() .oO@* ",
        "balloon2".into() => ".oO°Oo.",
        "noise".into() => "▓▒░",
        "bounce".into() => "⠁⠂⠄⠂",
        "boxBounce".into() => "▖▘▝▗",
        "boxBounce2".into() => "▌▀▐▄",
        "triangle".into() => "◢◣◤◥",
        "arc".into() => "◜◠◝◞◡◟",
        "circle".into() => "◡⊙◠",
        "squareCorners".into() => "◰◳◲◱",
        "circleQuarters".into() => "◴◷◶◵",
        "circleHalves".into() => "◐◓◑◒",
        "squish".into() => "╫╪",
        "toggle".into() => "⊶⊷",
        "toggle2".into() => "▫▪",
        "toggle3".into() => "□■",
        "toggle4".into() => "■□▪▫",
        "toggle5".into() => "▮▯",
        "toggle6".into() => "ဝ၀",
        "toggle7".into() => "⦾⦿",
        "toggle8".into() => "◍◌",
        "toggle9".into() => "◉◎",
        "toggle10".into() => "㊂㊀㊁",
        "toggle11".into() => "⧇⧆",
        "toggle12".into() => "☗☖",
        "toggle13".into() => "=*-",
        "arrow".into() => "←↖↑↗→↘↓↙"
        }
    };
}
