// A sample file for testing the code slicer.

struct MyStruct {
    field1: i32,
    field2: String,
}

fn my_function() {
    println!("This is a function.");
}

enum MyEnum {
    Variant1,
    Variant2(String),
}

// Let's extract this function
fn function_to_extract() {
    let s = MyStruct {
        field1: 10,
        field2: "hello".to_string(),
    };
    println!("Extract me!");
}
