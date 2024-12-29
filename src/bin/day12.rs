use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;
use std::ops::Add;

use itertools::Itertools;

fn garden_regions_from_string(data: String) -> Vec<Vec<char>> {
    data.lines()
        .map(|s| s.chars().collect())
        .rev()
        .collect::<Vec<Vec<char>>>()
}

fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day12.txt")?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EdgeSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn get_neighbors(&self) -> Vec<Position> {
        let directions: Vec<Position> = vec![
            Position { x: 1, y: 0 },
            Position { x: 0, y: -1 },
            Position { x: -1, y: 0 },
            Position { x: 0, y: 1 },
        ];
        directions.iter().map(|&dir| dir + *self).collect()
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    position: Position,
    side: EdgeSide,
}

impl Edge {
    fn new(position: Position, side: EdgeSide) -> Edge {
        Edge { position, side }
    }
}

#[derive(Debug)]
struct GardenRegion {
    positions: Vec<Position>,
}

impl GardenRegion {
    fn perimeter(&self) -> i32 {
        let positions_as_set: HashSet<Position> = self.positions.clone().into_iter().collect();
        let mut perimiter = 0;
        for position in self.positions.iter() {
            let number_of_neighbors_in_regions = position
                .get_neighbors()
                .iter()
                .filter(|potential_neighbor| positions_as_set.contains(potential_neighbor))
                .count() as i32;
            perimiter += 4 - number_of_neighbors_in_regions;
        }
        perimiter
    }

    fn area(&self) -> i32 {
        self.positions.len() as i32
    }

    fn move_one_edge_keeping_region_rightside(&self, edge: Edge) -> Edge {
        match edge.side {
            EdgeSide::Top => {
                if !self
                    .positions
                    .contains(&(edge.position + Position::new(-1, 0)))
                {
                    Edge::new(edge.position, EdgeSide::Left)
                } else if !self
                    .positions
                    .contains(&(edge.position + Position::new(-1, 1)))
                {
                    Edge::new(edge.position + Position::new(-1, 0), EdgeSide::Top)
                } else {
                    Edge::new(edge.position + Position::new(-1, 1), EdgeSide::Right)
                }
            }
            EdgeSide::Left => {
                if !self
                    .positions
                    .contains(&(edge.position + Position::new(0, -1)))
                {
                    Edge::new(edge.position, EdgeSide::Bottom)
                } else if !self
                    .positions
                    .contains(&(edge.position + Position::new(-1, -1)))
                {
                    Edge::new(edge.position + Position::new(0, -1), EdgeSide::Left)
                } else {
                    Edge::new(edge.position + Position::new(-1, -1), EdgeSide::Top)
                }
            }
            EdgeSide::Bottom => {
                if !self
                    .positions
                    .contains(&(edge.position + Position::new(1, 0)))
                {
                    Edge::new(edge.position, EdgeSide::Right)
                } else if !self
                    .positions
                    .contains(&(edge.position + Position::new(1, -1)))
                {
                    Edge::new(edge.position + Position::new(1, 0), EdgeSide::Bottom)
                } else {
                    Edge::new(edge.position + Position::new(1, -1), EdgeSide::Left)
                }
            }
            EdgeSide::Right => {
                if !self
                    .positions
                    .contains(&(edge.position + Position::new(0, 1)))
                {
                    Edge::new(edge.position, EdgeSide::Top)
                } else if !self
                    .positions
                    .contains(&(edge.position + Position::new(1, 1)))
                {
                    Edge::new(edge.position + Position::new(0, 1), EdgeSide::Right)
                } else {
                    Edge::new(edge.position + Position::new(1, 1), EdgeSide::Bottom)
                }
            }
        }
    }

    fn number_of_sides(&self) -> i32 {
        let mut visited_edges: HashSet<Edge> = HashSet::new();
        let mut number_of_sides = 0;
        while let Some(position_with_left_edge) = self
            .positions
            .iter()
            .filter(|&&position| {
                !self
                    .positions
                    .iter()
                    .contains(&(position + Position::new(-1, 0)))
            })
            .find(|&&position| !visited_edges.contains(&Edge::new(position, EdgeSide::Left)))
        {
            let mut current_edge = Edge::new(*position_with_left_edge, EdgeSide::Left);
            while !visited_edges.contains(&current_edge) {
                visited_edges.insert(current_edge);
                let next_edge = self.move_one_edge_keeping_region_rightside(current_edge);
                if next_edge.side != current_edge.side {
                    number_of_sides += 1;
                }
                current_edge = next_edge
            }
        }
        number_of_sides
    }

    fn split_region_into_connected_regions(&self) -> Vec<GardenRegion> {
        let mut remaining_positions: HashSet<Position> =
            self.positions.clone().into_iter().collect();

        let mut split_groups: Vec<GardenRegion> = Vec::new();
        while let Some(&start_point) = remaining_positions.iter().next() {
            let mut positions_in_new_region = Vec::new();
            let mut queue: Vec<Position> = Vec::new();
            queue.push(start_point);

            while let Some(current) = queue.pop() {
                if remaining_positions.remove(&current) {
                    positions_in_new_region.push(current);
                    for neighbor in current.get_neighbors() {
                        if remaining_positions.contains(&neighbor) {
                            queue.push(neighbor);
                        }
                    }
                }
            }
            split_groups.push(GardenRegion {
                positions: positions_in_new_region,
            });
        }
        split_groups
    }
}

struct Garden {
    regions: Vec<(char, GardenRegion)>,
}

impl Garden {
    fn from_garden_map(garden_map: &Vec<Vec<char>>) -> Garden {
        let mut regions: HashMap<char, GardenRegion> = HashMap::new();
        for (y, row) in garden_map.iter().enumerate() {
            for (x, character) in row.iter().enumerate() {
                let position = Position {
                    x: x as i32,
                    y: y as i32,
                };
                regions
                    .entry(*character)
                    .or_insert(GardenRegion {
                        positions: Vec::new(),
                    })
                    .positions
                    .push(position);
            }
        }

        let all_distinct_regions = regions
            .iter()
            .flat_map(|(&character, region)| {
                region
                    .split_region_into_connected_regions()
                    .into_iter()
                    .map(|split_region| (character, split_region))
                    .collect::<Vec<(char, GardenRegion)>>()
            })
            .collect();
        Garden {
            regions: all_distinct_regions,
        }
    }

    fn total_price(&self) -> i32 {
        self.regions
            .iter()
            .map(|(_, region)| region.perimeter() * region.area())
            .sum()
    }

    fn total_bulk_discounted_price(&self) -> i32 {
        self.regions
            .iter()
            .map(|(_, region)| region.number_of_sides() * region.area())
            .sum()
    }
}

fn part1(garden_regions: &Vec<Vec<char>>) -> Result<i32, Box<dyn Error>> {
    let garden = Garden::from_garden_map(garden_regions);
    Ok(garden.total_price())
}

fn part2(garden_regions: &Vec<Vec<char>>) -> Result<i32, Box<dyn Error>> {
    let garden = Garden::from_garden_map(garden_regions);
    Ok(garden.total_bulk_discounted_price())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let garden_regions = garden_regions_from_string(data);

    let result_part1 = part1(&garden_regions)?;
    let result_part2 = part2(&garden_regions)?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        "RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE"
            .replace("    ", "")
            .to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();

        let garden_regions = garden_regions_from_string(data_as_string);

        let result = part1(&garden_regions)?;

        println!("{}", result);
        assert!(result == 1930);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();

        let garden_regions = garden_regions_from_string(data_as_string);

        let result = part2(&garden_regions)?;

        println!("{}", result);
        assert!(result == 1206);
        Ok(())
    }
}
