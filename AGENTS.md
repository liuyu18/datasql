# AGENTS.md

## Build & Run
- `cargo build` - Build the project
- `cargo run` - Run the binary (currently just prints "Hello, world!")
- `cargo test` - Run tests (includes lexer tests in `src/sql/parser/lexer_test.rs`)

## Project Structure
- `src/main.rs` - Entry point
- `src/sql/` - SQL parser modules
- `src/sql/parser/` - Lexer and AST
- `src/sql/types/` - Type definitions

## Testing
Tests are inline in `src/sql/parser/lexer_test.rs`. Run all tests with `cargo test`.