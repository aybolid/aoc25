#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dial<const MAX: usize> {
    points_at: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialRotation {
    Left(usize),
    Right(usize),
}

impl std::str::FromStr for DialRotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("input string is empty".to_string());
        }
        let (dir, n_str) = s.split_at(1);
        let n = n_str.parse::<usize>().map_err(|err| err.to_string())?;
        match dir {
            "L" => Ok(Self::Left(n)),
            "R" => Ok(Self::Right(n)),
            _ => Err(format!("invalid direction: {}", dir)),
        }
    }
}

impl<const MAX: usize> Dial<MAX> {
    const MODULUS: usize = MAX + 1;

    pub const fn new(points_at: usize) -> Self {
        assert!(points_at <= MAX);
        Self { points_at }
    }

    pub fn points_at(&self) -> usize {
        self.points_at
    }

    pub fn rotate(&mut self, rotation: DialRotation) {
        match rotation {
            DialRotation::Left(n) => self.wrapping_sub(n),
            DialRotation::Right(n) => self.wrapping_add(n),
        }
    }

    pub fn rotate_with_wraps_count(&mut self, rotation: DialRotation) -> usize {
        match rotation {
            DialRotation::Left(n) => self.wrapping_sub_with_wraps_count(n),
            DialRotation::Right(n) => self.wrapping_add_with_wraps_count(n),
        }
    }

    fn wrapping_add(&mut self, n: usize) {
        let effective_add = n % Self::MODULUS;
        self.points_at = (self.points_at + effective_add) % Self::MODULUS;
    }

    fn wrapping_sub(&mut self, n: usize) {
        let effective_sub = n % Self::MODULUS;
        if effective_sub > 0 {
            let effective_add = Self::MODULUS - effective_sub;
            self.points_at = (self.points_at + effective_add) % Self::MODULUS;
        }
    }

    fn wrapping_add_with_wraps_count(&mut self, n: usize) -> usize {
        let full_wraps = n / Self::MODULUS;
        let remainder = n % Self::MODULUS;

        let extra_wrap = ((self.points_at + remainder) >= Self::MODULUS) as usize;

        self.wrapping_add(n);
        full_wraps + extra_wrap
    }

    fn wrapping_sub_with_wraps_count(&mut self, n: usize) -> usize {
        let full_wraps = n / Self::MODULUS;
        let remainder = n % Self::MODULUS;
        let remainder_wrap = (self.points_at < remainder) as usize;

        self.wrapping_sub(n);
        full_wraps + remainder_wrap
    }
}
