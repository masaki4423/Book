fn main() {
    let x = 4;

    let adder = |n| n + x;
    assert_eq!(adder(2), 4 + 2);

    let mut state = false;
    let mut flipflop = || {
        state = !state;
        state
    };

    assert!(flipflop());
    assert!(!flipflop());
    assert!(flipflop());

    assert!(state);

    //let b = 5;

    //let mut f = |a| a * 3 + b;
    //f = |a| a * 4 + b;

    let mut f: fn(i32) -> i32 = |n| n * 3;
    assert_eq!(f(-42), -126);

    // let x = 4;
    // f = |n| n * x;

    let v = vec!["I", "love", "Rust!"]
        .into_iter()
        .map(str::len)
        .collect::<Vec<_>>();
        // .map(|s| s.len())
        // .collect::<Vec<_>>();
    assert_eq!(v, vec![1, 4, 5]);
}