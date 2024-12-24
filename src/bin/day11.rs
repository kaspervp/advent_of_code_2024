use cached::cached;
use std::{error::Error, num::ParseIntError};

fn stone_numbers_from_string(data: String) -> Result<Vec<i64>, ParseIntError> {
    let stone_numbers: Vec<i64> = data
        .split(" ")
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, ParseIntError>>()?;

    Ok(stone_numbers)
}

fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day11.txt")?)
}

cached! {
    FIB: cached::UnboundCache<(Vec<i64>, i32), i64> = cached::UnboundCache::new();
    fn number_of_stones_after_blinks(stone_numbers: Vec<i64>, remaining_blinks: i32) -> i64 = {
        if remaining_blinks == 0 {
            return stone_numbers.len() as i64
        }

        let mut number_of_stones = 0;
        for stone_num in stone_numbers {
            match stone_num {
                0 => {
                    number_of_stones += number_of_stones_after_blinks(vec![1], remaining_blinks-1);
                },
                _ if stone_num.to_string().len() % 2 == 0 => {
                    let half_character_length = (stone_num.to_string().len() as i64) / 2;
                    let number_1 = stone_num / 10_i64.pow(half_character_length as u32);
                    let number_2 = stone_num - number_1 * 10_i64.pow(half_character_length as u32);
                    number_of_stones += number_of_stones_after_blinks(vec![number_1, number_2], remaining_blinks - 1);
                },
                _ => {
                    number_of_stones += number_of_stones_after_blinks(vec![2024 * stone_num], remaining_blinks - 1);
                }
            };
        };
        number_of_stones
    }
}

fn part1(stone_numbers: Vec<i64>) -> Result<i64, Box<dyn Error>> {
    Ok(number_of_stones_after_blinks(stone_numbers, 25))
}

fn part2(stone_numbers: Vec<i64>) -> Result<i64, Box<dyn Error>> {
    Ok(number_of_stones_after_blinks(stone_numbers, 75))
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let stone_numbers = stone_numbers_from_string(data)?;

    let result_part1 = part1(stone_numbers.clone())?;
    let result_part2 = part2(stone_numbers.clone())?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        "125 17".to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();

        let stone_numbers = stone_numbers_from_string(data_as_string)?;

        let result = part1(stone_numbers)?;

        println!("{}", result);
        assert!(result == 55312);
        Ok(())
    }
}
