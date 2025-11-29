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

use crate::common_types::Status;
use crate::theme_config::DisplayConfig;

#[derive(Debug, Clone, Copy)]
pub enum BranchStyle {
    /// Стандартный: веточный
    Unicode,
    /// С отступами вместо веток
    Indent,
}

impl BranchStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            BranchStyle::Unicode => "│  ",
            BranchStyle::Indent => "   ",
        }
    }
}

#[derive(Clone)]
pub struct Branch {
    pub branch_name: String,
    pub branch_message: String,
    pub branch_indent_level: usize,
    pub branch_display_config: DisplayConfig,
    pub branch_style: BranchStyle,
}

impl Branch {
    /// Создание нового корневого бранча
    pub fn new(display_config: DisplayConfig, style: BranchStyle) -> Self {
        Self {
            branch_name: String::new(),
            branch_message: String::new(),
            branch_indent_level: 0,
            branch_display_config: display_config,
            branch_style: style,
        }
    }

    /// Войти в новый бранч
    pub fn enter_branch(&self, name: &str) -> Self {
        let symbol = if matches!(self.branch_style, BranchStyle::Indent) {
            "╰"
        } else {
            "├"
        };

        println!("{}{}─ {}", self.format_indent(), symbol, name);

        Self {
            branch_name: name.to_string(),
            branch_message: String::new(),
            branch_indent_level: self.branch_indent_level + 1,
            branch_display_config: self.branch_display_config.clone(),
            branch_style: self.branch_style,
        }
    }

    /// Выйти из бранча
    pub fn leave_branch(&self, text: &str, _status: Status) -> String {
        format!("{}╰─ {}", self.format_indent(), text)
    }

    /// Форматировать отступ
    pub fn format_indent(&self) -> String {
        self.branch_style.as_str().repeat(self.branch_indent_level)
    }

    /// Форматировать строку бранча
    pub fn format_branch_line(&self, text: &str, prefix: &str) -> String {
        format!("{}{}{}", self.format_indent(), prefix, text)
    }

    /// Форматировать строку таблицы
    pub fn format_table_line(&self, line: &str) -> String {
        let visual_len = visual_len(line);
        let spaces_needed = self
            .branch_display_config
            .terminal_size
            .0
            .saturating_sub(visual_len + 4 + self.branch_indent_level * 3);

        format!(
            "{}│ {} {}│",
            self.format_indent(),
            line,
            " ".repeat(spaces_needed)
        )
    }

    /// Форматировать многострочный текст таблицы
    pub fn format_table_multi_line(&self, lines: &str) -> String {
        let max_line_width = self
            .branch_display_config
            .terminal_size
            .0
            .saturating_sub(3 + self.branch_indent_level * 3);
        let mut result = Vec::new();

        for orig_line in lines.lines() {
            let line = orig_line.trim();
            let mut pos = 0;
            let len = line.len();

            while pos < len {
                let end = std::cmp::min(pos + max_line_width - 1, len);

                // Ищем последний пробел в текущем диапазоне
                let break_pos = line[pos..=end]
                    .rfind(' ')
                    .map(|i| pos + i);

                let (chunk, next_pos) = if let Some(bp) = break_pos {
                    (&line[pos..bp], bp + 1)
                } else {
                    (&line[pos..=end], end + 1)
                };

                result.push(self.format_table_line(chunk));
                pos = next_pos;
            }
        }

        result.join("\n")
    }

    /// Форматировать строку кода
    pub fn format_code_line(&self, line_num: usize, code_line: &str, line_num_indent: usize) -> String {
        let line_num_str = line_num.to_string();
        let indent_len = line_num_indent.saturating_sub(line_num_str.len());
        format!("{}{}| {}", " ".repeat(indent_len), line_num_str, code_line)
    }

    /// Форматировать многострочный код в таблице
    pub fn format_table_code_multi_line(&self, line_num_first: usize, code_snippet: &str) -> String {
        let code_lines: Vec<&str> = code_snippet.lines().collect();
        let line_num_indent = (line_num_first + code_lines.len()).to_string().len();
        let mut result = Vec::new();

        for (i, code_line) in code_lines.iter().enumerate() {
            let line_num = line_num_first + i;
            let formatted = self.format_code_line(line_num, code_line, line_num_indent);
            let visual_width = visual_len(&formatted);
            
            let spaces_needed = self
                .branch_display_config
                .terminal_size
                .0
                .saturating_sub(self.branch_indent_level * 3 + visual_width + 6);

            result.push(format!(
                "{}│ {}{} │",
                self.format_indent(),
                formatted,
                " ".repeat(spaces_needed)
            ));
        }

        result.join("\n")
    }

    /// Форматировать заголовок таблицы
    pub fn format_table_header(&self, text: &str) -> String {
        let visual_len = visual_len(text);
        let dashes_needed = self
            .branch_display_config
            .terminal_size
            .0
            .saturating_sub(visual_len + 5 + self.branch_indent_level * 3);

        format!(
            "{}{} {}╮",
            self.format_branch_line(text, "├─ "),
            "─".repeat(dashes_needed),
            ""
        )
    }

    /// Форматировать подвал таблицы
    pub fn format_table_footer(&self) -> String {
        let dashes_needed = self
            .branch_display_config
            .terminal_size
            .0
            .saturating_sub(3 + self.branch_indent_level * 3);

        format!(
            "{}{}╯",
            self.format_branch_line("", "├─"),
            "─".repeat(dashes_needed)
        )
    }
}

/// Получить визуальную ширину строки в терминале
pub fn visual_len(s: &str) -> usize {
    let mut result = 0;
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if let Some(&'[') = chars.peek() {
                chars.next(); // Пропускаем [
                while let Some(&next_ch) = chars.peek() {
                    chars.next();
                    if next_ch == 'm' {
                        break;
                    }
                }
                continue;
            }
        }
        result += 1;
    }

    result
}