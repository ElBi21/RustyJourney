/// **Packages and Crates**
///
/// In Rust, a **crate** is the smallest amount of code that is considered at a time, during compile
/// time, by the Rust compiler. It comes in two types:
///  - **Binary crate**: it's a crate that compiles to a runnable executable, and it could be a
///                      command line program or, for instance, a server. Each binary crate has its
///                      own `main()` function;
///  - **Library crate**: this kind of crate doesn't have any `main()` function, but it's instead
///                       meant to hold extensible functions (an example could be the `Matplotlib`
///                       package for `Python`).
/// The word **crate** is mostly used to refer to **library crates**.
///
/// A crate gets compiled with the following steps:
///  1. The compiler looks for a `src/lib.rs` file (if the crate is a library crate) or a
///     `src/main.rs` file (if the crate is a binary crate);
///  2. When it finds one, it looks for any `mod <package_name>` inside the file. The `mod` keyword
///     allows Rust to recognise a **module**. An example follows, with an example module named
///     "house":
///
/// → File Path:
/// ```
/// [Entry Point]
///    ↓
/// main_crate
///    └ src
///       ├ main.rs
///       ├ house.rs
///       └ kitchen
///          └ sink.rs
/// ```
///
/// → `main.rs` (Case of the module's core inside the main file)
/// ```rust
/// mod house {
///     fn new_house() {
///         // Code...
///     }
/// }
///
/// fn main() {
///     new_house();
/// }
///```
///
/// → `house.rs` (Case of the module's core outside the main file)
/// ```rust
/// mod house;
///
/// fn new_house() {
///     // Code...
/// }
/// ```
///
///  3. Submodules can be declared the same way a module is declared. The compiler will look for
///     them after having looked for the main module;
///  4. A module by default is private. To make it public, the keyword `pub` must be used (an
///     example is the following: `pub mod house;`)
///  5. Inside the file where we want to use such module, we can use the `use` keyword, which allows
///     us to use the data inside one module without having to write everytime the full path. An
///     example follows:
/// ```rust
/// // Instead of crate::kitchen::sink::drain();...
/// use crate::kitchen::sink::drain();
///
/// // Now we can use freely the drain() function, like this:
/// drain();
/// ```
fn packages() {

}