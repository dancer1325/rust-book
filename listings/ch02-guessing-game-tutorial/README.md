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
## listing-02-03
* Add 
  * 'rand' crate dependency &
  * use it create a random number
## listing-02-04
* Try to compare unsuccessfully the inputNumber vs randomNumber
  * == `cargo build` does NOT compile
## no-listing-03-convert-string-to-number
* Parsing previously to compare inputNumber vs randomNumber
* 'Shadowing'
## no-listing-04-looping
* `loop`
  * keyword
  * infinite loop without breaking
## listing-02-05
* `loop` + specifying to quit (`break`)
* `expect` -> `match` to handle the error and let continue without skipping
## listing-02-06
* Remove logging line


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