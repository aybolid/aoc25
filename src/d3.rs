#[derive(Debug)]
pub struct BatteriesBank {
    joltage: Vec<u8>,
}

impl BatteriesBank {
    pub fn max_joltage(&self, for_n_batteries: usize) -> usize {
        assert!(for_n_batteries <= self.joltage.len());

        let mut stack: Vec<u8> = Vec::new();
        let mut to_remove = self.joltage.len().saturating_sub(for_n_batteries);

        for digit in self.joltage.iter().copied() {
            while let Some(&top) = stack.last() {
                if to_remove > 0 && digit > top {
                    stack.pop();
                    to_remove -= 1;
                } else {
                    break;
                }
            }
            stack.push(digit);
        }
        stack.truncate(for_n_batteries);
        assert_eq!(stack.len(), for_n_batteries);

        let result_str: String = stack.iter().map(|d| d.to_string()).collect();
        result_str.parse::<usize>().unwrap_or(0)
    }
}

impl std::str::FromStr for BatteriesBank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty input string".to_string());
        };

        let trimmed = s.trim();
        let mut joltage = vec![0; trimmed.len()];

        for (idx, char) in trimmed.chars().enumerate() {
            if !char.is_ascii_digit() {
                return Err("input must contain only ascii digits".to_string());
            }
            joltage[idx] = (char as u8) - b'0'
        }

        Ok(Self { joltage })
    }
}
