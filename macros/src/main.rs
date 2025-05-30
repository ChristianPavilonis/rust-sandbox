//macro_rules! generate_sum_function {
//    ($func_name:ident) => {
//        fn $func_name(a: i32, b: i32) -> i32 {
//            a + b
//        }
//    };
//}
//
//generate_sum_function!(sum);
//

// normal macros

//struct It<F: Fn() -> ()> {
//    name: String,
//    test: F,
//}
//
//impl<F: Fn() -> ()> It<F> {
//    pub fn new(name: String, test: F) -> Self {
//        Self { name, test }
//    }
//
//    pub fn run(&self) {
//        println!("Running: {}", self.name);
//        (self.test)();
//    }
//}
//
//macro_rules! it {
//    ($test_name:expr, $block:expr) => {
//        It::new(String::from($test_name), $block)
//    };
//}
//
//macro_rules! describe {
//    ($name:ident, $block:expr) => {
//        #[test]
//        fn $name() {
//            let mut tests = vec![];
//
//            $block;
//
//            for test in tests {
//                test.run()
//            }
//        }
//    }
//}
//
//fn sum(arg1: i32, arg2: i32) -> i32 {
//    arg1 + arg2
//}
//
//describe!(tests_sums, {
//
//    it!("does the thing", || {
//        assert_eq!(sum(2, 2), 4);
//    });
//
//});

// use macros::it;

use macros::show_streams;

fn main() {}

// Example: Basic function
#[show_streams(bar)]
fn invoke1() {
    println!("foo");
    let x = 0;
}

// Example: Attribute with input
// #[show_streams(bar)]
// fn invoke2() {}

// Example: Multiple tokens in the input
// #[show_streams(multiple => tokens)]
// fn invoke3() {}

// #[show_streams { delimiters }]
// fn invoke4() {}

// proc macros

// fn sum(arg1: i32, arg2: i32) -> i32 {
//     arg1 + arg2
// }
//
// it!("does the thing", {
//     assert_eq!(2 + 2, 4);
// });
