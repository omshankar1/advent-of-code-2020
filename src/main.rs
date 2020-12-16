use std::fs;
mod a01_1;
mod a02_1;
mod a03_1;
use a03_1::Forest;

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
    let input = std::fs::read_to_string("inputs/a03").unwrap();
    let jungle: Forest = Forest::new(&input);
    let a03_1 = jungle.traverse1(1, 1);
    let a03_3 = jungle.traverse1(3, 1);
    let a03_5 = jungle.traverse1(5, 1);
    let a03_7 = jungle.traverse1(7, 1);
    let a03_1_2 = jungle.traverse1(1, 2);
    println!("{}", a03_1 * a03_3 * a03_5 * a03_7 * a03_1_2);
}
