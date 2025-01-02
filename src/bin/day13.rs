use regex::Regex;
use std::error::Error;

fn claw_machines_from_string(input: String) -> Result<Vec<ClawMachine>, Box<dyn Error>> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\n\s*Button B: X\+(\d+), Y\+(\d+)\n\s*Prize: X=(\d+), Y=(\d+)",
    )?;
    let mut data = Vec::new();

    for cap in re.captures_iter(&input) {
        let button_a_x = cap[1].parse::<i64>()?;
        let button_a_y = cap[2].parse::<i64>()?;
        let button_b_x = cap[3].parse::<i64>()?;
        let button_b_y = cap[4].parse::<i64>()?;
        let prize_x = cap[5].parse::<i64>()?;
        let prize_y = cap[6].parse::<i64>()?;

        data.push(ClawMachine {
            button_a: (button_a_x, button_a_y),
            button_b: (button_b_x, button_b_y),
            prize: (prize_x, prize_y),
        });
    }

    Ok(data)
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    fn get_price_to_solve(&self) -> Option<i64> {
        let determinant = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;

        if determinant == 0 {
            if self.prize.0 / self.button_a.0 != self.prize.1 / self.button_a.1 {
                None
            } else {
                let mut possible_prices: Vec<i64> = Vec::new();
                let mut number_of_button_a_presses = 0;
                while self.button_a.0 * number_of_button_a_presses <= self.prize.0
                    && self.button_a.1 * number_of_button_a_presses <= self.prize.1
                {
                    let mut prize_0 = self.button_a.0 * number_of_button_a_presses;
                    let mut prize_1 = self.button_a.1 * number_of_button_a_presses;
                    let mut number_of_button_b_presses = 0;
                    while prize_0 <= self.prize.0 && prize_1 <= self.prize.1 {
                        if prize_0 == self.prize.0 && prize_1 == self.prize.1 {
                            possible_prices
                                .push(3 * number_of_button_a_presses + number_of_button_b_presses);
                        }
                        prize_0 += self.button_b.0;
                        prize_1 += self.button_b.1;
                        number_of_button_b_presses += 1;
                    }
                    number_of_button_a_presses += 1;
                }
                if possible_prices.is_empty() {
                    None
                } else {
                    possible_prices.into_iter().min()
                }
            }
        } else {
            let number_of_button_a_presses =
                (self.prize.0 * self.button_b.1 - self.prize.1 * self.button_b.0) / determinant;
            let number_of_button_b_presses =
                (-self.prize.0 * self.button_a.1 + self.prize.1 * self.button_a.0) / determinant;

            let prize_0 = self.button_a.0 * number_of_button_a_presses
                + self.button_b.0 * number_of_button_b_presses;
            let prize_1 = self.button_a.1 * number_of_button_a_presses
                + self.button_b.1 * number_of_button_b_presses;

            if prize_0 == self.prize.0 && prize_1 == self.prize.1 {
                Some(3 * number_of_button_a_presses + number_of_button_b_presses)
            } else {
                None
            }
        }
    }
}

fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day13.txt")?)
}

fn part1(claw_machines: &[ClawMachine]) -> Result<i64, Box<dyn Error>> {
    let total_price = claw_machines
        .iter()
        .filter_map(|claw_machine| claw_machine.get_price_to_solve())
        .sum();

    Ok(total_price)
}

fn part2(claw_machines: &[ClawMachine]) -> Result<i64, Box<dyn Error>> {
    let new_claw_machines: Vec<ClawMachine> = claw_machines
        .iter()
        .map(|claw_machine| ClawMachine {
            button_a: claw_machine.button_a,
            button_b: claw_machine.button_b,
            prize: (
                claw_machine.prize.0 + 10000000000000,
                claw_machine.prize.1 + 10000000000000,
            ),
        })
        .collect();

    let mut total_prize = 0;
    for claw_machine in new_claw_machines {
        if let Some(price) = claw_machine.get_price_to_solve() {
            total_prize += price;
        }
    }

    Ok(total_prize)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let claw_machines = claw_machines_from_string(data)?;

    let result_part1 = part1(&claw_machines)?;
    let result_part2 = part2(&claw_machines)?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        "Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+5, Y+5
        Button B: X+1, Y+1
        Prize: X=5, Y=5

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279"
            .replace("    ", "")
            .to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data();

        let claw_machines = claw_machines_from_string(data_as_string)?;

        let result = part1(&claw_machines)?;

        println!("{}", result);
        assert!(result == 483);
        Ok(())
    }
}
