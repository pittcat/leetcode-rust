# Repository Guidelines

## Project Structure & Module Organization
- Source: `src/solutions/` (one file per LeetCode problem, e.g. `two_sum.rs`, `match_basic.rs`, `match_map_get_explained.rs`).
- Entry: `src/main.rs` for demos and quick local runs.
- Config: `Cargo.toml`, `Cargo.lock`. Docs: `README.md`, `CLAUDE.md`, `AGENTS.md`.
- Tests: co-located in each module via `#[cfg(test)]` at the file end.

## Build, Test, and Development Commands
- `cargo check` — fast type/borrow checks; use during iteration.
- `cargo build` — compile (debug). `cargo run` — run `main`.
- `cargo test` — run all tests. Example: `cargo test test_two_sum`.
- `cargo clippy` — lints; fix warnings before PRs. `cargo fmt` — format.

## Coding Style & Naming Conventions
- Use `rustfmt` defaults; 4-space indent; avoid trailing whitespace.
- Naming: modules/files `snake_case`; types/structs/traits `CamelCase`;
  functions/methods `snake_case`; constants `SCREAMING_SNAKE_CASE`.
- Prefer borrowing over cloning; minimize heap allocs; favor iterators and
  `Option`/`Result` over panics. Keep solutions ergonomic and idiomatic.
- Per problem: `pub struct Solution;` with methods on `impl Solution`.

## Testing Guidelines
- Use Rust’s built-in test framework with `#[test]` inside `#[cfg(test)]`.
- Name tests descriptively (e.g., `test_two_sum_basic`, `test_edge_cases`).
- Cover boundaries (empty inputs, duplicates, large values). Keep tests fast
  and deterministic. Run `cargo test` locally before pushing.

## Commit & Pull Request Guidelines
- Commits: imperative and concise. Prefer Conventional Commits when possible
  (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`). English preferred; clear
  Chinese acceptable when needed. Add context in the body if non-trivial.
- PRs: include summary, rationale, and scope; link issues; show sample I/O for
  new solutions; update docs if behavior or structure changes. Pass `clippy`,
  `fmt`, and tests.

## Agent-Specific Notes (if using AI tools)
- Keep patches minimal and focused; one problem per file under `src/solutions/`.
- Add unit tests with new solutions; run `cargo fmt` and `cargo clippy`.
- Avoid heavy dependencies without discussion; document structural changes here.

