# AGENTS.md

## Commands
- `cargo fmt` formats the whole crate; no repo-local `rustfmt.toml` is present.
- `cargo test` is the main verification command; there is no CI workflow or task runner config in this repo.
- Run focused tests by substring, for example `cargo test test_plan_create_table` or `cargo test sql::plan::tests`.
- Use `cargo test sql::plan::tests -- --nocapture --test-threads=1` when debugging planner logs; `--test-threads=1` keeps the printed trace ordered.
- `cargo run` currently only enters `src/main.rs` and prints `Hello, world!`; SQL behavior is exercised through tests, not the binary.

## Project Shape
- This is a single Rust binary crate named `datasql` with no external dependencies in `Cargo.toml`.
- `src/main.rs` only declares `mod error; mod sql;`; the useful implementation lives under `src/sql/`.
- `src/sql/parser/mod.rs` is the parser entrypoint. It exposes `Parser` and `ast`, while `lexer.rs` is private to the parser module.
- `src/sql/plan/mod.rs` defines public `Plan` and `Node`; `Plan::build` delegates to private `planner::Planner` in `src/sql/plan/planner.rs`.
- `src/sql/schema.rs` and `src/sql/types/mod.rs` hold the planned schema/value types used by planner output.

## SQL Coverage To Preserve
- Parser currently accepts exactly one semicolon-terminated statement and rejects trailing tokens after the semicolon.
- Supported statements are `CREATE TABLE`, `INSERT INTO ... VALUES ...`, and `SELECT * FROM table_name`.
- Identifiers are normalized to lowercase by the lexer; SQL keywords are matched case-insensitively.
- Column type aliases collapse to internal `DataType`: `int/integer`, `bool/boolean`, `float/double`, and `string/text/varchar`.
- Planner converts nullable columns with no explicit default to `Some(Value::Null)` and non-null columns with no default to `None`.

## Tests
- Tests are inline in `src/error.rs`, `src/sql/parser/lexer.rs`, and `src/sql/plan/mod.rs`; there is no separate `tests/` directory.
- Planner tests intentionally trigger `#[cfg(test)]` `planner_log!` output in `planner.rs`; use `--nocapture` to see it.
