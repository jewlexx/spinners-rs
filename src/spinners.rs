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
    // rustfmt is disabled as the formatting gave weird results
    #[rustfmt::skip]
    /// Gets the frames for any given spinner
    pub const fn get_frames(&self) -> &'static [&'static str] {
        match *self {
            Spinners::Dots => &[
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
            Spinners::Dots2 => &[
            r"⣾",
            r"⣽",
            r"⣻",
            r"⢿",
            r"⡿",
            r"⣟",
            r"⣯",
            r"⣷"
            ],
            Spinners::Dots3 => &[
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
            Spinners::Dots4 => &[
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
            Spinners::Dots5 => &[
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
            Spinners::Dots6 => &[
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
            Spinners::Dots7 => &[
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
            Spinners::Dots8 => &[
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
            Spinners::Dots9 => &[
            r"⢹",
            r"⢺",
            r"⢼",
            r"⣸",
            r"⣇",
            r"⡧",
            r"⡗",
            r"⡏"
            ],
            Spinners::Dots10 => &[
            r"⢄",
            r"⢂",
            r"⢁",
            r"⡁",
            r"⡈",
            r"⡐",
            r"⡠"
            ],
            Spinners::Dots11 => &[
            r"⠁",
            r"⠂",
            r"⠄",
            r"⡀",
            r"⢀",
            r"⠠",
            r"⠐",
            r"⠈"
            ],
            Spinners::Pipe => &[
            r"┤",
            r"┘",
            r"┴",
            r"└",
            r"├",
            r"┌",
            r"┬",
            r"┐"
            ],
            Spinners::Star => &[
            r"✶",
            r"✸",
            r"✹",
            r"✺",
            r"✹",
            r"✷"
            ],
            Spinners::Star2 => &[
            r"+",
            r"x",
            r"*"
            ],
            Spinners::Flip => &[
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
            Spinners::Hamburger => &[
            r"☱",
            r"☲",
            r"☴"
            ],
            Spinners::GrowVertical => &[
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
            Spinners::GrowHorizontal => &[
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
            Spinners::Balloon => &[
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
            Spinners::Balloon2 => &[
            r".",
            r"o",
            r"O",
            r"°",
            r"O",
            r"o",
            r"."
            ],
            Spinners::Noise => &[
            r"▓",
            r"▒",
            r"░"
            ],
            Spinners::Bounce => &[
            r"⠁",
            r"⠂",
            r"⠄",
            r"⠂"
            ],
            Spinners::BoxBounce => &[
            r"▖",
            r"▘",
            r"▝",
            r"▗"
            ],
            Spinners::BoxBounce2 => &[
            r"▌",
            r"▀",
            r"▐",
            r"▄"
            ],
            Spinners::Triangle => &[
            r"◢",
            r"◣",
            r"◤",
            r"◥"
            ],
            Spinners::Arc => &[
            r"◜",
            r"◠",
            r"◝",
            r"◞",
            r"◡",
            r"◟"
            ],
            Spinners::Circle => &[
            r"◡",
            r"⊙",
            r"◠"
            ],
            Spinners::CircleQuaters => &[
            r"◴",
            r"◷",
            r"◶",
            r"◵"
            ],
            Spinners::SquareCorners => &[
            r"◰",
            r"◳",
            r"◲",
            r"◱"
            ],
            Spinners::CircleHalves => &[
            r"◐",
            r"◓",
            r"◑",
            r"◒"
            ],
            Spinners::Squish => &[
            r"╫",
            r"╪"
            ],
            Spinners::Toggle => &[
            r"⊶",
            r"⊷"
            ],
            Spinners::Toggle2 => &[
            r"▫",
            r"▪"
            ],
            Spinners::Toggle3 => &[
            r"□",
            r"■"
            ],
            Spinners::Toggle4 => &[
            r"■",
            r"□",
            r"▪",
            r"▫"
            ],
            Spinners::Toggle5 => &[
            r"▮",
            r"▯"
            ],
            Spinners::Toggle6 => &[
            r"ဝ",
            r"၀"
            ],
            Spinners::Toggle7 => &[
            r"⦾",
            r"⦿"
            ],
            Spinners::Toggle8 => &[
            r"◍",
            r"◌"
            ],
            Spinners::Toggle9 => &[
            r"◉",
            r"◎"
            ],
            Spinners::Toggle10 => &[
            r"㊂",
            r"㊀",
            r"㊁"
            ],
            Spinners::Toggle11 => &[
            r"⧇",
            r"⧆"
            ],
            Spinners::Toggle12 => &[
            r"☗",
            r"☖"
            ],
            Spinners::Toggle13 => &[
            r"=",
            r"*",
            r"-"
            ],
            Spinners::Arrow => &[
            r"←",
            r"↖",
            r"↑",
            r"↗",
            r"→",
            r"↘",
            r"↓",
            r"↙"
            ],
            Spinners::Dots8Bit => &[
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
            Spinners::Line => &[
            r"-",
            r"",
            r"|",
            r"/"
            ],
            Spinners::Line2 => &[
            r"⠂",
            r"-",
            r"–",
            r"—",
            r"–",
            r"-"
            ],
            Spinners::SimpleDots => &[
            r".  ",
            r".. ",
            r"...",
            r"   "
            ],
            Spinners::SimpleDotsScrolling => &[
            r".  ",
            r".. ",
            r"...",
            r" ..",
            r"  .",
            r"   "
            ],
            Spinners::Arrow2 => &[
            r"⬆️ ",
            r"↗️ ",
            r"➡️ ",
            r"↘️ ",
            r"⬇️ ",
            r"↙️ ",
            r"⬅️ ",
            r"↖️ "
            ],
            Spinners::Arrow3 => &[
            r"▹▹▹▹▹",
            r"▸▹▹▹▹",
            r"▹▸▹▹▹",
            r"▹▹▸▹▹",
            r"▹▹▹▸▹",
            r"▹▹▹▹▸"
            ],
            Spinners::BouncingBar => &[
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
            Spinners::BouncingBall => &[
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
            Spinners::Smiley => &[
            r"😄 ",
            r"😝 "
            ],
            Spinners::Monkey => &[
            r"🙈 ",
            r"🙈 ",
            r"🙉 ",
            r"🙊 "
            ],
            Spinners::Hearts => &[
            r"💛 ",
            r"💙 ",
            r"💜 ",
            r"💚 ",
            r"❤️ "
            ],
            Spinners::Clock => &[
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
            Spinners::Earth => &[
            r"🌍 ",
            r"🌎 ",
            r"🌏 "
            ],
            Spinners::Material => &[
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
            Spinners::Moon => &[
            r"🌑 ",
            r"🌒 ",
            r"🌓 ",
            r"🌔 ",
            r"🌕 ",
            r"🌖 ",
            r"🌗 ",
            r"🌘 "
            ],
            Spinners::Runner => &[
            r"🚶 ",
            r"🏃 "
            ],
            Spinners::Pong => &[
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
            Spinners::Shark => &[
            r"▐|____________▌",
            r"▐_|___________▌",
            r"▐__|__________▌",
            r"▐___|_________▌",
            r"▐____|________▌",
            r"▐_____|_______▌",
            r"▐______|______▌",
            r"▐_______|_____▌",
            r"▐________|____▌",
            r"▐_________|___▌",
            r"▐__________|__▌",
            r"▐___________|_▌",
            r"▐____________|▌",
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
            Spinners::Dqpb => &[
            r"d",
            r"q",
            r"p",
            r"b"
            ],
            Spinners::Weather => &[
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
            Spinners::Christmas => &[
            r"🌲",
            r"🎄"
            ],
            Spinners::Grenade => &[
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
            Spinners::Point => &[
            r"∙∙∙",
            r"●∙∙",
            r"∙●∙",
            r"∙∙●",
            r"∙∙∙"
            ],
            Spinners::Layer => &[
            r"-",
            r"=",
            r"≡"
            ],
            Spinners::BetaWave => &[
            r"ρββββββ",
            r"βρβββββ",
            r"ββρββββ",
            r"βββρβββ",
            r"ββββρββ",
            r"βββββρβ",
            r"ββββββρ"
            ],
            Spinners::FingerDance => &[
            r"🤘 ",
            r"🤟 ",
            r"🖖 ",
            r"✋ ",
            r"🤚 ",
            r"👆 "
            ],
            Spinners::FistBump => &[
            r"🤜　　　　🤛 ",
            r"🤜　　　　🤛 ",
            r"🤜　　　　🤛 ",
            r"　🤜　　🤛　 ",
            r"　　🤜🤛　　 ",
            r"　🤜✨🤛　　 ",
            r"🤜　✨　🤛　 "
            ],
            Spinners::SoccerHeader => &[
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
            Spinners::Mindblown => &[
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
            Spinners::Speaker => &[
            r"🔈 ",
            r"🔉 ",
            r"🔊 ",
            r"🔉 "
            ],
            Spinners::OrangePulse => &[
            r"🔸 ",
            r"🔶 ",
            r"🟠 ",
            r"🟠 ",
            r"🔶 "
            ],
            Spinners::BluePulse => &[
            r"🔹 ",
            r"🔷 ",
            r"🔵 ",
            r"🔵 ",
            r"🔷 "
            ],
            Spinners::OrangeBluePulse => &[
            r"🔸 ",
            r"🔶 ",
            r"🟠 ",
            r"🟠 ",
            r"🔶 ",
            r"🔹 ",
            r"🔷 ",
            r"🔷 "
            ],
            Spinners::TimeTravel => &[
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
            Spinners::Aesthetic => &[
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
    }
}
