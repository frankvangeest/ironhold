# Coding Standards

This document defines the initial coding standards for the Ironhold project. These guidelines are minimal and will be expanded as the project grows.

## Rust
- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/).
- Imports should **not** be written on the same line. Use multiple lines for clarity.
- Preferred style:

```rust
use clap::{
    Parser,
    Subcommand
};
```

instead of:

```rust
use clap::{Parser, Subcommand}; // Don't write them on the same line
```

- Use `rustfmt` for consistent formatting.

## HTML
- Use semantic HTML tags where possible.
- Indent using 2 spaces.
- Keep attributes in lowercase and use double quotes for values.

## CSS
- Use simple, readable class names (kebab-case).
- Indent using 2 spaces.
- Group related properties logically.

## JavaScript
- Use `const` and `let` instead of `var`.
- Prefer ES6+ syntax.
- Indent using 2 spaces.
- Use meaningful variable and function names.

---
These standards are a starting point and will be expanded later as needed.
