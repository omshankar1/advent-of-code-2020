use regex::Regex;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

#[derive(Debug)]
struct Password {
    policy: Policy,
    password: String,
}

impl Password {
    pub fn is_valid(&self) -> bool {
        true
    }
    pub fn numchar_occurrences(&self) -> usize {
        let letter = self.policy.letter;
        self.password.chars().filter(|c| *c == letter).count() as usize
    }
    pub fn char_validrange(&self) -> bool {
        let min = self.policy.min;
        let max = self.policy.max;
        let letter = self.policy.letter;
        let num = self.password.chars().filter(|c| *c == letter).count() as usize;
        num <= max && num >= min
    }
    pub fn char_validposition(&self) -> bool {
        let min = self.policy.min;
        let max = self.policy.max;
        let letter = self.policy.letter;
        let cmin = self.password.chars().nth(min - 1).unwrap();
        let cmax = self.password.chars().nth(max - 1).unwrap();
        // Exclusive OR
        (cmin == letter) ^ (cmax == letter)
    }
}

fn count_valid_password(password: Vec<Password>) -> usize {
    password.into_iter().fold(0, |mut acc, passwd| {
        if passwd.char_validrange() {
            acc += 1;
        }
        acc
    })
}
fn count_valid_position(password: Vec<Password>) -> usize {
    password.into_iter().fold(0, |mut acc, passwd| {
        if passwd.char_validposition() {
            acc += 1;
        }
        acc
    })
}

pub fn problem1(input: &str) -> usize {
    let password = parser(input);
    println!("{}", password.len());
    // let count = count_valid_password(password);
    let count = count_valid_password(password);
    println!("count: {}", count);
    count
}

pub fn problem2(input: &str) -> usize {
    let password = parser(input);
    println!("{}", password.len());
    // let count = count_valid_password(password);
    let count = count_valid_position(password);
    println!("count: {}", count);
    count
}

fn parser(input: &str) -> Vec<Password> {
    let re =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)\n?$").unwrap();

    let v = input
        .trim_end()
        .split('\n')
        .map(|line| match re.captures_iter(line).next() {
            Some(cap) => {
                match (
                    cap.name("min"),
                    cap.name("max"),
                    cap.name("letter"),
                    cap.name("password"),
                ) {
                    (Some(min), Some(max), Some(letter), Some(password)) => {
                        let min = min.as_str().parse::<usize>().unwrap();
                        let max = max.as_str().parse::<usize>().unwrap();
                        let letter = letter.as_str().parse::<char>().unwrap();
                        let password = password.as_str().to_string();

                        Password {
                            policy: Policy { min, max, letter },
                            password,
                        }
                    }
                    _ => panic!("Invalid format: {:?}", cap),
                }
            }
            None => panic!("non matching line: {}", line),
        })
        .collect::<Vec<_>>();
    // println!("{:?}", v);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_problem1() {
        let input = "
                1-3 a: abcde
                1-3 b: cdefg
                2-9 c: ccccccccc
                12-19 d: cccddd
           ";
        let input = std::fs::read_to_string("inputs/a02").unwrap();
        problem1(&input);
        problem2(&input);
    }
}
