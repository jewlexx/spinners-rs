use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

struct Spinner {
    name: String,
    frames: Vec<char>,
}

impl Spinner {
    pub fn new() -> Self {}
}

pub enum Spinner {
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
    static ref SPINNER_MAP: HashMap<&'static str, &'static str> = {
        hashmap! {
        "dots" => "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
        "dots2" => "⣾⣽⣻⢿⡿⣟⣯⣷",
        "dots3" => "⠋⠙⠚⠞⠖⠦⠴⠲⠳⠓",
        "dots4" => "⠄⠆⠇⠋⠙⠸⠰⠠⠰⠸⠙⠋⠇⠆",
        "dots5" => "⠋⠙⠚⠒⠂⠂⠒⠲⠴⠦⠖⠒⠐⠐⠒⠓⠋",
        "dots6" => "⠁⠉⠙⠚⠒⠂⠂⠒⠲⠴⠤⠄⠄⠤⠴⠲⠒⠂⠂⠒⠚⠙⠉⠁",
        "dots7" => "⠈⠉⠋⠓⠒⠐⠐⠒⠖⠦⠤⠠⠠⠤⠦⠖⠒⠐⠐⠒⠓⠋⠉⠈",
        "dots8" => "⠁⠁⠉⠙⠚⠒⠂⠂⠒⠲⠴⠤⠄⠄⠤⠠⠠⠤⠦⠖⠒⠐⠐⠒⠓⠋⠉⠈⠈",
        "dots9" => "⢹⢺⢼⣸⣇⡧⡗⡏",
        "dots10" => "⢄⢂⢁⡁⡈⡐⡠",
        "dots11" => "⠁⠂⠄⡀⢀⠠⠐⠈",
        "pipe" => "┤┘┴└├┌┬┐",
        "star" => "✶✸✹✺✹✷",
        "star2" => "+x*",
        "flip" => "___-``'´-___",
        "hamburger" => "☱☲☴",
        "growVertical" => "▁▃▄▅▆▇▆▅▄▃",
        "growHorizontal" => "▏▎▍▌▋▊▉▊▋▌▍▎",
        "balloon" => " .oO@* ",
        "balloon2" => ".oO°Oo.",
        "noise" => "▓▒░",
        "bounce" => "⠁⠂⠄⠂",
        "boxBounce" => "▖▘▝▗",
        "boxBounce2" => "▌▀▐▄",
        "triangle" => "◢◣◤◥",
        "arc" => "◜◠◝◞◡◟",
        "circle" => "◡⊙◠",
        "squareCorners" => "◰◳◲◱",
        "circleQuarters" => "◴◷◶◵",
        "circleHalves" => "◐◓◑◒",
        "squish" => "╫╪",
        "toggle" => "⊶⊷",
        "toggle2" => "▫▪",
        "toggle3" => "□■",
        "toggle4" => "■□▪▫",
        "toggle5" => "▮▯",
        "toggle6" => "ဝ၀",
        "toggle7" => "⦾⦿",
        "toggle8" => "◍◌",
        "toggle9" => "◉◎",
        "toggle10" => "㊂㊀㊁",
        "toggle11" => "⧇⧆",
        "toggle12" => "☗☖",
        "toggle13" => "=*-",
        "arrow" => "←↖↑↗→↘↓↙"
        }
    };
}
