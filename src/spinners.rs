use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumIter, Display, EnumString)]
#[strum(serialize_all = "camelCase")]
/// The enum of all available spinners
///
/// Implements [`Display`], [`EnumString`] and [`EnumIter`] traits for easy usage.
///
/// Additionally implements [`Into<Spinner>`] trait to be easily converted into a [`Spinner`] struct.
///
/// [`Spinner`]: ./struct.Spinner.html
///
/// Can also be iterated over
pub enum Spinners {
    /// The Dots spinner
    Dots,
    /// The Dots2 spinner
    Dots2,
    /// The Dots3 spinner
    Dots3,
    /// The Dots4 spinner
    Dots4,
    /// The Dots5 spinner
    Dots5,
    /// The Dots6 spinner
    Dots6,
    /// The Dots7 spinner
    Dots7,
    /// The Dots8 spinner
    Dots8,
    /// The Dots9 spinner
    Dots9,
    /// The Dots10 spinner
    Dots10,
    /// The Dots11 spinner
    Dots11,
    /// The Dots8Bit spinner
    Dots8Bit,
    /// The Arrow spinner
    Arrow,
    /// The Arrow2 spinner
    Arrow2,
    /// The Arrow3 spinner
    Arrow3,
    /// The BouncingBar spinner
    BouncingBar,
    /// The BouncingBall spinner
    BouncingBall,
    /// The Smiley spinner
    Smiley,
    /// The Monkey spinner
    Monkey,
    /// The Hearts spinner
    Hearts,
    /// The Clock spinner
    Clock,
    /// The Earth spinner
    Earth,
    /// The Material spinner
    Material,
    /// The Moon spinner
    Moon,
    /// The Runner spinner
    Runner,
    /// The Pong spinner
    Pong,
    /// The Shark spinner
    Shark,
    /// The Dqpb spinner
    Dqpb,
    /// The Weather spinner
    Weather,
    /// The Christmas spinner
    Christmas,
    /// The Grenade spinner
    Grenade,
    /// The Point spinner
    Point,
    /// The Layer spinner
    Layer,
    /// The BetaWave spinner
    BetaWave,
    /// The FingerDance spinner
    FingerDance,
    /// The FistBump spinner
    FistBump,
    /// The SoccerHeader spinner
    SoccerHeader,
    /// The Mindblown spinner
    Mindblown,
    /// The Speaker spinner
    Speaker,
    /// The OrangePulse spinner
    OrangePulse,
    /// The BluePulse spinner
    BluePulse,
    /// The OrangeBluePulse spinner
    OrangeBluePulse,
    /// The TimeTravel spinner
    TimeTravel,
    /// The Aesthetic Spinner
    Aesthetic,
    /// The Pipe spinner
    Pipe,
    /// The Star spinner
    Star,
    /// The Star2 spinner
    Star2,
    /// The Flip spinner
    Flip,
    /// The Hamburger spinner
    Hamburger,
    /// The GrowVertical spinner
    GrowVertical,
    /// The GrowHorizontal spinner
    GrowHorizontal,
    /// The Balloon spinner
    Balloon,
    /// The Balloon2 spinner
    Balloon2,
    /// The Noise spinner
    Noise,
    /// The Bounce spinner
    Bounce,
    /// The BoxBounce spinner
    BoxBounce,
    /// The BoxBounce2 spinner
    BoxBounce2,
    /// The Triangle Spinner
    Triangle,
    /// The Arc spinner
    Arc,
    /// The Circle spinner
    Circle,
    /// The SquareCorners spinner
    SquareCorners,
    /// The CircleQuarters spinner
    CircleQuaters,
    /// The CircleHalves spinner
    CircleHalves,
    /// The Squish spinner
    Squish,
    /// The Toggle spinner
    Toggle,
    /// The Toggle2 spinner
    Toggle2,
    /// The Toggle3 spinner
    Toggle3,
    /// The Toggle4 spinner
    Toggle4,
    /// The Toggle5 spinner
    Toggle5,
    /// The Toggle6 spinner
    Toggle6,
    /// The Toggle7 spinner
    Toggle7,
    /// The Toggle8 spinner
    Toggle8,
    /// The Toggle9 spinner
    Toggle9,
    /// The Toggle10 spinner
    Toggle10,
    /// The Toggle11 spinner
    Toggle11,
    /// The Toggle12 spinner
    Toggle12,
    /// The Toggle13 spinner
    Toggle13,
    /// The Line spinner
    Line,
    /// The Line2 spinner
    Line2,
    /// The SimpleDots spinner
    SimpleDots,
    /// The SimpleDotsScrolling spinner
    SimpleDotsScrolling,
}

impl Spinners {
    /// Gets the frames for any given spinner
    pub fn get_frames(&self) -> Vec<&'static str> {
        SPINNER_MAP.get(&self.to_string()).unwrap().to_vec()
    }
}

lazy_static! {
    static ref SPINNER_MAP: HashMap<String, Vec<&'static str>> = {
        hashmap! {
            r"dots".into() => vec![
            r"⠋",
            r"⠙",
            r"⠹",
            r"⠸",
            r"⠼",
            r"⠴",
            r"⠦",
            r"⠧",
            r"⠇",
            r"⠏"
            ],
            "dots2".into()  =>  vec![
            r"⣾",
            r"⣽",
            r"⣻",
            r"⢿",
            r"⡿",
            r"⣟",
            r"⣯",
            r"⣷"
            ],
            "dots3".into()  =>  vec![
            r"⠋",
            r"⠙",
            r"⠚",
            r"⠞",
            r"⠖",
            r"⠦",
            r"⠴",
            r"⠲",
            r"⠳",
            r"⠓"
            ],
            "dots4".into()  =>  vec![
            r"⠄",
            r"⠆",
            r"⠇",
            r"⠋",
            r"⠙",
            r"⠸",
            r"⠰",
            r"⠠",
            r"⠰",
            r"⠸",
            r"⠙",
            r"⠋",
            r"⠇",
            r"⠆"
            ],
            "dots5".into()  =>  vec![
            r"⠋",
            r"⠙",
            r"⠚",
            r"⠒",
            r"⠂",
            r"⠂",
            r"⠒",
            r"⠲",
            r"⠴",
            r"⠦",
            r"⠖",
            r"⠒",
            r"⠐",
            r"⠐",
            r"⠒",
            r"⠓",
            r"⠋"
            ],
            "dots6".into()  =>  vec![
            r"⠁",
            r"⠉",
            r"⠙",
            r"⠚",
            r"⠒",
            r"⠂",
            r"⠂",
            r"⠒",
            r"⠲",
            r"⠴",
            r"⠤",
            r"⠄",
            r"⠄",
            r"⠤",
            r"⠴",
            r"⠲",
            r"⠒",
            r"⠂",
            r"⠂",
            r"⠒",
            r"⠚",
            r"⠙",
            r"⠉",
            r"⠁"
            ],
            "dots7".into()  =>  vec![
            r"⠈",
            r"⠉",
            r"⠋",
            r"⠓",
            r"⠒",
            r"⠐",
            r"⠐",
            r"⠒",
            r"⠖",
            r"⠦",
            r"⠤",
            r"⠠",
            r"⠠",
            r"⠤",
            r"⠦",
            r"⠖",
            r"⠒",
            r"⠐",
            r"⠐",
            r"⠒",
            r"⠓",
            r"⠋",
            r"⠉",
            r"⠈"
            ],
            "dots8".into()  =>  vec![
            r"⠁",
            r"⠁",
            r"⠉",
            r"⠙",
            r"⠚",
            r"⠒",
            r"⠂",
            r"⠂",
            r"⠒",
            r"⠲",
            r"⠴",
            r"⠤",
            r"⠄",
            r"⠄",
            r"⠤",
            r"⠠",
            r"⠠",
            r"⠤",
            r"⠦",
            r"⠖",
            r"⠒",
            r"⠐",
            r"⠐",
            r"⠒",
            r"⠓",
            r"⠋",
            r"⠉",
            r"⠈",
            r"⠈"
            ],
            "dots9".into()  =>  vec![
            r"⢹",
            r"⢺",
            r"⢼",
            r"⣸",
            r"⣇",
            r"⡧",
            r"⡗",
            r"⡏"
            ],
            "dots10".into()  =>  vec![
            r"⢄",
            r"⢂",
            r"⢁",
            r"⡁",
            r"⡈",
            r"⡐",
            r"⡠"
            ],
            "dots11".into()  =>  vec![
            r"⠁",
            r"⠂",
            r"⠄",
            r"⡀",
            r"⢀",
            r"⠠",
            r"⠐",
            r"⠈"
            ],
            "pipe".into()  =>  vec![
            r"┤",
            r"┘",
            r"┴",
            r"└",
            r"├",
            r"┌",
            r"┬",
            r"┐"
            ],
            "star".into()  =>  vec![
            r"✶",
            r"✸",
            r"✹",
            r"✺",
            r"✹",
            r"✷"
            ],
            "star2".into()  =>  vec![
            r"+",
            r"x",
            r"*"
            ],
            "flip".into()  =>  vec![
            r"_",
            r"_",
            r"_",
            r"-",
            r"`",
            r"`",
            r"'",
            r"´",
            r"-",
            r"_",
            r"_",
            r"_"
            ],
            "hamburger".into()  =>  vec![
            r"☱",
            r"☲",
            r"☴"
            ],
            "growVertical".into()  =>  vec![
            r"▁",
            r"▃",
            r"▄",
            r"▅",
            r"▆",
            r"▇",
            r"▆",
            r"▅",
            r"▄",
            r"▃"
            ],
            "growHorizontal".into()  =>  vec![
            r"▏",
            r"▎",
            r"▍",
            r"▌",
            r"▋",
            r"▊",
            r"▉",
            r"▊",
            r"▋",
            r"▌",
            r"▍",
            r"▎"
            ],
            "balloon".into()  =>  vec![
            r".",
            r"i",
            r"n",
            r"t",
            r"o",
            r"(",
            r")",
            r" ",
            r".",
            r"o",
            r"O",
            r"@",
            r"*",
            r" "
            ],
            "balloon2".into()  =>  vec![
            r".",
            r"o",
            r"O",
            r"°",
            r"O",
            r"o",
            r"."
            ],
            "noise".into()  =>  vec![
            r"▓",
            r"▒",
            r"░"
            ],
            "bounce".into()  =>  vec![
            r"⠁",
            r"⠂",
            r"⠄",
            r"⠂"
            ],
            "boxBounce".into()  =>  vec![
            r"▖",
            r"▘",
            r"▝",
            r"▗"
            ],
            "boxBounce2".into()  =>  vec![
            r"▌",
            r"▀",
            r"▐",
            r"▄"
            ],
            "triangle".into()  =>  vec![
            r"◢",
            r"◣",
            r"◤",
            r"◥"
            ],
            "arc".into()  =>  vec![
            r"◜",
            r"◠",
            r"◝",
            r"◞",
            r"◡",
            r"◟"
            ],
            "circle".into()  =>  vec![
            r"◡",
            r"⊙",
            r"◠"
            ],
            "squareCorners".into()  =>  vec![
            r"◰",
            r"◳",
            r"◲",
            r"◱"
            ],
            "circleQuaters".into()  =>  vec![
            r"◴",
            r"◷",
            r"◶",
            r"◵"
            ],
            "circleHalves".into()  =>  vec![
            r"◐",
            r"◓",
            r"◑",
            r"◒"
            ],
            "squish".into()  =>  vec![
            r"╫",
            r"╪"
            ],
            "toggle".into()  =>  vec![
            r"⊶",
            r"⊷"
            ],
            "toggle2".into()  =>  vec![
            r"▫",
            r"▪"
            ],
            "toggle3".into()  =>  vec![
            r"□",
            r"■"
            ],
            "toggle4".into()  =>  vec![
            r"■",
            r"□",
            r"▪",
            r"▫"
            ],
            "toggle5".into()  =>  vec![
            r"▮",
            r"▯"
            ],
            "toggle6".into()  =>  vec![
            r"ဝ",
            r"၀"
            ],
            "toggle7".into()  =>  vec![
            r"⦾",
            r"⦿"
            ],
            "toggle8".into()  =>  vec![
            r"◍",
            r"◌"
            ],
            "toggle9".into()  =>  vec![
            r"◉",
            r"◎"
            ],
            "toggle10".into()  =>  vec![
            r"㊂",
            r"㊀",
            r"㊁"
            ],
            "toggle11".into()  =>  vec![
            r"⧇",
            r"⧆"
            ],
            "toggle12".into()  =>  vec![
            r"☗",
            r"☖"
            ],
            "toggle13".into()  =>  vec![
            r"=",
            r"*",
            r"-"
            ],
            "arrow".into()  =>  vec![
            r"←",
            r"↖",
            r"↑",
            r"↗",
            r"→",
            r"↘",
            r"↓",
            r"↙"
            ],
            "dots8Bit".into() => vec![
            r"⠀",
            r"⠁",
            r"⠂",
            r"⠃",
            r"⠄",
            r"⠅",
            r"⠆",
            r"⠇",
            r"⡀",
            r"⡁",
            r"⡂",
            r"⡃",
            r"⡄",
            r"⡅",
            r"⡆",
            r"⡇",
            r"⠈",
            r"⠉",
            r"⠊",
            r"⠋",
            r"⠌",
            r"⠍",
            r"⠎",
            r"⠏",
            r"⡈",
            r"⡉",
            r"⡊",
            r"⡋",
            r"⡌",
            r"⡍",
            r"⡎",
            r"⡏",
            r"⠐",
            r"⠑",
            r"⠒",
            r"⠓",
            r"⠔",
            r"⠕",
            r"⠖",
            r"⠗",
            r"⡐",
            r"⡑",
            r"⡒",
            r"⡓",
            r"⡔",
            r"⡕",
            r"⡖",
            r"⡗",
            r"⠘",
            r"⠙",
            r"⠚",
            r"⠛",
            r"⠜",
            r"⠝",
            r"⠞",
            r"⠟",
            r"⡘",
            r"⡙",
            r"⡚",
            r"⡛",
            r"⡜",
            r"⡝",
            r"⡞",
            r"⡟",
            r"⠠",
            r"⠡",
            r"⠢",
            r"⠣",
            r"⠤",
            r"⠥",
            r"⠦",
            r"⠧",
            r"⡠",
            r"⡡",
            r"⡢",
            r"⡣",
            r"⡤",
            r"⡥",
            r"⡦",
            r"⡧",
            r"⠨",
            r"⠩",
            r"⠪",
            r"⠫",
            r"⠬",
            r"⠭",
            r"⠮",
            r"⠯",
            r"⡨",
            r"⡩",
            r"⡪",
            r"⡫",
            r"⡬",
            r"⡭",
            r"⡮",
            r"⡯",
            r"⠰",
            r"⠱",
            r"⠲",
            r"⠳",
            r"⠴",
            r"⠵",
            r"⠶",
            r"⠷",
            r"⡰",
            r"⡱",
            r"⡲",
            r"⡳",
            r"⡴",
            r"⡵",
            r"⡶",
            r"⡷",
            r"⠸",
            r"⠹",
            r"⠺",
            r"⠻",
            r"⠼",
            r"⠽",
            r"⠾",
            r"⠿",
            r"⡸",
            r"⡹",
            r"⡺",
            r"⡻",
            r"⡼",
            r"⡽",
            r"⡾",
            r"⡿",
            r"⢀",
            r"⢁",
            r"⢂",
            r"⢃",
            r"⢄",
            r"⢅",
            r"⢆",
            r"⢇",
            r"⣀",
            r"⣁",
            r"⣂",
            r"⣃",
            r"⣄",
            r"⣅",
            r"⣆",
            r"⣇",
            r"⢈",
            r"⢉",
            r"⢊",
            r"⢋",
            r"⢌",
            r"⢍",
            r"⢎",
            r"⢏",
            r"⣈",
            r"⣉",
            r"⣊",
            r"⣋",
            r"⣌",
            r"⣍",
            r"⣎",
            r"⣏",
            r"⢐",
            r"⢑",
            r"⢒",
            r"⢓",
            r"⢔",
            r"⢕",
            r"⢖",
            r"⢗",
            r"⣐",
            r"⣑",
            r"⣒",
            r"⣓",
            r"⣔",
            r"⣕",
            r"⣖",
            r"⣗",
            r"⢘",
            r"⢙",
            r"⢚",
            r"⢛",
            r"⢜",
            r"⢝",
            r"⢞",
            r"⢟",
            r"⣘",
            r"⣙",
            r"⣚",
            r"⣛",
            r"⣜",
            r"⣝",
            r"⣞",
            r"⣟",
            r"⢠",
            r"⢡",
            r"⢢",
            r"⢣",
            r"⢤",
            r"⢥",
            r"⢦",
            r"⢧",
            r"⣠",
            r"⣡",
            r"⣢",
            r"⣣",
            r"⣤",
            r"⣥",
            r"⣦",
            r"⣧",
            r"⢨",
            r"⢩",
            r"⢪",
            r"⢫",
            r"⢬",
            r"⢭",
            r"⢮",
            r"⢯",
            r"⣨",
            r"⣩",
            r"⣪",
            r"⣫",
            r"⣬",
            r"⣭",
            r"⣮",
            r"⣯",
            r"⢰",
            r"⢱",
            r"⢲",
            r"⢳",
            r"⢴",
            r"⢵",
            r"⢶",
            r"⢷",
            r"⣰",
            r"⣱",
            r"⣲",
            r"⣳",
            r"⣴",
            r"⣵",
            r"⣶",
            r"⣷",
            r"⢸",
            r"⢹",
            r"⢺",
            r"⢻",
            r"⢼",
            r"⢽",
            r"⢾",
            r"⢿",
            r"⣸",
            r"⣹",
            r"⣺",
            r"⣻",
            r"⣼",
            r"⣽",
            r"⣾",
            r"⣿"
            ],
            "line".into() => vec![
            r"-",
            r"\",
            r"|",
            r"/"
            ],
            "line2".into() => vec![
            r"⠂",
            r"-",
            r"–",
            r"—",
            r"–",
            r"-"
            ],
            "pipe".into() => vec![
            r"┤",
            r"┘",
            r"┴",
            r"└",
            r"├",
            r"┌",
            r"┬",
            r"┐"
            ],
            "simpleDots".into() => vec![
            r".  ",
            r".. ",
            r"...",
            r"   "
            ],
            "simpleDotsScrolling".into() => vec![
            r".  ",
            r".. ",
            r"...",
            r" ..",
            r"  .",
            r"   "
            ],
            "star".into() => vec![
            r"✶",
            r"✸",
            r"✹",
            r"✺",
            r"✹",
            r"✷"
            ],
            "star2".into() => vec![
            r"+",
            r"x",
            r"*"
            ],
            "flip".into() => vec![
            r"_",
            r"_",
            r"_",
            r"-",
            r"`",
            r"`",
            r"'",
            r"´",
            r"-",
            r"_",
            r"_",
            r"_"
            ],
            "hamburger".into() => vec![
            r"☱",
            r"☲",
            r"☴"
            ],
            "growVertical".into() => vec![
            r"▁",
            r"▃",
            r"▄",
            r"▅",
            r"▆",
            r"▇",
            r"▆",
            r"▅",
            r"▄",
            r"▃"
            ],
            "growHorizontal".into() => vec![
            r"▏",
            r"▎",
            r"▍",
            r"▌",
            r"▋",
            r"▊",
            r"▉",
            r"▊",
            r"▋",
            r"▌",
            r"▍",
            r"▎"
            ],
            "balloon".into() => vec![
            r" ",
            r".",
            r"o",
            r"O",
            r"@",
            r"*",
            r" "
            ],
            "balloon2".into() => vec![
            r".",
            r"o",
            r"O",
            r"°",
            r"O",
            r"o",
            r"."
            ],
            "noise".into() => vec![
            r"▓",
            r"▒",
            r"░"
            ],
            "bounce".into() => vec![
            r"⠁",
            r"⠂",
            r"⠄",
            r"⠂"
            ],
            "boxBounce".into() => vec![
            r"▖",
            r"▘",
            r"▝",
            r"▗"
            ],
            "boxBounce2".into() => vec![
            r"▌",
            r"▀",
            r"▐",
            r"▄"
            ],
            "triangle".into() => vec![
            r"◢",
            r"◣",
            r"◤",
            r"◥"
            ],
            "arc".into() => vec![
            r"◜",
            r"◠",
            r"◝",
            r"◞",
            r"◡",
            r"◟"
            ],
            "circle".into() => vec![
            r"◡",
            r"⊙",
            r"◠"
            ],
            "squareCorners".into() => vec![
            r"◰",
            r"◳",
            r"◲",
            r"◱"
            ],
            "circleQuarters".into() => vec![
            r"◴",
            r"◷",
            r"◶",
            r"◵"
            ],
            "circleHalves".into() => vec![
            r"◐",
            r"◓",
            r"◑",
            r"◒"
            ],
            "squish".into() => vec![
            r"╫",
            r"╪"
            ],
            "toggle".into() => vec![
            r"⊶",
            r"⊷"
            ],
            "toggle2".into() => vec![
            r"▫",
            r"▪"
            ],
            "toggle3".into() => vec![
            r"□",
            r"■"
            ],
            "toggle4".into() => vec![
            r"■",
            r"□",
            r"▪",
            r"▫"
            ],
            "toggle5".into() => vec![
            r"▮",
            r"▯"
            ],
            "toggle6".into() => vec![
            r"ဝ",
            r"၀"
            ],
            "toggle7".into() => vec![
            r"⦾",
            r"⦿"
            ],
            "toggle8".into() => vec![
            r"◍",
            r"◌"
            ],
            "toggle9".into() => vec![
            r"◉",
            r"◎"
            ],
            "toggle10".into() => vec![
            r"㊂",
            r"㊀",
            r"㊁"
            ],
            "toggle11".into() => vec![
            r"⧇",
            r"⧆"
            ],
            "toggle12".into() => vec![
            r"☗",
            r"☖"
            ],
            "toggle13".into() => vec![
            r"=",
            r"*",
            r"-"
            ],
            "arrow".into() => vec![
            r"←",
            r"↖",
            r"↑",
            r"↗",
            r"→",
            r"↘",
            r"↓",
            r"↙"
            ],
            "arrow2".into() => vec![
            r"⬆️ ",
            r"↗️ ",
            r"➡️ ",
            r"↘️ ",
            r"⬇️ ",
            r"↙️ ",
            r"⬅️ ",
            r"↖️ "
            ],
            "arrow3".into() => vec![
            r"▹▹▹▹▹",
            r"▸▹▹▹▹",
            r"▹▸▹▹▹",
            r"▹▹▸▹▹",
            r"▹▹▹▸▹",
            r"▹▹▹▹▸"
            ],
            "bouncingBar".into() => vec![
            r"[    ]",
            r"[=   ]",
            r"[==  ]",
            r"[=== ]",
            r"[ ===]",
            r"[  ==]",
            r"[   =]",
            r"[    ]",
            r"[   =]",
            r"[  ==]",
            r"[ ===]",
            r"[====]",
            r"[=== ]",
            r"[==  ]",
            r"[=   ]"
            ],
            "bouncingBall".into() => vec![
            r"( ●    )",
            r"(  ●   )",
            r"(   ●  )",
            r"(    ● )",
            r"(     ●)",
            r"(    ● )",
            r"(   ●  )",
            r"(  ●   )",
            r"( ●    )",
            r"(●     )"
            ],
            "smiley".into() => vec![
            r"😄 ",
            r"😝 "
            ],
            "monkey".into() => vec![
            r"🙈 ",
            r"🙈 ",
            r"🙉 ",
            r"🙊 "
            ],
            "hearts".into() => vec![
            r"💛 ",
            r"💙 ",
            r"💜 ",
            r"💚 ",
            r"❤️ "
            ],
            "clock".into() => vec![
            r"🕛 ",
            r"🕐 ",
            r"🕑 ",
            r"🕒 ",
            r"🕓 ",
            r"🕔 ",
            r"🕕 ",
            r"🕖 ",
            r"🕗 ",
            r"🕘 ",
            r"🕙 ",
            r"🕚 "
            ],
            "earth".into() => vec![
            r"🌍 ",
            r"🌎 ",
            r"🌏 "
            ],
            "material".into() => vec![
            r"█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"███▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"████▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"███████▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"████████▁▁▁▁▁▁▁▁▁▁▁▁",
            r"█████████▁▁▁▁▁▁▁▁▁▁▁",
            r"█████████▁▁▁▁▁▁▁▁▁▁▁",
            r"██████████▁▁▁▁▁▁▁▁▁▁",
            r"███████████▁▁▁▁▁▁▁▁▁",
            r"█████████████▁▁▁▁▁▁▁",
            r"██████████████▁▁▁▁▁▁",
            r"██████████████▁▁▁▁▁▁",
            r"▁██████████████▁▁▁▁▁",
            r"▁██████████████▁▁▁▁▁",
            r"▁██████████████▁▁▁▁▁",
            r"▁▁██████████████▁▁▁▁",
            r"▁▁▁██████████████▁▁▁",
            r"▁▁▁▁█████████████▁▁▁",
            r"▁▁▁▁██████████████▁▁",
            r"▁▁▁▁██████████████▁▁",
            r"▁▁▁▁▁██████████████▁",
            r"▁▁▁▁▁██████████████▁",
            r"▁▁▁▁▁██████████████▁",
            r"▁▁▁▁▁▁██████████████",
            r"▁▁▁▁▁▁██████████████",
            r"▁▁▁▁▁▁▁█████████████",
            r"▁▁▁▁▁▁▁█████████████",
            r"▁▁▁▁▁▁▁▁████████████",
            r"▁▁▁▁▁▁▁▁████████████",
            r"▁▁▁▁▁▁▁▁▁███████████",
            r"▁▁▁▁▁▁▁▁▁███████████",
            r"▁▁▁▁▁▁▁▁▁▁██████████",
            r"▁▁▁▁▁▁▁▁▁▁██████████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁████████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁██████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            r"█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            r"██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            r"██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            r"███▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            r"████▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            r"█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            r"█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            r"██████▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            r"████████▁▁▁▁▁▁▁▁▁▁▁▁",
            r"█████████▁▁▁▁▁▁▁▁▁▁▁",
            r"█████████▁▁▁▁▁▁▁▁▁▁▁",
            r"█████████▁▁▁▁▁▁▁▁▁▁▁",
            r"█████████▁▁▁▁▁▁▁▁▁▁▁",
            r"███████████▁▁▁▁▁▁▁▁▁",
            r"████████████▁▁▁▁▁▁▁▁",
            r"████████████▁▁▁▁▁▁▁▁",
            r"██████████████▁▁▁▁▁▁",
            r"██████████████▁▁▁▁▁▁",
            r"▁██████████████▁▁▁▁▁",
            r"▁██████████████▁▁▁▁▁",
            r"▁▁▁█████████████▁▁▁▁",
            r"▁▁▁▁▁████████████▁▁▁",
            r"▁▁▁▁▁████████████▁▁▁",
            r"▁▁▁▁▁▁███████████▁▁▁",
            r"▁▁▁▁▁▁▁▁█████████▁▁▁",
            r"▁▁▁▁▁▁▁▁█████████▁▁▁",
            r"▁▁▁▁▁▁▁▁▁█████████▁▁",
            r"▁▁▁▁▁▁▁▁▁█████████▁▁",
            r"▁▁▁▁▁▁▁▁▁▁█████████▁",
            r"▁▁▁▁▁▁▁▁▁▁▁████████▁",
            r"▁▁▁▁▁▁▁▁▁▁▁████████▁",
            r"▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            r"▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            r"▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁"
            ],
            "moon".into() => vec![
            r"🌑 ",
            r"🌒 ",
            r"🌓 ",
            r"🌔 ",
            r"🌕 ",
            r"🌖 ",
            r"🌗 ",
            r"🌘 "
            ],
            "runner".into() => vec![
            r"🚶 ",
            r"🏃 "
            ],
            "pong".into() => vec![
            r"▐⠂       ▌",
            r"▐⠈       ▌",
            r"▐ ⠂      ▌",
            r"▐ ⠠      ▌",
            r"▐  ⡀     ▌",
            r"▐  ⠠     ▌",
            r"▐   ⠂    ▌",
            r"▐   ⠈    ▌",
            r"▐    ⠂   ▌",
            r"▐    ⠠   ▌",
            r"▐     ⡀  ▌",
            r"▐     ⠠  ▌",
            r"▐      ⠂ ▌",
            r"▐      ⠈ ▌",
            r"▐       ⠂▌",
            r"▐       ⠠▌",
            r"▐       ⡀▌",
            r"▐      ⠠ ▌",
            r"▐      ⠂ ▌",
            r"▐     ⠈  ▌",
            r"▐     ⠂  ▌",
            r"▐    ⠠   ▌",
            r"▐    ⡀   ▌",
            r"▐   ⠠    ▌",
            r"▐   ⠂    ▌",
            r"▐  ⠈     ▌",
            r"▐  ⠂     ▌",
            r"▐ ⠠      ▌",
            r"▐ ⡀      ▌",
            r"▐⠠       ▌"
            ],
            "shark".into() => vec![
            r"▐|\____________▌",
            r"▐_|\___________▌",
            r"▐__|\__________▌",
            r"▐___|\_________▌",
            r"▐____|\________▌",
            r"▐_____|\_______▌",
            r"▐______|\______▌",
            r"▐_______|\_____▌",
            r"▐________|\____▌",
            r"▐_________|\___▌",
            r"▐__________|\__▌",
            r"▐___________|\_▌",
            r"▐____________|\▌",
            r"▐____________/|▌",
            r"▐___________/|_▌",
            r"▐__________/|__▌",
            r"▐_________/|___▌",
            r"▐________/|____▌",
            r"▐_______/|_____▌",
            r"▐______/|______▌",
            r"▐_____/|_______▌",
            r"▐____/|________▌",
            r"▐___/|_________▌",
            r"▐__/|__________▌",
            r"▐_/|___________▌",
            r"▐/|____________▌"
            ],
            "dqpb".into() => vec![
            r"d",
            r"q",
            r"p",
            r"b"
            ],
            "weather".into() => vec![
            r"☀️ ",
            r"☀️ ",
            r"☀️ ",
            r"🌤  ",
            r"⛅️ ",
            r"🌥  ",
            r"☁️ ",
            r"🌧  ",
            r"🌨  ",
            r"🌧  ",
            r"🌨  ",
            r"🌧  ",
            r"🌨  ",
            r"⛈  ",
            r"🌨  ",
            r"🌧  ",
            r"🌨  ",
            r"☁️ ",
            r"🌥  ",
            r"⛅️ ",
            r"🌤  ",
            r"☀️ ",
            r"☀️ "
            ],
            "christmas".into() => vec![
            r"🌲",
            r"🎄"
            ],
            "grenade".into() => vec![
            r"،    ",
            r"′    ",
            r" ´   ",
            r" ‾   ",
            r"  ⸌  ",
            r"  ⸊  ",
            r"  |  ",
            r"  ⁎  ",
            r"  ⁕  ",
            r" ෴ ",
            r"  ⁓  ",
            r"     ",
            r"     ",
            r"     "
            ],
            "point".into() => vec![
            r"∙∙∙",
            r"●∙∙",
            r"∙●∙",
            r"∙∙●",
            r"∙∙∙"
            ],
            "layer".into() => vec![
            r"-",
            r"=",
            r"≡"
            ],
            "betaWave".into() => vec![
            r"ρββββββ",
            r"βρβββββ",
            r"ββρββββ",
            r"βββρβββ",
            r"ββββρββ",
            r"βββββρβ",
            r"ββββββρ"
            ],
            "fingerDance".into() => vec![
            r"🤘 ",
            r"🤟 ",
            r"🖖 ",
            r"✋ ",
            r"🤚 ",
            r"👆 "
            ],
            "fistBump".into() => vec![
            r"🤜　　　　🤛 ",
            r"🤜　　　　🤛 ",
            r"🤜　　　　🤛 ",
            r"　🤜　　🤛　 ",
            r"　　🤜🤛　　 ",
            r"　🤜✨🤛　　 ",
            r"🤜　✨　🤛　 "
            ],
            "soccerHeader".into() => vec![
            r" 🧑⚽️       🧑 ",
            r"🧑  ⚽️      🧑 ",
            r"🧑   ⚽️     🧑 ",
            r"🧑    ⚽️    🧑 ",
            r"🧑     ⚽️   🧑 ",
            r"🧑      ⚽️  🧑 ",
            r"🧑       ⚽️🧑  ",
            r"🧑      ⚽️  🧑 ",
            r"🧑     ⚽️   🧑 ",
            r"🧑    ⚽️    🧑 ",
            r"🧑   ⚽️     🧑 ",
            r"🧑  ⚽️      🧑 "
            ],
            "mindblown".into() => vec![
            r"😐 ",
            r"😐 ",
            r"😮 ",
            r"😮 ",
            r"😦 ",
            r"😦 ",
            r"😧 ",
            r"😧 ",
            r"🤯 ",
            r"💥 ",
            r"✨ ",
            r"　 ",
            r"　 ",
            r"　 "
            ],
            "speaker".into() => vec![
            r"🔈 ",
            r"🔉 ",
            r"🔊 ",
            r"🔉 "
            ],
            "orangePulse".into() => vec![
            r"🔸 ",
            r"🔶 ",
            r"🟠 ",
            r"🟠 ",
            r"🔶 "
            ],
            "bluePulse".into() => vec![
            r"🔹 ",
            r"🔷 ",
            r"🔵 ",
            r"🔵 ",
            r"🔷 "
            ],
            "orangeBluePulse".into() => vec![
            r"🔸 ",
            r"🔶 ",
            r"🟠 ",
            r"🟠 ",
            r"🔶 ",
            r"🔹 ",
            r"🔷 ",
            r"🔵 ",
            r"🔵 ",
            r"🔷 "
            ],
            "timeTravel".into() => vec![
            r"🕛 ",
            r"🕚 ",
            r"🕙 ",
            r"🕘 ",
            r"🕗 ",
            r"🕖 ",
            r"🕕 ",
            r"🕔 ",
            r"🕓 ",
            r"🕒 ",
            r"🕑 ",
            r"🕐 "
            ],
            "aesthetic".into() => vec![
            r"▰▱▱▱▱▱▱",
            r"▰▰▱▱▱▱▱",
            r"▰▰▰▱▱▱▱",
            r"▰▰▰▰▱▱▱",
            r"▰▰▰▰▰▱▱",
            r"▰▰▰▰▰▰▱",
            r"▰▰▰▰▰▰▰",
            r"▰▱▱▱▱▱▱"
            ],
        }
    };
}
