use std::collections::HashSet;

pub fn parser(input: &str) -> HashSet<i64> {
    input
        .split('\n')
        // .map(|s| {
        //     println!("{:?}", s);
        //     s
        // })
        .map(|s| s.trim().parse::<i64>())
        .filter_map(Result::ok)
        .collect::<HashSet<_>>()
}

fn find_numbers(set: &HashSet<i64>, n3: i64) -> Option<(i64, i64)> {
    for n1 in set.iter() {
        // println!("{:?}", n1);
        let n2 = 2020 - n1 - n3;
        if set.contains(&n2) {
            // println!("{} * {} = {:?}", n1, n2, n1 * n2);
            return Some((*n1, n2));
        }
    }
    None
}

// pub fn problem1(set: HashSet<i64>) -> i64 {
pub fn problem1(input: &str) -> Option<i64> {
    let set = parser(input);
    if let Some((n1, n2)) = find_numbers(&set, 0) {
        return Some(n1 * n2);
    }
    None
}

pub fn problem2(input: &str) -> Option<i64> {
    let set = parser(input);
    for n3 in set.iter() {
        if let Some((n1, n2)) = find_numbers(&set, *n3) {
            return Some(n1 * n2 * n3);
        }
    }
    None
}

// fn find_pairs(set: &HashSet<i64>) -> Option<(i64, i64)> {
//     for n1 in set.iter() {
//         // println!("{:?}", n1);
//         let n2 = 2020 - n1;
//         if set.contains(&n2) {
//             // println!("{} * {} = {:?}", n1, n2, n1 * n2);
//             return Some((*n1, n2));
//         }
//     }
//     None
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = " 1721
                    979
                    366
                    299
                    675
                    1456 ";
        problem1(input);

        assert_eq!(problem1(input), Some(514579));
    }
    #[test]
    fn test_input_problem1() {
        let input = std::fs::read_to_string("inputs/a01").unwrap();
        assert_eq!(problem1(&input), Some(1006176));
    }

    #[test]
    fn test_input_problem2() {
        let input = std::fs::read_to_string("inputs/a01").unwrap();
        assert_eq!(problem2(&input), Some(199132160));
    }
}
