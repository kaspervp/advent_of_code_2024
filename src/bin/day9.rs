use std::error::Error;

use itertools::Itertools;

fn file_blocks_from_string(data: String) -> FileSystem {
    let mut data_blocks: Vec<Option<File>> = Vec::new();
    for (index, character) in data.char_indices() {
        let number = character.to_digit(10).expect("A digit 0-9.") as i32;
        if index % 2 == 0 {
            data_blocks.extend((0..number).map(|_| {
                Some(File {
                    id: (index as i32) / 2,
                })
            }));
        } else {
            data_blocks.extend((0..number).map(|_| None));
        }
    }
    FileSystem {
        memory: data_blocks,
    }
}

fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day9.txt")?)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct File {
    id: i32,
}

#[derive(Debug, Clone)]
struct FileSystem {
    memory: Vec<Option<File>>,
}

impl FileSystem {
    fn is_fully_sorted(&self) -> bool {
        let mut end_of_system = false;
        for memory_block in self.memory.iter() {
            match (end_of_system, memory_block) {
                (true, Some(_)) => return false,
                (false, None) => end_of_system = true,
                _ => {}
            }
        }
        true
    }

    fn memory_length(&self) -> usize {
        self.memory.len()
    }

    fn sort_last_file_block_to_first_empty(&mut self) -> Result<(), Box<dyn Error>> {
        let memory_length = self.memory_length();
        let index_of_first_none: Option<usize> = self
            .memory
            .iter()
            .position(|memory_block| memory_block.is_none());
        let index_of_final_file_block_from_right = self
            .memory
            .iter()
            .rev()
            .position(|memory_block| memory_block.is_some());

        match (index_of_first_none, index_of_final_file_block_from_right) {
            (Some(idx1), Some(idx2)) => {
                self.memory.swap(idx1, memory_length - idx2 - 1);
                Ok(())
            }
            _ => Err("File must contain both None and Files.".into()),
        }
    }

    fn move_whole_file_to_first_empty(&mut self, file: File) -> Result<(), Box<dyn Error>> {
        let file_memory_indices: Vec<usize> = self
            .memory
            .iter()
            .positions(|mem| *mem == Some(file))
            .collect();

        let file_size = file_memory_indices.len();
        let position_of_empty_slice = self
            .memory
            .windows(file_size)
            .position(|memory_slice| memory_slice.iter().all(|mem| mem.is_none()));

        match position_of_empty_slice {
            Some(index) if index < file_memory_indices[0] => {
                for relative_mem_index in 0..file_size {
                    self.memory.swap(
                        file_memory_indices[0] + relative_mem_index,
                        index + relative_mem_index,
                    );
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn calculate_checksum(&self) -> i64 {
        self.memory
            .iter()
            .enumerate()
            .fold(0, |sum, (index, file_block)| match file_block {
                Some(f) => sum + (f.id as i64) * (index as i64),
                None => sum,
            })
    }
}

fn part1(mut file_system: FileSystem) -> Result<i64, Box<dyn Error>> {
    while !file_system.is_fully_sorted() {
        file_system.sort_last_file_block_to_first_empty()?;
    }

    Ok(file_system.calculate_checksum())
}

fn part2(mut file_system: FileSystem) -> Result<i64, Box<dyn Error>> {
    if let Some(max_file_id) = file_system
        .memory
        .iter()
        .filter_map(|mem| *mem)
        .map(|file| file.id)
        .max()
    {
        for file_index in (0..(max_file_id + 1)).rev() {
            file_system.move_whole_file_to_first_empty(File { id: file_index })?;
        }
    }

    Ok(file_system.calculate_checksum())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let file_system = file_blocks_from_string(data);

    let result_part1 = part1(file_system.clone())?;
    let result_part2 = part2(file_system.clone())?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        "2333133121414131402".to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();

        let file_system = file_blocks_from_string(data_as_string);

        let result = part1(file_system)?;

        assert!(result == 1928);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();

        let file_system = file_blocks_from_string(data_as_string);

        let result = part2(file_system)?;

        println!("{result}");
        assert!(result == 2858);
        Ok(())
    }
}
