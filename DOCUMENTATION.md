# PrettyTerm Documentation

## Quick Start

```rust
use prettyterm::*;

fn main() {
    // Styling
    println!("{}", sty!("<green|bold>SUCCESS</green|bold>"));
    
    // Logging
    let mut logger = Logger::new(LogTime::now(), true);
    let component = Component::new("main.rs".into(), "main".into(), "src".into());
    logger.add_log("Started".into(), component, Status::Info, None);
    
    // Tree-structured output
    let root = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    let child = root.enter_branch("Processing");
    println!("{}", child.format_branch_line("Step 1", "├─ "));
}
```

---

## 1. Colors

### Basic Constants

```rust
use prettyterm::*;

println!("{}Red text{}", FG_RED, RESET_COLOR);
println!("{}Green bg{}", BG_GREEN, RESET_COLOR);
println!("{}Bold{}", STYLE_BOLD, RESET_COLOR);
```

**Text colors:** `FG_BLACK`, `FG_RED`, `FG_GREEN`, `FG_YELLOW`, `FG_BLUE`, `FG_MAGENTA`, `FG_CYAN`, `FG_WHITE`

**Background colors:** `BG_BLACK`, `BG_RED`, `BG_GREEN`, `BG_YELLOW`, `BG_BLUE`, `BG_MAGENTA`, `BG_CYAN`, `BG_WHITE`

**Styles:** `STYLE_BOLD`, `STYLE_FADED`, `STYLE_ITALIC`, `STYLE_UNDERLINE`, `STYLE_BLINKING`, `STYLE_CROSSED_OUT`

**Reset:** `RESET_COLOR`

---

## 2. Stylish (XML-like Tags for Styling)

### The `sty!` Macro

```rust
// Simple tag
sty!("<red>Error!</red>");

// Combined styles using |
sty!("<red|bold|underline>Critical!</red|bold|underline>");

// With variable interpolation
let name = "Alice";
sty!("Hello, <cyan>{}</cyan>!", name);

// Nested tags
sty!("<red>Error: <bold>fatal</bold></red>");
```

**Available tags:**
- Colors: `red`, `green`, `blue`, `yellow`, `magenta`, `cyan`, `white`, `black`
- Styles: `bold`, `italic`, `underline`, `faded`, `blinking`, `crossedout`
- Background: `bg-red`, `bg-green`, `bg-blue`, `bg-yellow`, `bg-magenta`, `bg-cyan`, `bg-white`, `bg-black`

### Direct Usage

```rust
let text = process_style_tags("<green>OK</green>");
println!("{}", text);
```

---

## 3. Logger (Component-Based Logging)

### Creating a Logger

```rust
let mut logger = Logger::new(
    LogTime::now(),           // Creation time
    true                       // Output to terminal immediately
);

// Set output style
logger.style = LoggerPrintStyle::Full;
```

### Component (Source of the Log)

```rust
let component = Component::new(
    "lexer.rs".to_string(),      // File name
    "tokenize".to_string(),       // Function name
    "src/frontend".to_string()    // Directory path
);
```

### Adding Logs

```rust
// Automatic timestamp
logger.add_log(
    "Tokenization started".to_string(),
    component.clone(),
    Status::Info,
    None
);

// Manual timestamp
logger.add_log(
    "Error occurred".to_string(),
    component,
    Status::Error,
    Some(LogTime::new(14, 30, 45))
);
```

### Statuses

```rust
Status::Ok       // "Ok"
Status::Error    // "Error"
Status::Fatal    // "Fatal"
Status::Info     // "Info"
Status::Warn     // "Warning"
```

### Formatting Styles

```rust
logger.style = LoggerPrintStyle::Tiny;
// Ok: message | from file.rs-func:main, time is 14:30:45

logger.style = LoggerPrintStyle::Flat;
// Ok: message | file file.rs | time 14:30:45

logger.style = LoggerPrintStyle::Full;
// [Ok|14:30:45][src/frontend/lexer.rs-tokenize]: message
```

### Shutting Down

```rust
logger.destroy(
    "./output.log",   // File path
    true,             // Write to file?
    false             // Output all logs now?
).unwrap();
```

---

## 4. Tree Printer (Tree-Structured Output)

### Creating a Tree

```rust
let root = Branch::new(
    DisplayConfig::default(),
    BranchStyle::Unicode  // or BranchStyle::Indent
);
```

### Navigating the Tree

```rust
let root = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
println!("Compilation");

// Enter a child node
let lex = root.enter_branch("Lexical analysis");
// Outputs: ├─ Lexical analysis

// Print a line at the current level
println!("{}", lex.format_branch_line("Tokenizing...", "├─ "));
// Outputs: │  ├─ Tokenizing...

// Go deeper
let parser = lex.enter_branch("Parsing");

// Exit the node
println!("{}", parser.leave_branch("Done", Status::Ok));
// Outputs: │  │  ╰─ Done
```

### Printing Tables

```rust
// Table header
println!("{}", branch.format_table_header("Error Details"));
// ├─ Error Details ─────────────────╮

// Table row
println!("{}", branch.format_table_line("File: main.rs"));
// │  │ File: main.rs                │

// Multi-line text (automatic wrapping)
println!("{}", branch.format_table_multi_line(
    "This is a very long error message that will be \
     automatically wrapped to fit terminal width"
));

// Table footer
println!("{}", branch.format_table_footer());
// ├───────────────────────────────────╯
```

### Printing Code with Line Numbers

```rust
let code = r#"fn main() {
    println!("Hello");
}"#;

println!("{}", branch.format_table_code_multi_line(1, code));
// │  │ 1| fn main() {               │
// │  │ 2|     println!("Hello");    │
// │  │ 3| }                          │
```

### Branch Styles

```rust
// Unicode (default): │  ├─ ╰─ ╮ ╯
let branch = Branch::new(config, BranchStyle::Unicode);

// Indent (spaces):       
let branch = Branch::new(config, BranchStyle::Indent);
```

---

## 5. Theme Config (Theme Customization)

### Color Theme

```rust
let theme = ColorTheme::new(
    TerminalColors::Blue,     // Hint
    TerminalColors::Red,      // Error
    TerminalColors::Green,    // Success
    TerminalColors::Yellow    // Warning
);

// Or default
let theme = ColorTheme::default();
```

### Icon Theme

```rust
let icons = IconsTheme::new(
    "🛈".to_string(),  // Hint
    "✗".to_string(),   // Error
    "✓".to_string(),   // Success
    "⚠".to_string()    // Warning
);

// Or default
let icons = IconsTheme::default();
```

### Display Configuration

```rust
let config = DisplayConfig::new(
    ColorTheme::default(),
    IconsTheme::default(),
    (80, 24)  // (width, height) of the terminal
);

// Auto-detect terminal size
let config = DisplayConfig::default();
```

---

### Example: CLI util with logs

```rust
use prettyterm::*;

fn process_files(files: &[&str]) {
    let mut logger = Logger::new(LogTime::now(), true);
    logger.style = LoggerPrintStyle::Tiny;
    
    let root = Branch::new(DisplayConfig::default(), BranchStyle::Unicode);
    
    for file in files {
        let comp = Component::new(file.to_string(), "process".to_string(), ".".to_string());
        
        let branch = root.enter_branch(&format!("Processing {}", file));
        
        logger.add_log(format!("Opening {}", file), comp.clone(), Status::Info, None);
        
        // Симуляция обработки
        println!("{}", branch.format_branch_line("Reading content", "├─ "));
        println!("{}", branch.format_branch_line("Validating format", "├─ "));
        println!("{}", branch.format_branch_line("Writing output", "├─ "));
        
        logger.add_log(format!("Processed {}", file), comp, Status::Ok, None);
        println!("{}", branch.leave_branch("Done", Status::Ok));
    }
    
    logger.destroy("./processing.log", true, false).unwrap();
}

fn main() {
    process_files(&["file1.txt", "file2.txt", "file3.txt"]);
}
```

---

## Use visual len

```rust
use prettyterm::visual_len;

let styled = sty!("<red>Hello</red>");
let len = visual_len(&styled); // 5, без учёта ANSI-кодов
```

---

## testing

```bash
# run tests
cargo test
# with output
cargo test -- --nocapture
```
