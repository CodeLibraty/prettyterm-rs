# PrettyTerm-rs
PrettyTerm is a standalone logging and terminal visualization library for structured applications. Unlike general-purpose logging facades, it provides component-based logging with built-in formatting styles and tree visualization - perfect for compilers, build tools, and structured CLI applications

## Design Philosophy

1. **Component-based:** Each log knows where it came from (file, function, path)
2. **Zero-config:** Works out of the box, settings are optional
3. **Standalone:** Does not require integration with other logging systems
4. **Visual-first:** Priority on readability and output structure
5. **Compiler-friendly:** Designed for compilers, build systems, and CLI utilities

---

## When should I use PrettyTerm?

**Use it if:**
- You are writing a compiler/transpiler
- Are you creating a build system or a task runner
- We need a component logger with a context
- We need a tree view of the stages
- Do you want a beautiful output without customization

**Do not use it if:**
- You are writing a library (preferably a `log` trace)
- We need log rotation and complex configuration (better than `log4rs`)
- We need structured logging with JSON (better than `tracing`)
- We need async logging (better than `tracing` or `slog`)

---

## Authors

## Authors
- [`CodeLibraty Foundation`](https://github.com/CodeLibraty)
- [`Rejzi`](https://github.com/Rejzi-dich)

## License
see the [`LICENSE`](./LICENSE) file