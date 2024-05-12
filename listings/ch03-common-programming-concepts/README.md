# Goal
* Variables and mutability
* Data types
* Functions
* Comments
* Control flow

# How have the subdirectories been created?
* `cargo new project_name`

# How to run locally?
* `cargo run`

# Variables and mutability
* 'no-listing-01-variables-are-immutable'
  * `cargo run` does NOT run
    * Reason: Variables are immutable by default
* 'no-listing-02-adding-mut'
  * make a variable, mutable
  * `cargo run`, checking that it compiles now
* 'no-listing-03-shadowing'
  * shadowing a variable
* 'no-listing-04-shadowing-can-change-types'
  * shadowing can change the type
* 'no-listing-05-mut-cant-change-types'
  * mutable variables can NOT change the type
  * `cargo run` does NOT compile
## Constant
* 'no-listing-02-adding-mut'
