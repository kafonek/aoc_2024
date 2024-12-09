use log::debug;

#[derive(Debug)]
struct DiskState {
    blocks: Vec<Option<usize>>,
}

impl DiskState {
    fn from_str(input: &str) -> Self {
        let numbers: Vec<usize> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let mut blocks = Vec::new();
        let mut file_id = 0;
        let mut is_file = true;

        for &length in &numbers {
            if is_file {
                for _ in 0..length {
                    blocks.push(Some(file_id));
                }
                file_id += 1;
            } else {
                for _ in 0..length {
                    blocks.push(None);
                }
            }
            is_file = !is_file;
        }

        DiskState { blocks }
    }

    fn compact_step(&mut self) -> bool {
        // Find rightmost file
        let mut rightmost_file_pos = None;
        for (i, block) in self.blocks.iter().enumerate().rev() {
            if block.is_some() {
                rightmost_file_pos = Some(i);
                break;
            }
        }

        if let Some(file_pos) = rightmost_file_pos {
            // Find leftmost free space
            let mut leftmost_space = None;
            for i in 0..file_pos {
                if self.blocks[i].is_none() {
                    leftmost_space = Some(i);
                    break;
                }
            }

            if let Some(space_pos) = leftmost_space {
                // Move file block left
                self.blocks[space_pos] = self.blocks[file_pos];
                self.blocks[file_pos] = None;
                return true;
            }
        }

        false
    }

    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(pos, block)| block.map(|file_id| pos as u64 * file_id as u64))
            .sum()
    }

    #[cfg(debug_assertions)]
    fn debug_print(&self) {
        let state: String = self
            .blocks
            .iter()
            .map(|b| match b {
                None => '.',
                Some(id) => std::char::from_digit(*id as u32, 10).unwrap_or('X'),
            })
            .collect();
        debug!("State: {}", state);
    }

    fn compact_whole_files(&mut self) {
        // Get the highest file ID
        let max_file_id = self.blocks.iter().filter_map(|&b| b).max().unwrap_or(0);

        // Iterate over file IDs in decreasing order
        for file_id in (0..=max_file_id).rev() {
            // Find the start and end of the current file
            let mut file_start = None;
            let mut file_end = None;

            for (i, &block) in self.blocks.iter().enumerate() {
                if block == Some(file_id) {
                    if file_start.is_none() {
                        file_start = Some(i);
                    }
                    file_end = Some(i);
                }
            }

            if let (Some(start), Some(end)) = (file_start, file_end) {
                let file_length = end - start + 1;

                // Find the leftmost free space large enough for the file
                let mut space_start = None;
                let mut space_count = 0;

                for i in 0..start {
                    if self.blocks[i].is_none() {
                        if space_start.is_none() {
                            space_start = Some(i);
                        }
                        space_count += 1;
                    } else {
                        space_start = None;
                        space_count = 0;
                    }

                    if space_count >= file_length {
                        // Move the whole file
                        let start_pos = space_start.unwrap();
                        for j in 0..file_length {
                            self.blocks[start_pos + j] = Some(file_id);
                            self.blocks[start + j] = None;
                        }
                        break;
                    }
                }
            }
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let content = std::fs::read_to_string(input).unwrap();
    let mut state = DiskState::from_str(&content);
    debug!("Initial state: {:?}", state);

    while state.compact_step() {
        #[cfg(debug_assertions)]
        state.debug_print();
    }

    state.checksum()
}

pub fn part2(input: &str) -> u64 {
    let content = std::fs::read_to_string(input).unwrap();
    let mut state = DiskState::from_str(&content);
    debug!("Initial state: {:?}", state);

    state.compact_whole_files();

    #[cfg(debug_assertions)]
    state.debug_print();

    state.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/09_sample.txt"), 1928);
        assert_eq!(part1("data/09.txt"), 6331212425418);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/09_sample.txt"), 2858);
        assert_eq!(part2("data/09.txt"), 6363268339304);
    }
}
