# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.


## Tooling for shell interactions (Install if missing)

Is it about finding FILES? use 'fd'

Is it about finding TEXT/strings? use 'rg'

Is it about finding CODE STRUCTURE? use 'ast-grep',You run in an environment where `ast-grep` is available; whenever a search requires syntax-aware or structural matching, default to `ast-grep --lang rust -p '<pattern>'` (or set `--lang` appropriately) and avoid falling back to text-only tools like `rg` or `grep` unless I explicitly request a plain-text search.

Is it about SELECTING from multiple results? pipe to 'fzf'

Is it about interacting with JSON? use 'jq'

Is it about interacting with YAML or XML? use 'yq'

Is it about analyzing LOG PATTERNS? use 'angle-grinder'

Is it about working with CSV/TSV? use 'xsv'

Is it about viewing CODE with syntax? use 'bat' 


## Common Commands

- `cargo run` - Compile and run the main program which demonstrates solutions
- `cargo test` - Run all unit tests
- `cargo test <test_name>` - Run a specific test (e.g., `cargo test two_sum`)
- `cargo check` - Fast compile check without generating binaries
- `cargo build` - Build the project without running
- `cargo clippy` - Run the Rust linter for code quality checks

## Project Structure

This is a LeetCode practice repository organized as follows:

- **`src/main.rs`** - Entry point that demonstrates solution usage
- **`src/solutions/`** - Directory containing individual problem solutions
- **Solution Pattern** - Each problem is implemented as:
  - A `pub struct Solution;` with static methods
  - Unit tests in the same file using `#[cfg(test)]` mod
  - Problems are organized in separate modules/files

## Adding New Solutions

1. Create a new file in `src/solutions/` (e.g., `problem_name.rs`)
2. Implement the solution following the existing pattern:
   ```rust
   pub struct Solution;
   
   impl Solution {
       pub fn problem_name(/* params */) -> /* return type */ {
           // implementation
       }
   }
   
   #[cfg(test)]
   mod tests {
       use super::*;
       #[test]
       fn test_problem_name() {
           // test cases
       }
   }
   ```
3. Add the module to `src/main.rs`: `pub mod problem_name;`
4. Add a demonstration call in the `main()` function

## Testing Strategy

- Each solution includes unit tests in the same file
- Tests follow the pattern `test_<problem_name>()`
- Use `assert_eq!` for expected outputs
- Run individual tests: `cargo test test_two_sum`
