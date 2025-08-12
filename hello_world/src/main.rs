
// compiler directive applied to the entire file.
#![allow(unused_variables)]

const STORE_SPACE: i32 = 100;
type Units = i32;

fn main() {
    println!("Hello, world!");
    println!("Lets see what happens here:");
    sum_fruits();

    // compiler directive
    #[allow(unused_variables)]
    let unused_variable = "this variable its unused";
}

fn sum_fruits() {
    let apple = 5;
    let banana = 10;
    let total: Units = apple + banana;
    println!("The total space is: {STORE_SPACE}");
    println!("The total number of fruits is: {total}");
    //  STORE_SPACE = 10;
    let banana = 12;
    println!("Total number of bananas with variables shadowing {banana}");

    println!(
        "I have {0} apples and {1} bananas, I cant believe I have all of this apples {0}",
        apple, banana
    );

    un_used_variables()
}

// compiler directive applied to a function.
#[allow(unused_variables)]
fn un_used_variables() {
    let unused_variable = "this variable its unused";
    let unused_variable_two = "this variable its unused";
}
