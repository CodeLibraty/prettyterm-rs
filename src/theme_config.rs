/*
 | PrettyTerm - Pretty Terminal Printers
 | =====================================
 | Description: Make your terminal interfaces prettier!
 | File:        src/theme_config.rs
 | Repository:  https://github.com/CodeLibraty/prettyterm-rs
 |
 | SPDX-License-Identifier: GPL-3.0-or-later
 | CopyRight: © 2025 CodeLibraty Foundation
 */

use crate::colors::*;

#[derive(Debug, Clone, Copy)]
pub enum TerminalColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl TerminalColors {
    pub fn as_str(&self) -> &'static str {
        match self {
            TerminalColors::Black => FG_BLACK,
            TerminalColors::Red => FG_RED,
            TerminalColors::Green => FG_GREEN,
            TerminalColors::Yellow => FG_YELLOW,
            TerminalColors::Blue => FG_BLUE,
            TerminalColors::Magenta => FG_MAGENTA,
            TerminalColors::Cyan => FG_CYAN,
            TerminalColors::White => FG_WHITE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub hint_color: TerminalColors,
    pub error_color: TerminalColors,
    pub success_color: TerminalColors,
    pub warning_color: TerminalColors,
}

impl ColorTheme {
    pub fn new(
        hint_color: TerminalColors,
        error_color: TerminalColors,
        success_color: TerminalColors,
        warning_color: TerminalColors,
    ) -> Self {
        Self {
            hint_color,
            error_color,
            success_color,
            warning_color,
        }
    }
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self {
            hint_color: TerminalColors::Blue,
            error_color: TerminalColors::Red,
            success_color: TerminalColors::Green,
            warning_color: TerminalColors::Yellow,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IconsTheme {
    pub hint_icon: String,
    pub error_icon: String,
    pub success_icon: String,
    pub warning_icon: String,
}

impl IconsTheme {
    pub fn new(
        hint_icon: String,
        error_icon: String,
        success_icon: String,
        warning_icon: String,
    ) -> Self {
        Self {
            hint_icon,
            error_icon,
            success_icon,
            warning_icon,
        }
    }
}

impl Default for IconsTheme {
    fn default() -> Self {
        Self {
            hint_icon: "🛈".to_string(),
            error_icon: "✗".to_string(),
            success_icon: "✓".to_string(),
            warning_icon: "⚠".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub color_theme: ColorTheme,
    pub icons_theme: IconsTheme,
    pub terminal_size: (usize, usize),
}

impl DisplayConfig {
    pub fn new(
        color_theme: ColorTheme,
        icons_theme: IconsTheme,
        terminal_size: (usize, usize),
    ) -> Self {
        Self {
            color_theme,
            icons_theme,
            terminal_size,
        }
    }

    /// Получить размер терминала (через nix на Unix-системах)
    #[cfg(unix)]
    pub fn get_terminal_size() -> (usize, usize) {
        use nix::ioctl_read_bad;
        use std::os::unix::io::AsRawFd;

        #[repr(C)]
        struct Winsize {
            ws_row: u16,
            ws_col: u16,
            ws_xpixel: u16,
            ws_ypixel: u16,
        }

        ioctl_read_bad!(get_winsize, nix::libc::TIOCGWINSZ, Winsize);

        let mut winsize = Winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        unsafe {
            if get_winsize(std::io::stdout().as_raw_fd(), &mut winsize).is_ok() {
                (winsize.ws_col as usize, winsize.ws_row as usize)
            } else {
                (80, 24) // Значение по умолчанию
            }
        }
    }

    /// Получить размер терминала (fallback для других систем)
    #[cfg(not(unix))]
    pub fn get_terminal_size() -> (usize, usize) {
        (80, 24) // Значение по умолчанию
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            color_theme: ColorTheme::default(),
            icons_theme: IconsTheme::default(),
            terminal_size: Self::get_terminal_size(),
        }
    }
}