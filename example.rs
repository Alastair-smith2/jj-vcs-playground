#[derive(Debug)]
struct Example {
    fieldOne: bool,
    fieldTwo: String,
    fieldThree: u32
}

fn main() {
    let val = 5;
    println!("Hello alpha");
    let another_val = 10;
    let something_else = 15;
    let another_value = 20;
    let val = 20;
    let test = 25;
    let example = Example {fieldOne: false, fieldTwo: String::from("Hey"), fieldThree: 5};
    println!("No formatter makes the syntax check harder :) {:?}", example);
    let some_example = Example {fieldOne: true, fieldTwo: String::from("King Boo"), fieldThree: 10};
    let another_example = String::from("A new world");
}