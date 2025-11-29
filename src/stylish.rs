/*
 | PrettyTerm - Pretty Terminal Printers
 | =====================================
 | Description: Make your terminal interfaces prettier!
 | File:        src/stylish.rs
 | Repository:  https://github.com/CodeLibraty/prettyterm-rs
 |
 | SPDX-License-Identifier: GPL-3.0-or-later
 | CopyRight: © 2025 CodeLibraty Foundation
 */

use crate::colors::*;

/// Обработка стилевых тегов
pub fn process_style_tags(text: &str) -> String {
    let mut result = String::new();
    let mut style_stack: Vec<String> = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '<' {
            // Начало тега
            let mut is_closing = false;

            if let Some(&'/') = chars.peek() {
                is_closing = true;
                chars.next(); // Пропускаем /
            }

            let mut tag = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '>' {
                    chars.next(); // Пропускаем >
                    break;
                }
                tag.push(chars.next().unwrap());
            }

            if is_closing {
                // Закрывающий тег - восстанавливаем предыдущий стиль
                if !style_stack.is_empty() {
                    style_stack.pop();
                    result.push_str(RESET_COLOR);

                    // Восстанавливаем все активные стили
                    for style in &style_stack {
                        for s in style.split('|') {
                            result.push_str(get_style_code(s));
                        }
                    }
                }
            } else {
                // Открывающий тег - применяем стиль
                style_stack.push(tag.clone());
                for s in tag.split('|') {
                    result.push_str(get_style_code(s));
                }
            }
        } else {
            // Обычный символ
            result.push(ch);
        }
    }

    // Сбрасываем все стили в конце
    if !style_stack.is_empty() {
        result.push_str(RESET_COLOR);
    }

    result
}

/// Получение кода стиля по имени
fn get_style_code(style: &str) -> &'static str {
    match style.to_lowercase().as_str() {
        "red" => FG_RED,
        "green" => FG_GREEN,
        "blue" => FG_BLUE,
        "yellow" => FG_YELLOW,
        "magenta" => FG_MAGENTA,
        "cyan" => FG_CYAN,
        "white" => FG_WHITE,
        "black" => FG_BLACK,
        "bold" => STYLE_BOLD,
        "italic" => STYLE_ITALIC,
        "underline" => STYLE_UNDERLINE,
        "faded" => STYLE_FADED,
        "blinking" => STYLE_BLINKING,
        "crossedout" => STYLE_CROSSED_OUT,
        "bg-red" => BG_RED,
        "bg-green" => BG_GREEN,
        "bg-blue" => BG_BLUE,
        "bg-yellow" => BG_YELLOW,
        "bg-magenta" => BG_MAGENTA,
        "bg-cyan" => BG_CYAN,
        "bg-white" => BG_WHITE,
        "bg-black" => BG_BLACK,
        _ => "",
    }
}

/// Макрос для форматирования строк с поддержкой стилевых тегов
#[macro_export]
macro_rules! sty {
    ($fmt:expr) => {
        $crate::stylish::process_style_tags($fmt)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::stylish::process_style_tags(&format!($fmt, $($arg)*))
    };
}