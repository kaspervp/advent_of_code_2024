use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::error::Error;

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Antenna {
    position: (i32, i32),
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Frequency {
    symbol: char,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Antinode {
    position: (i32, i32),
}

impl Antinode {
    fn is_within_problem_area(&self, problem_area: &ProblemArea) -> bool {
        let (x, y) = self.position;
        x >= 0 && x < problem_area.size.0 && y >= 0 && y < problem_area.size.1
    }
}

#[derive(Debug, Clone, Copy)]
struct ProblemArea {
    size: (i32, i32),
}

fn extract_elements_from_string(data: String) -> (HashMap<Frequency, Vec<Antenna>>, ProblemArea) {
    let data_as_matrix = data
        .lines()
        .map(|row| row.chars().filter(|&c| c != ' ').collect())
        .collect::<Vec<Vec<char>>>();

    let problem_area = ProblemArea {
        size: (data_as_matrix[0].len() as i32, data_as_matrix.len() as i32),
    };
    let mut frequency_to_antennas_map: HashMap<Frequency, Vec<Antenna>> = HashMap::new();
    for (inverted_y, row) in data_as_matrix.iter().enumerate() {
        let y = problem_area.size.1 - inverted_y as i32 - 1;
        for (x, element) in row.iter().enumerate() {
            match element {
                '.' => {}
                _ => {
                    let frequency = Frequency { symbol: *element };
                    frequency_to_antennas_map
                        .entry(frequency)
                        .or_insert(Vec::new())
                        .push(Antenna {
                            position: (x as i32, y as i32),
                        })
                }
            }
        }
    }
    (frequency_to_antennas_map, problem_area)
}

fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day8.txt")?)
}

fn calculate_two_antinodes_inside_problem_area(
    antenna_1: &Antenna,
    antenna_2: &Antenna,
    problem_area: &ProblemArea,
) -> HashSet<Antinode> {
    let anti_node_1_position = (
        2 * antenna_1.position.0 - antenna_2.position.0,
        2 * antenna_1.position.1 - antenna_2.position.1,
    );
    let anti_node_2_position = (
        2 * antenna_2.position.0 - antenna_1.position.0,
        2 * antenna_2.position.1 - antenna_1.position.1,
    );
    let potential_antinodes = vec![
        Antinode {
            position: anti_node_1_position,
        },
        Antinode {
            position: anti_node_2_position,
        },
    ];

    potential_antinodes
        .into_iter()
        .filter(|anti_node| anti_node.is_within_problem_area(problem_area))
        .collect()
}

fn part1(
    frequency_to_antennas_map: &HashMap<Frequency, Vec<Antenna>>,
    problem_area: &ProblemArea,
) -> Result<i32, Box<dyn Error>> {
    let mut all_anti_nodes: HashSet<Antinode> = HashSet::new();
    for (_, antennas) in frequency_to_antennas_map.iter() {
        let antinodes: HashSet<Antinode> = antennas
            .iter()
            .combinations(2)
            .map(|antennas| {
                calculate_two_antinodes_inside_problem_area(antennas[0], antennas[1], problem_area)
            })
            .flat_map(|s| s)
            .collect();
        all_anti_nodes.extend(antinodes);
    }
    Ok(all_anti_nodes.len() as i32)
}

fn calculate_all_antinodes_inside_problem_area(
    antenna_1: &Antenna,
    antenna_2: &Antenna,
    problem_area: &ProblemArea,
) -> HashSet<Antinode> {
    let delta_x = antenna_1.position.0 - antenna_2.position.0;
    let delta_y = antenna_1.position.1 - antenna_2.position.1;

    let max_number_of_antinodes = max(problem_area.size.0, problem_area.size.1);

    ((-max_number_of_antinodes)..(max_number_of_antinodes))
        .into_iter()
        .map(|index| Antinode {
            position: (
                antenna_1.position.0 + index * delta_x,
                antenna_1.position.1 + index * delta_y,
            ),
        })
        .filter(|anti_node| anti_node.is_within_problem_area(problem_area))
        .collect()
}

fn part2(
    frequency_to_antennas_map: &HashMap<Frequency, Vec<Antenna>>,
    problem_area: &ProblemArea,
) -> Result<i32, Box<dyn Error>> {
    let mut all_anti_nodes: HashSet<Antinode> = HashSet::new();
    for (_, antennas) in frequency_to_antennas_map.iter() {
        let antinodes: HashSet<Antinode> = antennas
            .iter()
            .combinations(2)
            .map(|antennas| {
                calculate_all_antinodes_inside_problem_area(antennas[0], antennas[1], problem_area)
            })
            .flat_map(|s| s)
            .collect();
        all_anti_nodes.extend(antinodes);
    }
    Ok(all_anti_nodes.len() as i32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let (frequency_to_antennas_map, problem_area) = extract_elements_from_string(data);

    let result_part1 = part1(&frequency_to_antennas_map, &problem_area)?;
    let result_part2 = part2(&frequency_to_antennas_map, &problem_area)?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data_part1() -> String {
        "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............"
            .trim_start_matches("\n")
            .replace("    ", "")
            .to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data_part1();

        let (frequency_to_antennas_map, problem_area) =
            extract_elements_from_string(data_as_string);

        let result = part1(&frequency_to_antennas_map, &problem_area)?;

        println!("{result}");
        assert!(result == 14);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data_part1();

        let (frequency_to_antennas_map, problem_area) =
            extract_elements_from_string(data_as_string);

        let result = part2(&frequency_to_antennas_map, &problem_area)?;

        println!("{result}");
        assert!(result == 34);
        Ok(())
    }
}
