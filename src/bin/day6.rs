use std::hash::Hash;
use std::{collections::HashSet, error::Error};
use std::time::Instant;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }
    }

    fn move_one(&mut self) {
        self.position = self.get_next_move_position()
    }

    fn get_next_move_position(&self) -> (i32, i32) {
        let x = self.position.0;
        let y = self.position.1;
        match self.direction {
            Direction::North => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
        }
    }

    fn next_position_is_within_area(&self, problem_area: &ProblemArea) -> bool {
        let x = self.get_next_move_position().0;
        let y = self.get_next_move_position().1;
        x >= 0 && x < problem_area.size.0 && y >= 0 && y < problem_area.size.1
    }
}

#[derive(Debug)]
struct Obstacle {
    position: (i32, i32),
}

#[derive(Debug)]
struct ProblemArea {
    size: (i32, i32),
}

fn extract_elements_from_string(
    data: String,
) -> Result<(ProblemArea, Guard, Vec<Obstacle>), Box<dyn Error>> {
    let data_as_matrix = data
        .lines()
        .map(|row| row.chars().filter(|&c| c != ' ').collect())
        .collect::<Vec<Vec<char>>>();

    let problem_area = ProblemArea {
        size: (data_as_matrix.len() as i32, data_as_matrix[0].len() as i32),
    };
    let mut obstacles: Vec<Obstacle> = vec![];
    let mut guard: Option<Guard> = None;

    for (inverted_y, row) in data_as_matrix.iter().enumerate() {
        let y = problem_area.size.1 - inverted_y as i32 - 1;
        for (x, element) in row.iter().enumerate() {
            match element {
                '#' => obstacles.push(Obstacle {
                    position: (x as i32, y as i32),
                }),
                '^' => {
                    guard = Some(Guard {
                        position: (x as i32, y as i32),
                        direction: Direction::North,
                    })
                }
                _ => {}
            }
        }
    }

    match guard {
        Some(g) => return Ok((problem_area, g, obstacles)),
        _ => return Err("Guard was not found in date.".into()),
    }
}

fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day6.txt")?)
}

fn get_all_positions_visited_by_guard(    
    mut guard: Guard,
    problem_area: &ProblemArea,
    obstacles: &Vec<Obstacle>,
) -> HashSet<(i32, i32)> {
    let mut positions_visited: HashSet<(i32, i32)> = HashSet::new();
    while guard.next_position_is_within_area(problem_area) {
        let guards_next_position = guard.get_next_move_position();
        let guard_has_hit_obstacle = obstacles
            .iter()
            .any(|obstacle| obstacle.position == guards_next_position);
        if guard_has_hit_obstacle {
            guard.turn_right();
        } else {
            guard.move_one();
            positions_visited.insert(guard.position);
        }
    }
    positions_visited
}

fn part1(
    guard: Guard,
    problem_area: &ProblemArea,
    obstacles: &Vec<Obstacle>,
) -> Result<i32, Box<dyn Error>> {
    let guard_initial_position = guard.position;
    let mut positions_visited = get_all_positions_visited_by_guard(guard, problem_area, obstacles);
    positions_visited.insert(guard_initial_position);

    Ok(positions_visited.len() as i32)
}

fn does_guard_loops_forever(
    mut guard: Guard,
    problem_area: &ProblemArea,
    obstacles: &Vec<Obstacle>,
    additional_obstacle: Obstacle,
) -> bool {
    let mut positions_and_direction_visited = HashSet::from([(guard.position, guard.direction)]);

    while guard.next_position_is_within_area(problem_area) {
        let guards_next_position = guard.get_next_move_position();
        let guard_has_hit_obstacle = obstacles
            .iter()
            .any(|obstacle| obstacle.position == guards_next_position);
        if guard_has_hit_obstacle || additional_obstacle.position == guards_next_position {
            guard.turn_right();
        } else {
            guard.move_one();

            let guard_has_reached_previous_position_and_direction =
                positions_and_direction_visited.contains(&(guard.position, guard.direction));
            if guard_has_reached_previous_position_and_direction {
                return true;
            }
        }
        positions_and_direction_visited.insert((guard.position, guard.direction));
    }

    return false;
}

fn part2(
    guard: Guard,
    problem_area: &ProblemArea,
    obstacles: &Vec<Obstacle>,
) -> Result<i32, Box<dyn Error>> {
    let possible_new_obstacle_positions = get_all_positions_visited_by_guard(guard.clone(), problem_area, obstacles);
    
    let sum = possible_new_obstacle_positions
        .par_iter()
        .filter(|(x, y)| {
            let additional_obstacle = Obstacle { position: (*x, *y) };
            does_guard_loops_forever(
                guard.clone(),
                problem_area,
                obstacles,
                additional_obstacle,
            )
        }).count();

    Ok(sum as i32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let (problem_area, guard, obstacles) = extract_elements_from_string(data)?;

    let result_part1 = part1(guard.clone(), &problem_area, &obstacles)?;
    let start = Instant::now();
    let result_part2 = part2(guard.clone(), &problem_area, &obstacles)?;
    println!("{:?}", start.elapsed());

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data_part1() -> String {
        "
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#..."
            .trim_start_matches("\n")
            .replace(" ", "")
            .to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data_part1();

        let (problem_area, guard, obstacles) = extract_elements_from_string(data_as_string)?;

        let result = part1(guard, &problem_area, &obstacles)?;

        println!("{result}");
        assert!(result == 41);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data_part1();

        let (problem_area, guard, obstacles) = extract_elements_from_string(data_as_string)?;

        let result = part2(guard, &problem_area, &obstacles)?;

        println!("{result}");
        assert!(result == 6);
        Ok(())
    }
}
