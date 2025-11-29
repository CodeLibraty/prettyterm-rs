/*
 | PrettyTerm - Pretty Terminal Printers
 | =====================================
 | Description: Make your terminal interfaces prettier!
 | File:        src/logger.rs
 | Repository:  https://github.com/CodeLibraty/prettyterm-rs
 |
 | SPDX-License-Identifier: GPL-3.0-or-later
 | CopyRight: © 2025 CodeLibraty Foundation
 */

use chrono::{Local, Timelike};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::common_types::Status;

/// Время регистрации лога
#[derive(Debug, Clone)]
pub struct LogTime {
    pub hour: u32,
    pub minute: u32,
    pub seconds: u32,
}

impl LogTime {
    /// Создание нового времени лога с ручным указанием
    pub fn new(hour: u32, minute: u32, seconds: u32) -> Self {
        Self {
            hour,
            minute,
            seconds,
        }
    }

    /// Создание нового времени лога с автоматическим определением
    pub fn now() -> Self {
        let time = Local::now();
        Self {
            hour: time.hour(),
            minute: time.minute(),
            seconds: time.second(),
        }
    }

    /// Форматирование времени в строку
    pub fn format(&self) -> String {
        format!("{}:{}:{}", self.hour, self.minute, self.seconds)
    }
}

/// Компонент
#[derive(Debug, Clone)]
pub struct Component {
    pub file_name: String,
    pub func_name: String,
    pub dir_path: String,
}

impl Component {
    /// Создание нового компонента
    pub fn new(file_name: String, func_name: String, dir_path: String) -> Self {
        Self {
            file_name,
            func_name,
            dir_path,
        }
    }
}

/// Одно сообщение лога
#[derive(Debug, Clone)]
pub struct Log {
    pub status: Status,
    pub message: String,
    pub component: Component,
    pub time: LogTime,
}

impl Log {
    /// Создание нового лога
    pub fn new(status: Status, message: String, component: Component, time: LogTime) -> Self {
        Self {
            status,
            message,
            component,
            time,
        }
    }

    /// Форматирование лога в строку
    pub fn format(&self, style: &LoggerPrintStyle) -> String {
        match style {
            LoggerPrintStyle::Tiny => {
                format!(
                    "{}: {} | from {}-func:{}, time is {}",
                    self.status,
                    self.message,
                    self.component.file_name,
                    self.component.func_name,
                    self.time.format()
                )
            }
            LoggerPrintStyle::Flat => {
                format!(
                    "{}: {} | file {} | time {}",
                    self.status,
                    self.message,
                    self.component.file_name,
                    self.time.format()
                )
            }
            LoggerPrintStyle::Full => {
                format!(
                    "[{}|{}][{}/{}-{}]: {}",
                    self.status,
                    self.time.format(),
                    self.component.dir_path,
                    self.component.file_name,
                    self.component.func_name,
                    self.message
                )
            }
        }
    }
}

/// Стиль вывода логгера
#[derive(Debug, Clone, Copy)]
pub enum LoggerPrintStyle {
    /// Стиль без символов, больше слов
    Flat,
    /// Компактный стиль с упрощённым выводом
    Tiny,
    /// Полный вывод всей информации с чётким форматированием
    Full,
}

/// Основной класс логгера
pub struct Logger {
    pub logs: Vec<Log>,
    pub creation_time: LogTime,
    pub destruction_time: Option<LogTime>,
    pub printable_in_terminal: bool,
    pub style: LoggerPrintStyle,
}

impl Logger {
    /// Инициализация нового экземпляра логгера
    pub fn new(creation_time: LogTime, printable_in_terminal: bool) -> Self {
        Self {
            logs: Vec::new(),
            creation_time,
            destruction_time: None,
            printable_in_terminal,
            style: LoggerPrintStyle::Tiny,
        }
    }

    /// Добавление лога в логгер
    pub fn add_log(
        &mut self,
        message: String,
        component: Component,
        status: Status,
        time: Option<LogTime>,
    ) -> Option<String> {
        let time = time.unwrap_or_else(LogTime::now);
        let log = Log::new(status, message, component, time);

        let formatted = if self.printable_in_terminal {
            Some(log.format(&self.style))
        } else {
            None
        };

        self.logs.push(log);
        formatted
    }

    /// Уничтожение экземпляра логгера
    pub fn destroy(
        mut self,
        file_for_logs: &str,
        write_to_file: bool,
        print_everything_now: bool,
    ) -> io::Result<()> {
        self.destruction_time = Some(LogTime::now());

        let formatted_logs: Vec<String> = self
            .logs
            .iter()
            .map(|log| log.format(&self.style))
            .collect();

        if print_everything_now {
            for log in &formatted_logs {
                println!("{}", log);
            }
        }

        if write_to_file {
            // Создаём файл если не существует
            if !Path::new(file_for_logs).exists() {
                File::create(file_for_logs)?;
            }

            let mut file = File::create(file_for_logs)?;
            file.write_all(formatted_logs.join("\n").as_bytes())?;
        }

        Ok(())
    }
}