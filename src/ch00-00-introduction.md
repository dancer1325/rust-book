# Introduction

* "The Rust Programming Language" book
  * == introductory book -- about -- Rust

* the compiler
  * ❌if there are elusive bugs (ALSO concurrency bugs) -> NOT compile❌

## How to Use This Book

* [LOWER number chapters, HIGHER number chapters]
* types of chapters
  * concept chapters
  * project chapters
    * _Examples:_ 
      * [2](ch02-00-guessing-game-tutorial.md)
      * [12](ch12-00-an-io-project.md)
      * [20](ch20-00-final-project-a-web-server.md)

TODO:

* Chapter 19 contains a

more about lifetimes,

Finally, some appendices contain useful information about the language in a
more reference-like format
* Appendix A covers Rust’s keywords, Appendix B
covers Rust’s operators and symbols, Appendix C covers derivable traits
provided by the standard library, Appendix D covers some useful development
tools, and Appendix E explains Rust editions
* In Appendix F, you can find
translations of the book, and in Appendix G we’ll cover how Rust is made and
what nightly Rust is.

There is no wrong way to read this book: if you want to skip ahead, go for it!
You might have to jump back to earlier chapters if you experience any
confusion
* But do whatever works for you.

<span id="ferris"></span>

An important part of the process of learning Rust is learning how to read the
error messages the compiler displays: these will guide you toward working code.
As such, we’ll provide many examples that don’t compile along with the error
message the compiler will show you in each situation
* Know that if you enter
and run a random example, it may not compile! Make sure you read the
surrounding text to see whether the example you’re trying to run is meant to
error
* Ferris will also help you distinguish code that isn’t meant to work:

| Ferris                                                                                                           | Meaning                                          |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | This code panics!                                |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | This code does not produce the desired behavior. |

In most situations, we’ll lead you to the correct version of any code that
doesn’t compile.
