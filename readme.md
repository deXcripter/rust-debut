### Why Rust

1. High-level language features without performance penalties.
2. Program behaviours can be enforced at compile time.
   - Enhance program readability.
3. Built-in dependency management, similar to npm.
4. Quickly growing ecosystem of libraries.

### Technical Rust Goodies

- First-class multitreading
  - Compiler error to improperly access shared data.
- Type system:
  - Can uncover bugs at compile time.
  - Makes refactoring simple.
  - Reduces t he number of tests needed.
- Module system makes code separation simple.
- Adding a dependency is a 1 line job in a config file.
- Tooling:
  - Generate docs, linting code, and auto format.

### Data Types INTRO

- Memory only every store binary data - and anything can be represented as binary data.
- Program determines what the binary data represents.
- Basic types that are universally usefil are provided by the language, but we can define our own.

- Basic Data Types
- Boolean: true or false
- Integer: whole numbers, positive or negative
- Double / Float: decimal numbers
- Character: single characters (e.g., 'a', 'b', 'c')
- String: sequence of characters (e.g., "hello", "world")

### Variables

- Assigns data to a temporary memory location
  - Allows programmer to easuly work with data
- Variables can be set to any value and type
- Variables in rust are immutable by default
  - Mutable (can be changed)
  - Immutable (cannot be changed)

```rust
let two = 2;
let hello = "hello";
let my_half = 0.5;
let mut my_name = "Bill";
let your_half = my_half;
```
