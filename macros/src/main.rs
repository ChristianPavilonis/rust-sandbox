macro_rules! generate_sum_function {
    ($func_name:ident) => {
        fn $func_name(a: i32, b: i32) -> i32 {
            a + b
        }
    };
}

generate_sum_function!(sum);


macro_rules! it {
    ($test_name:ident, $closure:expr) => {
        #[test]
        fn $test_name() {
            $closure();
        }
    };
}

fn main() {
    let result = sum(2, 3);
    println!("{}", result); // prints "5"
}



it!(works, || {
    assert!(true);
});