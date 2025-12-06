#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CephalopodMathOp {
    Mul,
    Add,
}

impl std::str::FromStr for CephalopodMathOp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty input string".to_string());
        };

        match s.trim() {
            "*" => Ok(Self::Mul),
            "+" => Ok(Self::Add),
            _ => Err("not an operator".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BadCephalopodMathSolver {
    width: usize,
    height: usize,
    numbers: Vec<usize>,

    ops: Vec<CephalopodMathOp>,
}

impl BadCephalopodMathSolver {
    pub fn grand_total(&self) -> usize {
        let mut total = 0;
        for x in 0..self.width {
            let mut numbers = Vec::with_capacity(self.height);
            for y in 0..self.height {
                numbers.push(self.numbers[y * self.width + x]);
            }

            total += match self.ops[x] {
                CephalopodMathOp::Add => numbers.iter().fold(0, |acc, n| acc + n),
                CephalopodMathOp::Mul => numbers.iter().fold(1, |acc, n| acc * n),
            };
        }
        total
    }
}

impl std::str::FromStr for BadCephalopodMathSolver {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty input string".to_string());
        };

        let mut numbers = vec![];
        let mut ops = vec![];

        let trimmed = s.trim();
        let height = trimmed.lines().count();

        for (idx, line) in trimmed.lines().enumerate() {
            if idx == height - 1 {
                for str in line.split(|c: char| c.is_whitespace()) {
                    if str.is_empty() {
                        continue;
                    }
                    ops.push(str.parse()?);
                }
            } else {
                for str in line.split(|c: char| c.is_whitespace()) {
                    if str.is_empty() {
                        continue;
                    }
                    numbers.push(str.parse::<usize>().map_err(|err| err.to_string())?);
                }
            }
        }

        Ok(Self {
            width: ops.len(),
            height: height - 1,
            numbers,
            ops,
        })
    }
}

pub fn cephalopod_grand_total(s: &str) -> usize {
    let mut total = 0;

    let height = s.lines().count();
    let width = s.lines().next().unwrap().len() + 1;
    let bytes = s.as_bytes();

    let mut numbers: Vec<usize> = vec![];

    for x in (0..width).rev() {
        let mut n_buffer = String::new();
        for y in 0..height {
            let idx = y * width + x;
            let char = bytes[idx] as char;
            if char.is_ascii_digit() {
                n_buffer.push(char);
            } else {
                if !n_buffer.is_empty() {
                    numbers.push(n_buffer.parse().unwrap());
                    n_buffer.clear();
                }
                match char {
                    '+' => {
                        total += numbers.iter().fold(0, |acc, curr| acc + curr);
                        numbers.clear();
                    }
                    '*' => {
                        total += numbers.iter().fold(1, |acc, curr| acc * curr);
                        numbers.clear();
                    }
                    _ => {}
                }
            }
        }
    }

    total
}
