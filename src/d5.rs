use crate::d2::IdRange;

#[derive(Debug)]
pub struct Database {
    fresh_ids: Vec<IdRange>,
    ids: Vec<usize>,
}

impl Database {
    pub fn count_fresh_ids(&self) -> usize {
        if self.fresh_ids.is_empty() {
            return 0;
        }

        let mut ranges = self.fresh_ids.clone();
        ranges.sort_by_key(|r| r.0);

        let mut total = 0;
        let mut current_start = ranges[0].0;
        let mut current_end = ranges[0].1;

        for range in ranges.into_iter().skip(1) {
            if range.0 <= current_end + 1 {
                current_end = current_end.max(range.1);
            } else {
                total += current_end - current_start + 1;
                current_start = range.0;
                current_end = range.1;
            }
        }

        total += current_end - current_start + 1;
        total
    }

    pub fn count_fresh_products(&self) -> usize {
        let mut count = 0;
        for id in &self.ids {
            for range in &self.fresh_ids {
                let in_range = *id >= range.0 && *id <= range.1;
                if in_range {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}

impl std::str::FromStr for Database {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty input string".to_string());
        };
        let Some(sep_idx) = s.find("\n\n") else {
            return Err("invalid input".to_string());
        };
        let (ranges_str, ids_str) = s.split_at(sep_idx);

        let mut ranges = vec![];
        for line in ranges_str.trim().lines() {
            ranges.push(line.parse()?);
        }

        let mut ids = vec![];
        for line in ids_str.trim().lines() {
            ids.push(line.parse::<usize>().map_err(|err| err.to_string())?);
        }

        Ok(Self {
            fresh_ids: ranges,
            ids,
        })
    }
}
