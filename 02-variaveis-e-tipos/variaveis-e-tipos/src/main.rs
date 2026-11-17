fn main() {
    let a = 1;
    let b = 2;

    let result = sum(a, b);
    println!("A soma de: {a} + {b} é {result}");
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}
