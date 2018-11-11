fn main() {
   assert!(fibbo(0) == 0);
   assert!(fibbo(1) == 1);
   assert!(fibbo(2) == 1);
   assert!(fibbo(10) == 55);
   assert!(fibbo(20) == 6765);
}

fn fibbo(n: i32) -> i32 {
    assert!(n >= 0);
    if n == 1 {
        return 1;
    }
    let mut result = 0;
    let mut num1 = 0;
    let mut num2 = 1;
    for _i in 1..n {
        result = num1 + num2;
        num1 = num2;
        num2 = result;
    }
    println!("Result {}", result);
    result
}

