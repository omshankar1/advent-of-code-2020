use std::fs;
mod a01_1;
mod a02_1;

fn main() {
    let input = std::fs::read_to_string("inputs/a01").unwrap();
    let a01 = a01_1::problem1(&input[..]);
    println!("{:?}", a01);
    let a01 = a01_1::problem2(&input[..]);
    println!("{:?}", a01);
    let input = std::fs::read_to_string("inputs/a02").unwrap();
    let a02 = a02_1::problem1(&input[..]);
    println!("{:?}", a01);
    let a02 = a02_1::problem2(&input[..]);
    println!("{:?}", a01);
}
