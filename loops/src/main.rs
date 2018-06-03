fn main() {
    loop {
        println!("again!");
        break;
    }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0 ;

    while index < 5 {
        println!("the vlue is: {}", a[index]);
        index = index + 1;
    }

    for el in a.iter() {
        println!("the value is: {}", el);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
