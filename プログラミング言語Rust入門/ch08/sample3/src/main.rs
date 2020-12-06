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

    let n = "xxx".parse::<i32>().unwrap();
    println!("n is {}", n);
}
