

fn five() -> i32 {
    5
}

fn main(){
    let x = five();
    println!("The value of x is: {}", x);
}





// // Calling a function or a macro is an expression. " {} " is also an expression.
// fn main() {
//     let _x = 5;

//     let y = {
//         let x= 3;
//         // If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. 
//         x + 1
//     };
//     println!("The value of y is: {}", y)
// }





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
