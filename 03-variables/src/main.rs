// Fix the error below with least amount of modification to the code
/*
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
*/

/*
// Fill the blanks in the code to make it compile
fn main() {
    let mut x:i32 = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}
*/

/*
// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    let y: i32 = 15;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y); 
}

fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    println!("Inner scope value of x is {} and value of y is {}", x, y);
    println!("Outer scope value of x is {} and value of y is {}", x, y); 
}
*/

/*
// Fix the error with the use of define_x
fn main() {
    println!("{}, world", define_x());
}

/*
fn define_x() -> &'static str {
    let x = "hello";
    x
}
*/

fn define_x() -> String {
    let x = "hello";
    x.to_string()
}
*/

/*
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}
*/

/*
// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;
    // let mut x = x;
    // x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}
*/

/*
fn main() {
    let _x = 1; 
}
*/

/*
#[allow(unused_variables)]
fn main() {
    let _x = 1;
}
*/

/*
// Fix the error below with least amount of modification
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
*/

/*
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}
*/
