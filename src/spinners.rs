use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;
use strum::{Display, EnumIter, EnumString};

use crate::Error;

#[derive(Debug, Clone, Copy, EnumIter, Display, EnumString)]
#[strum(serialize_all = "camelCase")]
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

impl Spinners {
    pub fn get_frames(&self) -> Result<Vec<char>, Error> {
        if let Some(frames) = SPINNER_MAP.get(&self.to_string()) {
            Ok(frames.chars().collect())
        } else {
            Err(Error::UnknownSpinner(self.to_string()))
        }
    }
}

lazy_static! {
    pub(crate) static ref SPINNER_MAP: HashMap<String, &'static str> = {
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
            "circleQuaters".into() => "◴◷◶◵",
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
