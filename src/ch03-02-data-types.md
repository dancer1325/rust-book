## Data Types

* 👁️ALL value | Rust -- is of a -- certain *data type* 👁️
* data type subsets
  * scalar
  * compound

* ⭐Rust is a *statically typed* language ⭐
  * == MUST know ALL variables' types | compile time
* Rust compiler -- can usually infer -- type / want to use
  * -- based on the --
    * value
    * how we use it
  * 👁️ if many types are possible -> we must add a type annotation 👁️
    * _Example:_ if we convert, via `parse`, a `String` -- to a -- numeric type
      * -> type annotation recommended

      ```rust
      let guess: u32 = "42".parse().expect("Not a number!");
      ```

      * Check [“Comparing the Guess to the Secret
        Number”][comparing-the-guess-to-the-secret-number]

      * else, following error displayed -- Check 'no_type_annotations' --

      ```console
      {{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
      ```

### Scalar Types

* = 1! value
* 4 primary scalar types
  * integers,
  * floating-point numbers,
  * Booleans,
  * characters

#### Integer Types

* = number / WITHOUT a fractional component
* variants
  * unsigned -- `u` --
    * == negative or positive
    * range of numbers [0, 2<sup>n</sup> - 1]
      * _Example:_ `u8` has a range of [0, 2<sup>8</sup> - 1] = [0, 255]
  * signed -- `i` -- 
    * == ONLY positive
    * range of numbers [-(2<sup>n - 1</sup>), 2(<sup>n - 1</sup>) - 1], being n, the # of bits
      * _Example:_ `i8` has a range of [-(2<sup>7</sup>), 2<sup>7</sup> - 1] =[-128, 127]
    * is stored -- via -- [two’s complement][twos-complement] representation

<span class="caption">Table 3-1: Integer Types in Rust</span>

| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

* "arch" 
  * `isize` & `usize` types -- depend on the -- architecture of the computer | your program is running on
  * _Example:_ 64 bits == 64-bit architecture, 32 bits == 32-bit architecture

* TODO:
You can write integer literals in any of the forms shown in Table 3-2. Note
that number literals that can be multiple numeric types allow a type suffix,
such as `57u8`, to designate the type. Number literals can also use `_` as a
visual separator to make the number easier to read, such as `1_000`, which will
have the same value as if you had specified `1000`.

<span class="caption">Table 3-2: Integer Literals in Rust</span>

| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

So how do you know which type of integer to use? If you’re unsure, Rust’s
defaults are generally good places to start: integer types default to `i32`.
The primary situation in which you’d use `isize` or `usize` is when indexing
some sort of collection.

> ##### Integer Overflow
>
> * If you try to change the variable -- to a -- value outside it's range -> happen *integer overflow* / possible behaviors 
>   * _Example: `u8` variable / try to assign 256 
>   * if you’re compiling in debug mode -> Rust includes checks for integer overflow
>     * == if it happens | runtime -> *panic* -- [“Unrecoverable Errors with `panic!`”][unrecoverable-errors-with-panic]
>   * if you’re compiling in release mode -- via -- `--release` -> Rust performs *two’s complement wrapping*
>     * == values / > maximum value -> the type can hold “wrap around” to the minimum 
>       * _Example:_ let's `u8` -> 256 becomes 0, the value 257 becomes 1, ...
>     * != panic 
>     * recommendations
>       * 👁️NOT rely on integer overflow’s wrapping 👁️
>     * ways to explicitly handle
>       * wrap in all modes
>         * `wrapping_*` methods -- _Example:_ `wrapping_add` -- 
>       * return the `None` value 
>         * `checked_*` methods 
>       * return the value + boolean / indicate whether there was overflow
>         * `overflowing_*` methods
>       * saturate | value’s minimum or maximum values
>         * `saturating_*`

* _Example:_ TODO:

#### Floating-Point Types

* == numbers / decimal points
  * ONLY signed
  * -- represented according to the -- IEEE-754 standard
* primitive types for *floating-point numbers*
  * `f32`
    * 32 bits size
    * single-precision
  * `f64`
    * 64 bits size
    * default one
    * double precision
    * vs `f32`
      * == speed | modern CPUs
      * more precision
* _Example:_

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

#### Numeric Operations

* basic mathematical operations
  * addition,
  * subtraction,
  * multiplication,
  * division
    * Integer division -- truncates toward -- zero to the nearest integer
  * remainder
* [Appendix B - list of ALL operators][appendix_b]
* _Example:_ 

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

#### The Boolean Type -- `bool` --

* allowed values
  * `true`
  * `false`
* 1 byte size The Boolean type in
* _Example:_ 

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

* uses
  * | conditionals
    * _Example:_ `if` expression  [“Control Flow”][control-flow]<!-- ignore --> section.

#### The Character Type -- `char` -- 

* MOST primitive alphabetic type
* 👁️ single quotes - `''` 👁️
  * vs `String`, with -- `""` --
* _Example:_ 

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```
* 4 bytes in size
* -- represents a -- Unicode Scalar Value
  * == -- can represent -- a lot more than just ASCII
  * _Example:_ Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces
  * range [`U+0000`, `U+D7FF`] & [`U+E000`, `U+10FFFF`]
  * “character” | Unicode != concept
    * check [“Storing UTF-8 Encoded Text with Strings”][strings]

### Compound Types

* allows
  * multiple values -- can be grouped into -- 1 type
* primitive built-in compound types
  * tuples
  * arrays

#### The Tuple Type

* allows
  * grouping together values / variety of types
* fixed length
  * == once declared -> they can NOT grow or shrink size
* `(value1, value2, ...)`
  * `value1`'s type can be != `value2`'s type
  * `()`
    * named *unit*
    * == tuple / NO values
    * uses
      * expressions / do NOT return ANY value
* _Example:_

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

* ways to access individual tuple's values
  * destructuring it

  <span class="filename">Filename: src/main.rs</span>
  
  ```rust
  {{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
  ```

  * `tupleVariable.index` / index starts by 0 

  <span class="filename">Filename: src/main.rs</span>
  
  ```rust
  {{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
  ```

#### The Array Type

* TODO:
Another way to have a collection of multiple values is with an *array*. Unlike
a tuple, every element of an array must have the same type. Unlike arrays in
some other languages, arrays in Rust have a fixed length.

We write the values in an array as a comma-separated list inside square
brackets:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Arrays are useful when you want your data allocated on the stack rather than
the heap (we will discuss the stack and the heap more in [Chapter
4][stack-and-heap]<!-- ignore -->) or when you want to ensure you always have a
fixed number of elements. An array isn’t as flexible as the vector type,
though. A *vector* is a similar collection type provided by the standard
library that *is* allowed to grow or shrink in size. If you’re unsure whether
to use an array or a vector, chances are you should use a vector. [Chapter
8][vectors]<!-- ignore --> discusses vectors in more detail.

However, arrays are more useful when you know the number of elements will not
need to change. For example, if you were using the names of the month in a
program, you would probably use an array rather than a vector because you know
it will always contain 12 elements:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

You write an array’s type using square brackets with the type of each element,
a semicolon, and then the number of elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of each element. After the semicolon, the number `5`
indicates the array contains five elements.

You can also initialize an array to contain the same value for each element by
specifying the initial value, followed by a semicolon, and then the length of
the array in square brackets, as shown here:

```rust
let a = [3; 5];
```

The array named `a` will contain `5` elements that will all be set to the value
`3` initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a
more concise way.

##### Accessing Array Elements

An array is a single chunk of memory of a known, fixed size that can be
allocated on the stack. You can access elements of an array using indexing,
like this:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

In this example, the variable named `first` will get the value `1` because that
is the value at index `[0]` in the array. The variable named `second` will get
the value `2` from index `[1]` in the array.

##### Invalid Array Element Access

Let’s see what happens if you try to access an element of an array that is past
the end of the array. Say you run this code, similar to the guessing game in
Chapter 2, to get an array index from the user:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

This code compiles successfully. If you run this code using `cargo run` and
enter `0`, `1`, `2`, `3`, or `4`, the program will print out the corresponding
value at that index in the array. If you instead enter a number past the end of
the array, such as `10`, you’ll see output like this:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The program resulted in a *runtime* error at the point of using an invalid
value in the indexing operation. The program exited with an error message and
didn’t execute the final `println!` statement. When you attempt to access an
element using indexing, Rust will check that the index you’ve specified is less
than the array length. If the index is greater than or equal to the length,
Rust will panic. This check has to happen at runtime, especially in this case,
because the compiler can’t possibly know what value a user will enter when they
run the code later.

This is an example of Rust’s memory safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects you against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. Chapter 9 discusses more of Rust’s error handling and how you can
write readable, safe code that neither panics nor allows invalid memory access.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
