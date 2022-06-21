fn main() {
    println!("Hello, world!");
}



#[test]
fn test_numbers() {
    // literals
    // hex
    assert_eq!(0xff, 255);

    // Binary
    assert_eq!(0b0010_1010, 42);

    // base 8
    assert_eq!(0o106, 70);

    // Char codes
    assert_eq!(b'A', 65u8);
}

#[test]
fn test_arrays() {
    let numbers: [u32; 6] = [1,2,3,4,5,6];
    let legends = ["Pathfinder", "Ash", "White Castle"];

    assert_eq!(numbers[2], 3);
    assert_eq!(legends.len(), 3);

    // an array 1024 elements long all filled with 0
    let kilobyte_buffer = [0u8; 1024];

    let mut chaos = [4, 8, 3 , 0];
    chaos.sort();
    assert_eq!(chaos, [0,3,4,8]);
}

#[test]
fn test_vectors() {
    let mut primes = vec![2,3,5,7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);

    assert_eq!(primes.iter().product::<i32>(), 30030);

    let pixles = vec![0; 1080 * 1920];

    // let list = vec![...];
    // is the same as
    // let list = Vec::new();
    // list.push(...);

}

#[test]
fn test_slices() {
    // a slice is a "fat pointer" made up of a pointer to the slices first element and the number of elements in a slice.

}