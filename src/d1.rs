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
    const MOD: usize = MAX + 1;

    pub const fn new(points_at: usize) -> Self {
        assert!(points_at < Self::MOD);
        Self { points_at }
    }

    pub fn points_at(&self) -> usize {
        self.points_at
    }

    pub fn rotate(&mut self, rotation: DialRotation) {
        match rotation {
            DialRotation::Right(n) => {
                self.points_at = (self.points_at + n) % Self::MOD;
            }
            DialRotation::Left(n) => {
                self.points_at = (self.points_at + Self::MOD - (n % Self::MOD)) % Self::MOD;
            }
        }
    }

    pub fn rotate_with_wraps_count(&mut self, rotation: DialRotation) -> usize {
        let before = self.points_at;
        self.rotate(rotation);

        match rotation {
            DialRotation::Right(n) => {
                let full_wraps = n / Self::MOD;
                let additional_wrap = ((before + (n % Self::MOD)) >= Self::MOD) as usize;
                full_wraps + additional_wrap
            }
            DialRotation::Left(n) => {
                let full_wraps = n / Self::MOD;
                let additional_wrap = (before < (n % Self::MOD)) as usize;
                full_wraps + additional_wrap
            }
        }
    }
}
