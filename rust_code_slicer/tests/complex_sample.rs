// This is a complex sample file for testing the rust_code_slicer tool.
use std::collections::HashMap;

/// A doc comment for a struct.
#[derive(Debug, Clone)]
pub struct ComplexStruct {
    pub name: String,
    pub value: i64,
}

/// A doc comment for a trait.
pub trait DoSomething {
    fn do_it(&self) -> String;
}

// Implementation of the trait for our struct.
impl DoSomething for ComplexStruct {
    fn do_it(&self) -> String {
        format!("{} has value {}", self.name, self.value)
    }
}

// A standalone function with attributes and comments.
#[allow(unused_variables)]
pub fn standalone_function(arg1: &str) {
    // A comment inside the function.
    let x = 10;
    println!("Hello, {}!", arg1);
}

// A macro definition.
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("This is a macro!");
    };
}

// A module containing other items.
pub mod inner_module {
    // A function inside a module.
    pub fn inner_function() {
        println!("I am inside a module.");
    }

    // A nested struct.
    pub struct NestedStruct {
        pub data: Vec<u8>,
    }
}

// An enum with different kinds of variants.
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
