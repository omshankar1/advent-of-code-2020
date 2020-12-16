pub struct Forest(Vec<Vec<char>>);

impl Forest {
    pub fn new(input: &str) -> Self {
        let v = input
            .split('\n')
            .map(|s| s.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        Self(v)
    }
    fn char_at_col(&self, row: usize, col: usize) -> char {
        let line = self.0.get(row).unwrap();
        let len = line.len();
        line[col % len]
    }
    pub fn traverse1(&self, skip_right: usize, skip_down: usize) -> usize {
        let mut acc = 0;
        let num_rows = self.0.len();
        for row in (1..(num_rows - 1)).step_by(skip_down) {
            let col = row * skip_right;
            let ch = self.char_at_col(row, col);
            if ch == '#' {
                acc += 1;
            }
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_problem1() {
        let input = std::fs::read_to_string("inputs/a03_sample").unwrap();
        println!("{}", input);
        let jungle: Forest = Forest::new(&input);
        let a03_1 = jungle.traverse1(3, 1);
        assert_eq!(a03_1, 164);
    }
}
