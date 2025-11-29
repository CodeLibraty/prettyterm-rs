/*
 | PrettyTerm - Pretty Terminal Printers
 | =====================================
 | Description: Make your terminal interfaces prettier!
 | File:        src/common_types.rs
 | Repository:  https://github.com/CodeLibraty/prettyterm-rs
 |
 | SPDX-License-Identifier: GPL-3.0-or-later
 | CopyRight: © 2025 CodeLibraty Foundation
 */

use std::fmt;

/// Статусы, показывают результат выполнения
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok,
    Error,
    Fatal,
    Info,
    Warn,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Status::Ok => "Ok",
            Status::Error => "Error",
            Status::Fatal => "Fatal",
            Status::Info => "Info",
            Status::Warn => "Warning",
        };
        write!(f, "{}", status_str)
    }
}