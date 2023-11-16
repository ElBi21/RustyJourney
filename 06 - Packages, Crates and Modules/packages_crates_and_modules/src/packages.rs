use crate::a_module;

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
///  2. When it finds one, it looks for any `mod <package_name>` inside the///root* file. The `mod`
///     keyword allows Rust to recognise a **module**. Rust will find for the specific file, so if
///     we type `mod my_module;`, then there must either be:
///      - a `my_module.rs` file;
///      - a folder named `my_module` where inside there is a `mod.rs` file.
///     All the modules specified in the `main.rs` file can be used across the whole crate.
///     An example follows, with an example module named "house":
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
///          └ mod.rs
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
/// mod kitchen;
///
/// fn new_house() {
///     kitchen::clean_sink();
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
pub(crate) fn modules_func() {
    {
        let an_integer: i32 = 16;
        a_module::number_adder(an_integer);
    }
}

/// When using modules we might have a situation of multiple modules nested in each other. We can
/// refer to them with a notation similar to the one used in OOP. For instance, let us assume that we
/// have the following crate tree:
///
/// ```txt
/// crate
///  ├─ module_a
///  │    └ module_c
///  └─ module_b
/// ```
///
/// In this case, `module_a` is considered the **parent** of `module_c`, while `module_a` and
/// `module_b` are said to be **siblings**. `module_c` is called **child** of `module_a`.
///
///
/// When referring to modules, the first time that we run a program with a self made module we will
/// probably get an error, but why is that? That's because usually in Rust, if there is no
/// specification, a module is considered **private**. A solution could be to use the `pub` keyword in
/// front of the declaration of the module, but it won't work. The error that we would get would
/// always be that the function inside our module is private. What about using `pub` also in front
/// of the function?
///