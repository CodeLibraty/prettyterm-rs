//! Integration tests for PrettyTerm
//!
//! Run with: cargo test
//! Run with output: cargo test -- --nocapture

use prettyterm::*;
use std::fs;

#[test]
fn test_colors_constants() {
    // Проверяем что все цветовые константы валидны
    assert_eq!(FG_RED, "\x1b[31m");
    assert_eq!(FG_GREEN, "\x1b[32m");
    assert_eq!(BG_BLUE, "\x1b[44m");
    assert_eq!(RESET_COLOR, "\x1b[0m");
    assert_eq!(STYLE_BOLD, "\x1b[1m");
}

#[test]
fn test_status_display() {
    assert_eq!(Status::Ok.to_string(), "Ok");
    assert_eq!(Status::Error.to_string(), "Error");
    assert_eq!(Status::Fatal.to_string(), "Fatal");
    assert_eq!(Status::Info.to_string(), "Info");
    assert_eq!(Status::Warn.to_string(), "Warning");
}

#[test]
fn test_log_time_creation() {
    let time = LogTime::new(14, 30, 45);
    assert_eq!(time.hour, 14);
    assert_eq!(time.minute, 30);
    assert_eq!(time.seconds, 45);
    assert_eq!(time.format(), "14:30:45");
}

#[test]
fn test_log_time_now() {
    let time = LogTime::now();
    // Проверяем что время валидное
    assert!(time.hour < 24);
    assert!(time.minute < 60);
    assert!(time.seconds < 60);
}

#[test]
fn test_component_creation() {
    let component = Component::new(
        "test.rs".to_string(),
        "test_func".to_string(),
        "/path/to".to_string(),
    );
    assert_eq!(component.file_name, "test.rs");
    assert_eq!(component.func_name, "test_func");
    assert_eq!(component.dir_path, "/path/to");
}

#[test]
fn test_log_format_tiny() {
    let time = LogTime::new(10, 20, 30);
    let component = Component::new("file.rs".to_string(), "func".to_string(), "/src".to_string());
    let log = Log::new(Status::Ok, "test message".to_string(), component, time);

    let formatted = log.format(&LoggerPrintStyle::Tiny);
    assert!(formatted.contains("Ok"));
    assert!(formatted.contains("test message"));
    assert!(formatted.contains("file.rs"));
    assert!(formatted.contains("func"));
    assert!(formatted.contains("10:20:30"));
}

#[test]
fn test_log_format_flat() {
    let time = LogTime::new(10, 20, 30);
    let component = Component::new("file.rs".to_string(), "func".to_string(), "/src".to_string());
    let log = Log::new(Status::Error, "error occurred".to_string(), component, time);

    let formatted = log.format(&LoggerPrintStyle::Flat);
    assert!(formatted.contains("Error"));
    assert!(formatted.contains("error occurred"));
    assert!(formatted.contains("file.rs"));
    assert!(formatted.contains("10:20:30"));
}

#[test]
fn test_log_format_full() {
    let time = LogTime::new(10, 20, 30);
    let component = Component::new("file.rs".to_string(), "func".to_string(), "/src".to_string());
    let log = Log::new(Status::Warn, "warning".to_string(), component, time);

    let formatted = log.format(&LoggerPrintStyle::Full);
    assert!(formatted.contains("[Warning|10:20:30]"));
    assert!(formatted.contains("[/src/file.rs-func]"));
    assert!(formatted.contains("warning"));
}

#[test]
fn test_logger_creation() {
    let logger = Logger::new(LogTime::now(), true);
    assert_eq!(logger.logs.len(), 0);
    assert!(logger.printable_in_terminal);
    assert!(logger.destruction_time.is_none());
}

#[test]
fn test_logger_add_log() {
    let mut logger = Logger::new(LogTime::now(), true);
    let component = Component::new("test.rs".to_string(), "main".to_string(), "/src".to_string());

    let result = logger.add_log(
        "test log".to_string(),
        component,
        Status::Info,
        None,
    );

    assert_eq!(logger.logs.len(), 1);
    assert!(result.is_some());
    assert!(result.unwrap().contains("test log"));
}

#[test]
fn test_logger_multiple_logs() {
    let mut logger = Logger::new(LogTime::now(), false);
    let component = Component::new("test.rs".to_string(), "main".to_string(), "/src".to_string());

    for i in 0..5 {
        logger.add_log(
            format!("log {}", i),
            component.clone(),
            Status::Ok,
            None,
        );
    }

    assert_eq!(logger.logs.len(), 5);
}

#[test]
fn test_logger_destroy_without_file() {
    let mut logger = Logger::new(LogTime::now(), false);
    let component = Component::new("test.rs".to_string(), "main".to_string(), "/src".to_string());

    logger.add_log("test".to_string(), component, Status::Ok, None);

    // Не пишем в файл
    let result = logger.destroy("/tmp/test_prettyterm.log", false, false);
    assert!(result.is_ok());
}

#[test]
fn test_logger_destroy_with_file() {
    let mut logger = Logger::new(LogTime::now(), false);
    let component = Component::new("test.rs".to_string(), "main".to_string(), "/src".to_string());

    logger.add_log("test log entry".to_string(), component, Status::Ok, None);

    let log_file = "/tmp/test_prettyterm_write.log";
    let result = logger.destroy(log_file, true, false);
    assert!(result.is_ok());

    // Проверяем что файл создан и содержит логи
    let content = fs::read_to_string(log_file).unwrap();
    assert!(content.contains("test log entry"));

    // Очистка
    let _ = fs::remove_file(log_file);
}

#[test]
fn test_stylish_simple_tag() {
    let result = process_style_tags("<red>Hello</red>");
    assert!(result.contains("\x1b[31m")); // FG_RED
    assert!(result.contains("Hello"));
    assert!(result.contains("\x1b[0m")); // RESET
}

#[test]
fn test_stylish_multiple_tags() {
    let result = process_style_tags("<red>Red</red> and <blue>Blue</blue>");
    assert!(result.contains("\x1b[31m"));
    assert!(result.contains("\x1b[34m"));
    assert!(result.contains("Red"));
    assert!(result.contains("Blue"));
}

#[test]
fn test_stylish_combined_styles() {
    let result = process_style_tags("<red|bold>Bold Red</red|bold>");
    assert!(result.contains("\x1b[31m")); // RED
    assert!(result.contains("\x1b[1m"));  // BOLD
}

#[test]
fn test_stylish_nested_tags() {
    let result = process_style_tags("<red>Red <bold>Bold</bold> Normal</red>");
    assert!(result.contains("\x1b[31m"));
    assert!(result.contains("\x1b[1m"));
}

#[test]
fn test_stylish_background() {
    let result = process_style_tags("<bg-green>Green BG</bg-green>");
    assert!(result.contains("\x1b[42m")); // BG_GREEN
}

#[test]
fn test_sty_macro() {
    let name = "Test";
    let result = sty!("Hello, <red>{}</red>!", name);
    assert!(result.contains("\x1b[31m"));
    assert!(result.contains("Test"));
}

#[test]
fn test_visual_len_plain_text() {
    assert_eq!(visual_len("Hello"), 5);
    assert_eq!(visual_len("Привет"), 6);
}

#[test]
fn test_visual_len_with_ansi() {
    let text = format!("{}Hello{}", FG_RED, RESET_COLOR);
    assert_eq!(visual_len(&text), 5); // Только "Hello"
}

#[test]
fn test_branch_creation() {
    let branch = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    assert_eq!(branch.branch_indent_level, 0);
    assert_eq!(branch.branch_name, "");
}

#[test]
fn test_branch_enter() {
    let root = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    let child = root.enter_branch("Child");
    assert_eq!(child.branch_name, "Child");
    assert_eq!(child.branch_indent_level, 1);
}

#[test]
fn test_branch_format_indent() {
    let root = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    assert_eq!(root.format_indent(), "");

    let child = root.enter_branch("Child");
    assert_eq!(child.format_indent(), "│  ");
}

#[test]
fn test_branch_format_line() {
    let branch = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    let result = branch.format_branch_line("Test", "├─ ");
    assert!(result.contains("├─ Test"));
}

#[test]
fn test_branch_leave() {
    let branch = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    let result = branch.leave_branch("Done", Status::Ok);
    assert!(result.contains("╰─ Done"));
}

#[test]
fn test_branch_table_header() {
    let branch = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    let result = branch.format_table_header("Header");
    assert!(result.contains("Header"));
    assert!(result.contains("╮"));
    assert!(result.contains("─"));
}

#[test]
fn test_branch_table_footer() {
    let branch = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    let result = branch.format_table_footer();
    assert!(result.contains("╯"));
    assert!(result.contains("─"));
}

#[test]
fn test_branch_code_line() {
    let branch = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    let result = branch.format_code_line(1, "let x = 5;", 3);
    assert!(result.contains("  1| let x = 5;"));
}

#[test]
fn test_color_theme_default() {
    let theme = ColorTheme::default();
    assert_eq!(theme.hint_color.as_str(), FG_BLUE);
    assert_eq!(theme.error_color.as_str(), FG_RED);
    assert_eq!(theme.success_color.as_str(), FG_GREEN);
    assert_eq!(theme.warning_color.as_str(), FG_YELLOW);
}

#[test]
fn test_icons_theme_default() {
    let theme = IconsTheme::default();
    assert_eq!(theme.hint_icon, "🛈");
    assert_eq!(theme.error_icon, "✗");
    assert_eq!(theme.success_icon, "✓");
    assert_eq!(theme.warning_icon, "⚠");
}

#[test]
fn test_display_config_default() {
    let config = DisplayConfig::default();
    assert_eq!(config.color_theme.hint_color.as_str(), FG_BLUE);
    assert!(config.terminal_size.0 > 0);
    assert!(config.terminal_size.1 > 0);
}

#[test]
fn test_terminal_colors_enum() {
    assert_eq!(TerminalColors::Red.as_str(), FG_RED);
    assert_eq!(TerminalColors::Green.as_str(), FG_GREEN);
    assert_eq!(TerminalColors::Blue.as_str(), FG_BLUE);
}

// Интеграционный тест: полный сценарий использования
#[test]
fn test_full_scenario_compiler_output() {
    let mut logger = Logger::new(LogTime::now(), false);
    logger.style = LoggerPrintStyle::Full;

    let root = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);

    // Этап лексического анализа
    let lex_component = Component::new(
        "lexer.rs".to_string(),
        "tokenize".to_string(),
        "src/frontend".to_string(),
    );
    logger.add_log("Starting tokenization".to_string(), lex_component.clone(), Status::Info, None);
    let lex = root.enter_branch("Lexical analysis");
    logger.add_log("Tokenization complete".to_string(), lex_component, Status::Ok, None);

    // Этап синтаксического анализа
    let syntax_component = Component::new(
        "parser.rs".to_string(),
        "parse".to_string(),
        "src/frontend".to_string(),
    );
    logger.add_log("Building AST".to_string(), syntax_component.clone(), Status::Info, None);
    let syntax = lex.enter_branch("Syntax analysis");
    logger.add_log("AST built successfully".to_string(), syntax_component, Status::Ok, None);

    // Проверяем что все логи записались
    assert!(logger.logs.len() >= 4);

    // Проверяем форматирование
    let formatted = syntax.format_branch_line("Check complete", "├─ ");
    assert!(formatted.contains("Check complete"));

    let log_file = "/tmp/test_compiler_scenario.log";
    let _ = logger.destroy(log_file, true, false);

    // Проверяем файл
    let content = fs::read_to_string(log_file).unwrap();
    assert!(content.contains("tokenization"));
    assert!(content.contains("AST"));

    let _ = fs::remove_file(log_file);
}

#[test]
fn test_full_scenario_stylish_formatting() {
    let output = sty!("<green|bold>SUCCESS:</green|bold> All tests passed!");
    assert!(output.contains("\x1b[32m")); // GREEN
    assert!(output.contains("\x1b[1m"));  // BOLD
    assert!(output.contains("SUCCESS"));

    let name = "PrettyTerm";
    let version = "0.1.0";
    let output = sty!("<cyan>{}</cyan> v<yellow>{}</yellow>", name, version);
    assert!(output.contains("PrettyTerm"));
    assert!(output.contains("0.1.0"));
}

// Benchmark-подобный тест для производительности
#[test]
fn test_performance_many_logs() {
    let mut logger = Logger::new(LogTime::now(), false);
    let component = Component::new("test.rs".to_string(), "bench".to_string(), "/bench".to_string());

    let start = std::time::Instant::now();

    for i in 0..1000 {
        logger.add_log(
            format!("Log entry {}", i),
            component.clone(),
            Status::Info,
            None,
        );
    }

    let duration = start.elapsed();

    assert_eq!(logger.logs.len(), 1000);
    // Проверяем что 1000 логов создаются быстро (< 100ms)
    assert!(duration.as_millis() < 100, "Logging too slow: {:?}", duration);
}

#[test]
fn test_performance_stylish_processing() {
    let text = "<red>R</red><green>G</green><blue>B</blue>".repeat(100);

    let start = std::time::Instant::now();
    let _ = process_style_tags(&text);
    let duration = start.elapsed();

    // Проверяем что обработка стилей быстрая
    assert!(duration.as_millis() < 50, "Style processing too slow: {:?}", duration);
}
