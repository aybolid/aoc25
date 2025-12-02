#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdRange(usize, usize);

pub trait IdCheck {
    fn check(id: usize) -> bool;
}

#[derive(Debug)]
pub struct NoDigitSequenceRepeatedTwice;

impl IdCheck for NoDigitSequenceRepeatedTwice {
    fn check(id: usize) -> bool {
        let id_str = id.to_string();
        let len = id_str.len();
        if len % 2 != 0 {
            return true;
        }
        let (part1, part2) = id_str.split_at(len / 2);
        part1 != part2
    }
}

#[derive(Debug)]
pub struct NoDigitSequenceRepeated;

impl IdCheck for NoDigitSequenceRepeated {
    fn check(id: usize) -> bool {
        let id_str = id.to_string();
        if id_str.len() < 2 {
            return true;
        }
        let doubled = format!("{}{}", id_str, id_str);
        let substr = &doubled[1..doubled.len() - 1];
        !substr.contains(&id_str)
    }
}

impl IdRange {
    pub fn collect_invalid<C: IdCheck>(&self) -> Vec<usize> {
        (self.0..=self.1).filter(|&id| !C::check(id)).collect()
    }
}

impl std::str::FromStr for IdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty input string".to_string());
        };

        let Some((min_str, max_str)) = s.split_once("-") else {
            return Err("id range must be separated by '-'".to_string());
        };

        let min = min_str
            .trim()
            .parse::<usize>()
            .map_err(|err| err.to_string())?;
        let max = max_str
            .trim()
            .parse::<usize>()
            .map_err(|err| err.to_string())?;

        if min > max {
            return Err("min id > max id".to_string());
        };

        Ok(Self(min, max))
    }
}
