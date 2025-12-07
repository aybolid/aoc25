pub fn count_beam_splits(s: &str) -> usize {
    let width = s.lines().next().unwrap().len();
    let height = s.lines().count();

    let start_idx = s.find("S").unwrap();

    let bytes = s
        .as_bytes()
        .iter()
        .filter(|b| !(**b as char).is_whitespace())
        .copied()
        .collect::<Vec<_>>();

    let mut current_beam_positions = vec![(start_idx % width, start_idx / width + 1)];
    let mut next_beam_positions = vec![];

    let mut splits_count = 0;

    for y in 1..height {
        current_beam_positions.sort_unstable();
        current_beam_positions.dedup();

        for &(beam_x, _prev_beam_y) in &current_beam_positions {
            let idx = y * width + beam_x;

            if bytes[idx] == b'^' {
                splits_count += 1;

                if beam_x > 0 {
                    next_beam_positions.push((beam_x - 1, y));
                }
                if beam_x + 1 < width {
                    next_beam_positions.push((beam_x + 1, y));
                }
            } else {
                next_beam_positions.push((beam_x, y));
            }
        }

        current_beam_positions = next_beam_positions.clone();
        next_beam_positions.clear();

        if current_beam_positions.is_empty() {
            break;
        }
    }

    splits_count
}

pub fn count_timelines(s: &str) -> u64 {
    let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut counts = vec![0; width];

    let start_idx = grid[0].iter().position(|&c| c == 'S').unwrap();
    counts[start_idx] = 1;

    for y in 0..height {
        let mut next_counts = vec![0; width];

        for x in 0..width {
            if counts[x] == 0 {
                continue;
            }

            match grid[y][x] {
                '^' => {
                    if x > 0 {
                        next_counts[x - 1] += counts[x];
                    }
                    if x + 1 < width {
                        next_counts[x + 1] += counts[x];
                    }
                }
                _ => {
                    next_counts[x] += counts[x];
                }
            }
        }

        counts = next_counts;
    }

    counts.iter().sum()
}
