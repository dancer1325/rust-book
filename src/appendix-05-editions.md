## Appendix E - Rust Editions

* `cargo new`
  * adds a bit of metadata | your "Cargo.toml"
    * _Example:_ about an edition

* (Rust language & compiler) 's release cycle: 6-week

* Rust *edition* 
  * / EACH 2 OR 3 years
  * == ALL features / have been landed TILL THEN
  * uses / audience
    * | active Rust users
      * == incremental changes / easy-to-understand package
    * | non-users,
      * == major advancements / might make Rust worth another look
    * | Rust developers,
      * == rallying point -- for -- the project

* this book
  * written -- by -- using Rust 2021 edition idioms

TODO: 
The `edition` key in *Cargo.toml* indicates which edition the compiler should
use for your code. If the key doesn’t exist, Rust uses `2015` as the edition
value for backward compatibility reasons.

Each project can opt in to an edition other than the default 2015 edition.
Editions can contain incompatible changes, such as including a new keyword that
conflicts with identifiers in code. However, unless you opt in to those
changes, your code will continue to compile even as you upgrade the Rust
compiler version you use.

All Rust compiler versions support any edition that existed prior to that
compiler’s release, and they can link crates of any supported editions
together. Edition changes only affect the way the compiler initially parses
code. Therefore, if you’re using Rust 2015 and one of your dependencies uses
Rust 2018, your project will compile and be able to use that dependency. The
opposite situation, where your project uses Rust 2018 and a dependency uses
Rust 2015, works as well.

To be clear: most features will be available on all editions. Developers using
any Rust edition will continue to see improvements as new stable releases are
made. However, in some cases, mainly when new keywords are added, some new
features might only be available in later editions. You will need to switch
editions if you want to take advantage of such features.

[Edition Guide book](https://doc.rust-lang.org/stable/edition-guide/) 
