/*
 | PrettyTerm - Pretty Terminal Printers
 | =====================================
 | Description: Make your terminal interfaces prettier!
 | File:        src/tree_printer.rs
 | Repository:  https://github.com/CodeLibraty/prettyterm-rs
 |
 | SPDX-License-Identifier: GPL-3.0-or-later
 | CopyRight: © 2025 CodeLibraty Foundation
 */

/// Цвета текста
pub const FG_BLACK: &str = "\x1b[30m";
pub const FG_RED: &str = "\x1b[31m";
pub const FG_GREEN: &str = "\x1b[32m";
pub const FG_YELLOW: &str = "\x1b[33m";
pub const FG_BLUE: &str = "\x1b[34m";
pub const FG_MAGENTA: &str = "\x1b[35m";
pub const FG_CYAN: &str = "\x1b[36m";
pub const FG_WHITE: &str = "\x1b[37m";

/// Цвета фона
pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE: &str = "\x1b[44m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_CYAN: &str = "\x1b[46m";
pub const BG_WHITE: &str = "\x1b[47m";

/// Стили текста
pub const STYLE_BOLD: &str = "\x1b[1m";
pub const STYLE_FADED: &str = "\x1b[2m";
pub const STYLE_ITALIC: &str = "\x1b[3m";
pub const STYLE_UNDERLINE: &str = "\x1b[4m";
pub const STYLE_BLINKING: &str = "\x1b[5m";
pub const STYLE_CROSSED_OUT: &str = "\x1b[9m";

/// Сбрасывает все цвета и стили
pub const RESET_COLOR: &str = "\x1b[0m";