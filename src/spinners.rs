use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

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
