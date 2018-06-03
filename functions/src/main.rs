fn main() {
    another_function(5, 6);

    let _y = 6;  // This is a statement
    // let x = (let y = 6);  Not allowed, can't assign statements

    let _x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x : i32, y : i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x : i32) -> i32 {
    x + 1
}

// fn plus_one_wrong(x : i32) -> i32 {
//     x + 1;  Wrong, this semicolon causes the function to return ()
// }
