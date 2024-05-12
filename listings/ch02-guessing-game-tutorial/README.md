# Game rules
* Program generates a random integer [1, 100]
* User needs to enter a number -> program will say if
  * higher
  * lower
  * exact

# How have the subdirectories been created?
* `cargo new project_name`

# How to run locally?
* `cargo run`

# Projects
## listing-02-01
* Read an input line
## listing-02-02
* Add 'rand' crate dependency

# Notes
* `std::io`
  * standard library
  * input and output
  * `use ...`
    * if the type is NOT in the prelude -> `use` is necessary -- NOT in this case --
    * 'listing-02-01' -- importing by default --
    * 'listing-02-01-nouse' -- WITHOUT importing --
* `let variableName`
  * create variables
* `io::stdin().read_line()`
  * if you do NOT append with `.expect()` & run `cargo build` -> you get a warning
* `cargo update`
  * update a crate
    * ignoring Cargo.lock