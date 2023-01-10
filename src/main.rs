

// Calling a function or a macro is an expression. " {} " is also an expression.
fn main() {
    let _x = 5;

    let y = {
        let x= 3;
        x + 1
    };
    println!("The value of y is: {}", y)
}





// Function parameters
//
// fn main() {
//     println!("Hello, world!");

//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }
