pub mod packages;
pub mod a_module;

/// As one proceeds in writing a project, it may be normal to want to split the code into multiple
/// files, may it be for clearance, for keeping order, or for the need of modularity. Rust allows
/// for that.
///
/// A lot of it works with the concept of "scope": in a scope, two elements can't have the same
/// name, but we can allow or deny the use or access of some items inside scopes.
///
/// There are 4 important features that Rust adopts for splitting the code into multiple parts:
///  - **Packages**: this feature allows for the creation, sharing and use of crates, as well as
///                  building and testing;
///  - **Crates**: a tree of modules, which ultimately creates a library;
///  - **Modules and use**: Allows to control the organization, scope and privacy of the various
///                         paths in the code;
///  - **Paths**: The way of naming an item (be it a function, a variable, ...).
fn main() {
    {
        packages::modules_func();
    }
}


