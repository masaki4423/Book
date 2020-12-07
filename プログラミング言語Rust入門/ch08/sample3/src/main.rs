use std::num::ParseIntError;
type Result<T> = std::result::Result<T, ParseIntError>;

fn main() {
    let r = "100".parse::<i32>();
    match r {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error: {:?}", e),
    }

    let r = "xxx".parse::<i32>();
    match r {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error: {:?}", e),
    }

    let n = "100".parse::<i32>().unwrap();
    println!("n is {}", n);

    // let n = "xxx".parse::<i32>().unwrap();
    // println!("n is {}", n);

    // let n: i32 = half_number("100");
    // println!("n is {}", n);

    // let n: i32 = half_number("xxx");
    // println!("n is {}", n);

    // match half_number("100") {
        // Ok(n) => println!("Ok: {}", n),
        // Err(err) => println!("Error: {:?}", err),
    // }

    // match half_number("xxx") {
        // Ok(n) => println!("Ok: {}", n),
        // Err(err) => println!("Error: {:?}", err),
    // }

    match half_number("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}

// fn half_number(s: &str) -> Result<i32, ParseIntError> {
    // match s.parse::<i32>() {
        // Ok(n) => Ok(n / 2),
        // Err(err) => Err(err),
    // }
// }

fn half_number(s: &str) -> Result<i32> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err(err),
    }
}
