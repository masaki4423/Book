fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("v[0] is {}", v[0]);
    println!("v[4] is {}", v[4]);
    println!("v.len is {}", v.len());

    let v = vec![1, 2, 3, 4, 5];
    println!("v.get(0) is {:?}", v.get(0));
    println!("v.get(4) is {:?}", v.get(4));
    println!("v.get(0) is {}", v.get(0).unwrap());
    println!("v.get(4) is {}", v.get(4).unwrap());

    let v = vec!["one", "two", "three", "four", "five"];
    println!("v[0] is {}", v[0]);
    println!("v[4] is {}", v[4]);
    println!("v.len is {}", v.len());
}
