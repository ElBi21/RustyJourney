fn increments(int: i32) -> i32 {
    int + 1
}

pub mod operations {
    pub fn custom_addition(x: i32, y: i32) -> i32 {
        super::increments(x) + y
    }
}

/// We can also implement `structs` and `enums` within a module, and in order to make them available
/// outside we have to use the `pub` keyword. For instance, here the [Equation] struct is public, while
/// [NumberType] is private. We can use the [NumberType] enumeration by using some functions, such as
/// [get_num_type()]

#[derive(Debug)]
pub struct Equation {
    numbers: NumberType,
    operator: String,
}

impl Equation {
    pub fn new(ntype: i32, op: String) -> Equation {
        Equation {
            numbers: get_num_type(ntype),
            operator: op,
        }
    }
}

#[derive(Debug)]
enum NumberType {
    Integer,
    Rational,
    Float,
    Complex,
}

fn get_num_type(num_type: i32) -> NumberType {
    match num_type {
        1 => NumberType::Integer,
        2 => NumberType::Rational,
        3 => NumberType::Float,
        _ => NumberType::Complex
    }
}