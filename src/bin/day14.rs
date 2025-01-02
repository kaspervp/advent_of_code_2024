use regex::Regex;
use std::error::Error;

fn robots_from_string(input: String) -> Result<Vec<Robot>, Box<dyn Error>> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;
    let mut data = Vec::new();

    for cap in re.captures_iter(&input) {
        let position_x = cap[1].parse::<i32>()?;
        let position_y = cap[2].parse::<i32>()?;
        let velocity_x = cap[3].parse::<i32>()?;
        let velocity_y = cap[4].parse::<i32>()?;

        data.push(Robot {
            position: (position_x, position_y),
            velocity: (velocity_x, velocity_y),
        });
    }

    Ok(data)
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn evolve(&mut self, time: i32, robot_space: &RobotSpace) {
        self.position = (
            ((self.position.0 + self.velocity.0 * time) % robot_space.size.0 + robot_space.size.0)
                % robot_space.size.0,
            ((self.position.1 + self.velocity.1 * time) % robot_space.size.1 + robot_space.size.1)
                % robot_space.size.1,
        );
    }
}

struct RobotSpace {
    size: (i32, i32),
}

fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day14.txt")?)
}

fn calculate_heurestic_from_robots(robots: &[Robot], robot_space: &RobotSpace) -> i32 {
    let mut number_of_robots_in_top_left = 0;
    let mut number_of_robots_in_top_right = 0;
    let mut number_of_robots_in_buttom_left = 0;
    let mut number_of_robots_in_buttom_right = 0;

    let mid_size = ((robot_space.size.0 - 1) / 2, (robot_space.size.1 - 1) / 2);

    for robot in robots.iter() {
        if robot.position.0 < mid_size.0 && robot.position.1 < mid_size.1 {
            number_of_robots_in_top_left += 1;
        } else if robot.position.0 > mid_size.0 && robot.position.1 < mid_size.1 {
            number_of_robots_in_top_right += 1;
        } else if robot.position.0 < mid_size.0 && robot.position.1 > mid_size.1 {
            number_of_robots_in_buttom_left += 1;
        } else if robot.position.0 > mid_size.0 && robot.position.1 > mid_size.1 {
            number_of_robots_in_buttom_right += 1;
        }
    }

    number_of_robots_in_top_left
        * number_of_robots_in_top_right
        * number_of_robots_in_buttom_left
        * number_of_robots_in_buttom_right
}

fn calculate_new_heurestic_from_robots(robots: &[Robot], _robot_space: &RobotSpace) -> i64 {
    let number_of_positions = robots.len() as i64;

    let mean_of_x: i64 = robots
        .iter()
        .map(|robot| robot.position.0 as i64)
        .sum::<i64>()
        / number_of_positions;
    let mean_of_y: i64 = robots
        .iter()
        .map(|robot| robot.position.1 as i64)
        .sum::<i64>()
        / number_of_positions;

    let var_of_x: i64 = robots
        .iter()
        .map(|robot| (robot.position.0 as i64 - mean_of_x).pow(2))
        .sum::<i64>()
        / number_of_positions;
    let var_of_y: i64 = robots
        .iter()
        .map(|robot| (robot.position.1 as i64 - mean_of_y).pow(2))
        .sum::<i64>()
        / number_of_positions;

    var_of_x + var_of_y
}

fn print_robots(robots: &Vec<Robot>, robot_space: &RobotSpace) {
    let mut grid = vec![vec!['.'; robot_space.size.0 as usize]; robot_space.size.1 as usize];

    for robot in robots {
        grid[robot.position.1 as usize][robot.position.0 as usize] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part1(mut robots: Vec<Robot>, robot_space: &RobotSpace) -> Result<i32, Box<dyn Error>> {
    for robot in robots.iter_mut() {
        robot.evolve(100, robot_space);
    }
    Ok(calculate_heurestic_from_robots(&robots, robot_space))
}

fn part2(mut robots: Vec<Robot>, robot_space: &RobotSpace) -> Result<i32, Box<dyn Error>> {
    let mut heurestics = Vec::new();
    let mut clone_of_robots = robots.clone();

    for _ in 0..100000 {
        for robot in robots.iter_mut() {
            robot.evolve(1, robot_space);
        }
        let heurestic = calculate_new_heurestic_from_robots(&robots, robot_space);
        heurestics.push(heurestic);
    }

    let time_at_min_heurestic = heurestics
        .iter()
        .position(|&h| h == *heurestics.iter().min().unwrap())
        .unwrap() as i32
        + 1;

    for robot in clone_of_robots.iter_mut() {
        robot.evolve(time_at_min_heurestic, robot_space);
    }
    println!("Robots at time={}\n", time_at_min_heurestic);
    print_robots(&clone_of_robots, robot_space);

    Ok(time_at_min_heurestic)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let robots = robots_from_string(data)?;
    let robot_space = RobotSpace { size: (101, 103) };

    let result_part1 = part1(robots.clone(), &robot_space)?;
    let result_part2 = part2(robots.clone(), &robot_space)?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        "p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3"
            .replace("    ", "")
            .to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();
        let robot_space = RobotSpace { size: (11, 7) };

        let robots = robots_from_string(data_as_string)?;

        let result = part1(robots, &robot_space)?;

        println!("{}", result);
        assert!(result == 12);
        Ok(())
    }
}
