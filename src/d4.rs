#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapCell {
    Empty,
    PaperRoll,
}

#[derive(Debug)]
pub struct PaperRollsMap {
    size: (usize, usize),
    data: Vec<MapCell>,
}

#[rustfmt::skip]
const NEIGHBORS_TO_CHECK: &[(isize, isize)] = &[
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0),           ( 1,  0),
    (-1,  1), ( 0,  1), ( 1,  1),
];

impl PaperRollsMap {
    pub fn count_accessible_paper_rolls_with_removal(&self) -> usize {
        let mut cloned_data = self.data.clone();

        let mut count = 0;
        let mut has_changes = true;

        while has_changes {
            has_changes = false;

            for idx in 0..cloned_data.len() {
                if cloned_data[idx] != MapCell::PaperRoll {
                    continue;
                }

                let xy = self.idx_to_xy(idx);
                let mut neighbors = 0;

                for (dx, dy) in NEIGHBORS_TO_CHECK {
                    let nx = xy.0 + dx;
                    let ny = xy.1 + dy;

                    let nx_in_bounds = nx >= 0 && nx < self.size.0 as isize;
                    let ny_in_bounds = ny >= 0 && ny < self.size.1 as isize;

                    if nx_in_bounds && ny_in_bounds {
                        let nidx = self.xy_to_idx((nx, ny));
                        if cloned_data[nidx] == MapCell::PaperRoll {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    count += 1;
                    cloned_data[idx] = MapCell::Empty;
                    has_changes = true;
                }
            }
        }

        count
    }

    pub fn count_accessible_paper_rolls(&self) -> usize {
        let mut count = 0;

        for idx in 0..self.data.len() {
            if self.data[idx] != MapCell::PaperRoll {
                continue;
            }
            let xy = self.idx_to_xy(idx);

            let mut neighbors = 0;
            for (dx, dy) in NEIGHBORS_TO_CHECK {
                let nx = xy.0 + dx;
                let ny = xy.1 + dy;

                let nx_in_bounds = nx >= 0 && nx < self.size.0 as isize;
                let ny_in_bounds = ny >= 0 && ny < self.size.1 as isize;

                if nx_in_bounds && ny_in_bounds {
                    let nidx = self.xy_to_idx((nx, ny));
                    if self.data[nidx] == MapCell::PaperRoll {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                count += 1;
            }
        }

        count
    }

    fn idx_to_xy(&self, idx: usize) -> (isize, isize) {
        assert!(idx < self.data.len());
        ((idx % self.size.0) as isize, (idx / self.size.0) as isize)
    }

    fn xy_to_idx(&self, xy: (isize, isize)) -> usize {
        (xy.1 * self.size.0 as isize + xy.0) as usize
    }
}

impl std::str::FromStr for PaperRollsMap {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty input string".to_string());
        }
        let s = s.trim();

        let mut data = vec![];
        let mut size = (0, 0);

        for line in s.lines() {
            if size.0 == 0 {
                size.0 = line.len();
            };
            size.1 += 1;

            for char in line.chars() {
                match char {
                    '@' => data.push(MapCell::PaperRoll),
                    '.' => data.push(MapCell::Empty),
                    c => return Err(format!("unknown map symbol: {:?}", c)),
                }
            }
        }

        Ok(Self { size, data })
    }
}
