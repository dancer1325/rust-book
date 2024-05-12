# How has it been created?
* `cargo new created_with_cargo`
  * if it's created under a git repo -> NO '.gitignore' is created

# How to run locally?
* ways
  * 2 steps
    * `cargo build`
      * causes that
        * 'cargo.lock' is created at top level
      * Problems
        * Problem1: Where to find the '/target'?
          * Attempt1: $HOME/target
          * Attempt2: ProjectDirectory/target
          * Solution1: Specify `cargo build --target-dir=./target`
          * Solution: TODO
    * `./target/debug/created_with_cargo`
      * executes the executable
  * 1! step
    * `cargo run`

# Notes
* vs 'manuallyCreated'
  * 'cargo.toml'
  * '/src'
* `cargo check`
  * check if the code compiles / 👁️NOT run an executable 👁️
* `cargo build --release`
  * create an optimized executable under '/target/release'
  

