use std::collections::HashSet;
use std::ops::Add;
use std::{error::Error, num::ParseIntError};

fn height_map_from_string(data: String) -> Result<HeightMap, ParseIntError> {
    let heights: Vec<Vec<i32>> = data
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_string().parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>()
        })
        .rev()
        .collect::<Result<Vec<Vec<i32>>, ParseIntError>>()?;

    Ok(HeightMap { heights })
}

fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day10.txt")?)
}

struct HeightMap {
    heights: Vec<Vec<i32>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl HeightMap {
    fn get_trail_heads(&self) -> Vec<Position> {
        let mut trail_heads: Vec<Position> = Vec::new();
        for (y, row) in self.heights.iter().enumerate() {
            for (x, height) in row.iter().enumerate() {
                if *height == 0 {
                    trail_heads.push(Position {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        trail_heads
    }

    fn step_one_up_from_position(&self, position: Position) -> Vec<Position> {
        let possible_directions = [
            Position { x: 1, y: 0 },
            Position { x: 0, y: -1 },
            Position { x: -1, y: 0 },
            Position { x: 0, y: 1 },
        ];
        let old_height = self.get_height(position);

        possible_directions
            .iter()
            .map(|&p| position + p)
            .filter(|&p| self.is_position_within_map(p))
            .filter(|&p| self.get_height(p) == old_height + 1)
            .collect()
    }

    fn get_height(&self, position: Position) -> i32 {
        self.heights[position.y as usize][position.x as usize]
    }

    fn is_position_within_map(&self, position: Position) -> bool {
        let length_x = self.heights[0].len() as i32;
        let length_y = self.heights.len() as i32;
        let x = position.x;
        let y = position.y;
        x >= 0 && x < length_x && y >= 0 && y < length_y
    }

    fn number_of_ways_to_a_top(&self, position: Position) -> i32 {
        match self.get_height(position) {
            9 => 1,
            _ => self
                .step_one_up_from_position(position)
                .iter()
                .map(|&p| self.number_of_ways_to_a_top(p))
                .sum(),
        }
    }
}

fn part1(height_map: &HeightMap) -> Result<i32, Box<dyn Error>> {
    let mut score = 0;

    for trail_heads in height_map.get_trail_heads() {
        let mut current_points = HashSet::new();
        current_points.insert(trail_heads);

        for _ in 0..9 {
            current_points = current_points
                .iter()
                .flat_map(|&p| height_map.step_one_up_from_position(p))
                .collect();
        }
        score += current_points.len() as i32
    }
    Ok(score)
}

fn part2(height_map: &HeightMap) -> Result<i32, Box<dyn Error>> {
    let mut score = 0;
    for trail_heads in height_map.get_trail_heads() {
        score += height_map.number_of_ways_to_a_top(trail_heads);
    }
    Ok(score)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let height_map = height_map_from_string(data)?;

    let result_part1 = part1(&height_map)?;
    let result_part2 = part2(&height_map)?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"
            .trim_start_matches("\n")
            .replace("    ", "")
            .to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();

        let height_map = height_map_from_string(data_as_string)?;

        let result = part1(&height_map)?;

        println!("{}", result);
        assert!(result == 36);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();

        let height_map = height_map_from_string(data_as_string)?;

        let result = part2(&height_map)?;

        println!("{}", result);
        assert!(result == 81);
        Ok(())
    }
}
