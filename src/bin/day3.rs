use regex::Regex;
use std::{error::Error, num::ParseIntError};

fn import_data() -> Result<String, Box<dyn Error>> {
    let data = std::fs::read_to_string("data/day3.txt")?;

    Ok(data)
}

///Extract the pair of number in a string of type mul(a,b) and return a*b.
fn get_product_from_mul_string(mul_string: &str) -> Result<i32, Box<dyn Error>> {
    let pairs = mul_string
        .replace("mul(", "")
        .replace(")", "")
        .split(",")
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()?;
    Ok(pairs[0] * pairs[1])
}

fn part1(data: &str) -> Result<i32, Box<dyn Error>> {
    let regular_expression = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let sum: i32 = regular_expression
        .captures_iter(data)
        .map(|cap| get_product_from_mul_string(&cap[0]))
        .collect::<Result<Vec<i32>, Box<dyn Error>>>()?
        .iter()
        .sum();

    Ok(sum)
}

fn part2(data: &str) -> Result<i32, Box<dyn Error>> {
    let regular_expression = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")?;
    let mut sum = 0;
    let mut keep_calculating = true;
    for capture in regular_expression.captures_iter(data) {
        let capture_as_string = &capture[0];
        match capture_as_string {
            "do()" => keep_calculating = true,
            "don't()" => keep_calculating = false,
            _ if keep_calculating => sum += get_product_from_mul_string(capture_as_string)?,
            _ => {}
        }
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let result_part1 = part1(&data)?;
    let result_part2 = part2(&data)?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data_part1() -> String {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()
    }

    fn get_test_data_part2() -> String {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data = get_test_data_part1();

        let result = part1(&data)?;

        assert!(result == 161);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let data = get_test_data_part2();

        let result = part2(&data)?;

        assert!(result == 48);
        Ok(())
    }
}
