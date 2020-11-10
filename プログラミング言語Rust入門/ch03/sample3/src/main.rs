fn main() {
    // closure
    let num = 10;
    let add_one = |x| { num + x };
    let add_two = |x, y| { x + y };

    let ans = add_one(1);
    println!("ans is {}", ans);
    let ans = add_two(10, 20);
    println!("ans is {}", ans);
}
