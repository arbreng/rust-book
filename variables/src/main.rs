fn main() {
    // let _x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;  // Not allowed, x is immutable by default.
    // println!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x =5 ;
    let x = x + 1;
    let x = x + 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let _spaces = spaces.len();

    // let _mut spaces = "    ";
    // spaces = spaces.len();  Not allowed, can't change types without let

    // Not allowed, can't infer type in this case
    //let _guess = "42".parse().expect("Not a number");
    let _guess : u32 = "42".parse().expect("Not a number");

    let _x = 2.0;
    let _y: f32 = 3.0;

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _z = ' ';
    let _heart_eyed_cat = '😻';

    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    let a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June",
                  "July", "August", "September", "Octoboer", "November",
                  "December"];
    let _first = a[0];
    let _second = a[1];
    // let index = 10;
    // let _element = a[index];  Not allowed, array out-of-bounds access
    // /println!("The value of element is {}", element);
}