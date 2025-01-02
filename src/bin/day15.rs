use std::collections::HashSet;
use std::error::Error;
use std::ops::Add;

fn data_from_string(input: String) -> Result<(WareHouse, Vec<Direction>), Box<dyn Error>> {

    let data_string: Vec<&str> = input
        .split("\n\n")
        .map(|s| s)
        .collect();
    
    let size:(i64, i64) = (data_string[0].lines().next().map(|l| l.len()).unwrap() as i64, data_string[0].lines().count() as i64);

    let mut obstacles = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot: Option<Robot> = None;

    for (y, row) in data_string[0].lines().rev().enumerate() {
        for (x, character) in row.chars().enumerate(){
            match character {
                '#' => {obstacles.insert(Obstacle::new(x as i64, y as i64));},
                'O' => {boxes.insert(Package::new(x as i64, y as i64));},
                '@' => robot = Some(Robot::new(x as i64, y as i64)),
                _ => {}
            }
        }
    }

    let instructions = data_string[1]
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err("Unknown instruction found".into())
        }).collect::<Result<Vec<Direction>, &str>>()?;

    if let Some(r) = robot {
        let ware_house = WareHouse::new(obstacles, boxes, r, size);
        Ok((ware_house, instructions))
    } else {
        Err("No Robot symbol (@) found in the input.".into())
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64
}

impl Position {
    fn new(x: i64, y: i64) -> Position {
        Position {x, y}
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y)
    }
}
impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, other: Direction) -> Position {
        match other {
            Direction::Up => Position::new(self.x, self.y + 1),
            Direction::Down => Position::new(self.x, self.y - 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WareHouseObject{
    Obstacle,
    Package,
    Robot,
    Empty
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MovementOutcome {
    Moved,
    Blocked
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Obstacle {
    position: Position,
}

impl Obstacle {
    fn new(x: i64, y: i64) -> Obstacle {
        Obstacle {position: Position::new(x, y)}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Package {
    position: Position,
}

impl Package {
    fn new(x: i64, y: i64) -> Package {
        Package {position: Position::new(x, y)}
    }
    fn gps_coordinate(&self, size: (i64, i64)) -> i64 {
        self.position.x + 100*(size.1 - self.position.y - 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    position: Position,
}

impl Robot {
    fn new(x: i64, y: i64) -> Robot {
        Robot {position: Position::new(x, y)}
    }
}

#[derive(Debug, Clone)]
struct WareHouse {
    obstacles: HashSet<Obstacle>,
    packages: HashSet<Package>,
    robot: Robot,
    size: (i64, i64)
}

impl WareHouse {
    fn new(obstacles: HashSet<Obstacle>, packages: HashSet<Package>, robot: Robot, size: (i64, i64)) -> WareHouse {
        WareHouse {obstacles, packages, robot, size}
    }

    fn get_object_at_position(&self, position: Position) -> WareHouseObject {
        if self.obstacles.contains(&Obstacle {position}) {
            WareHouseObject::Obstacle
        } else if self.packages.contains(&Package {position}) {
            WareHouseObject::Package
        } else if self.robot.position == position {
            WareHouseObject::Robot
        } else {
            WareHouseObject::Empty
        }
    }

    fn move_object(&mut self, position: Position, direction: Direction) -> Result<MovementOutcome, Box<dyn Error>> {
        let new_position = position + direction;
        match self.get_object_at_position(new_position) {
            WareHouseObject::Obstacle => {
                Ok(MovementOutcome::Blocked)
            }
            WareHouseObject::Package => {
                let move_result = self.move_object(new_position, direction)?;
                match move_result {
                    MovementOutcome::Blocked => Ok(MovementOutcome::Blocked),
                    MovementOutcome::Moved => {
                        self.move_object(position, direction)?;
                        Ok(MovementOutcome::Moved)
                    },
                }
            }
            WareHouseObject::Robot => {
                Err("Moving objects to the position of the robot is not well defined".into())
            }
            WareHouseObject::Empty => {
                match self.get_object_at_position(position) {
                    WareHouseObject::Robot => {
                        self.robot.position = new_position; 
                        Ok(MovementOutcome::Moved)
                    }
                    WareHouseObject::Package => {
                        self.packages.remove(&Package {position});
                        self.packages.insert(Package {position: new_position});
                        Ok(MovementOutcome::Moved)
                    }
                    _ => Err("Obstacle or empty spot can not be moved.".into())
                }

            }
        }
    }

    fn print(&self) {
        let mut grid = vec![vec!['.'; self.size.0 as usize]; self.size.1 as usize];

        for obstacle in self.obstacles.iter() {
            grid[(self.size.1 - obstacle.position.y - 1) as usize][obstacle.position.x as usize] = '#';
        }
        for package in self.packages.iter() {
            grid[(self.size.1 - package.position.y - 1) as usize][package.position.x as usize] = 'O';
        }
        grid[(self.size.1 - self.robot.position.y - 1) as usize][self.robot.position.x as usize] = '@';


        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn move_robot(&mut self, direction: Direction) -> Result<(), Box<dyn Error>> {
        let _ = self.move_object(self.robot.position, direction)?;
        Ok(())
    }

    fn sum_of_obstacle_gps_coordinates(&self) -> i64 {
        self.packages.iter().map(|package: &Package| package.gps_coordinate(self.size)).sum()
    }
}


fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day15.txt")?)
}


fn part1(mut ware_house: WareHouse, instructions: &Vec<Direction>) -> Result<i64, Box<dyn Error>> {
    for instruction in instructions {
        ware_house.move_robot(*instruction)?;
    }

    Ok(ware_house.sum_of_obstacle_gps_coordinates())
}

fn part2(ware_house: WareHouse, instructions: &Vec<Direction>) -> Result<i64, Box<dyn Error>> {

    Ok(-1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let (ware_house, instructions) = data_from_string(data)?;

    let result_part1 = part1(ware_house.clone(), &instructions)?;
    let result_part2 = part2(ware_house.clone(), &instructions)?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        "##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .replace("    ", "")
            .to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();

        let (ware_house, instructions) = data_from_string(data_as_string)?;

        let result = part1(ware_house.clone(), &instructions)?;

        println!("{}", result);
        assert!(result == 10092);
        Ok(())
    }
}
