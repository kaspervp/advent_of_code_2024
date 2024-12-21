use std::num::ParseIntError;
use std::error::Error;


fn extract_elements_from_string(
    data: String,
) -> Result<Vec<(i64, Vec<i64>)>, ParseIntError> {
    data
        .lines()
        .map(|row| 
            row
                .replace(":", "")
                .split(" ")
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<i64>, ParseIntError>>())
        .map(|numbers| {
            match numbers {
                Ok(n) => Ok((n[0], n[1..].to_vec())),
                Err(e) => Err(e)
            }
        })
        .collect()
}

fn calculate_plus_and_times(left: Vec<i64>, right: i64) -> Vec<i64> {
    let mut result: Vec<i64> = left.iter().map(|l| l + right).collect();
    result.extend(left.iter().map(|l| l * right));
    result
}

fn concatenate_digits(left: i64, right: i64) -> i64{
    left * (10_i64.pow(right.to_string().len() as u32)) + right
}

fn calculate_plus_times_and_concatenation(left: Vec<i64>, right: i64) -> Vec<i64> {
    let mut result: Vec<i64> = left.iter().map(|l| l + right).collect();
    result.extend(left.iter().map(|l| l * right));
    result.extend(left.iter().map(|&l| concatenate_digits(l, right)));
    result
}

fn import_data() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("data/day7.txt")?)
}

fn part1(
    input_data: &Vec<(i64, Vec<i64>)>,
) -> Result<i64, Box<dyn Error>> {
    let mut sum = 0;
    for (result_ref, operands) in input_data{
        
        let mut result = vec![operands[0]];
        for operand in operands[1..].to_vec() {
            result = calculate_plus_and_times(result, operand);
        };
        if result.contains(&result_ref) {
            sum += result_ref;
        };
    };
    Ok(sum)
    
}

fn part2(
    input_data: &Vec<(i64, Vec<i64>)>,
) -> Result<i64, Box<dyn Error>> {
    let mut sum = 0;
    for (result_ref, operands) in input_data{
        
        let mut result = vec![operands[0]];
        for operand in operands[1..].to_vec() {
            result = calculate_plus_times_and_concatenation(result, operand);
        };
        if result.contains(&result_ref) {
            sum += result_ref;
        };
    };
    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let input_data = extract_elements_from_string(data)?;

    let result_part1 = part1(&input_data)?;
    let result_part2 = part2(&input_data)?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data_part1() -> String {
        "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20"
            .trim_start_matches("\n")
            .replace("    ", "")
            .to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data_part1();

        let input_data = extract_elements_from_string(data_as_string)?;

        let result = part1(&input_data)?;

        println!("{result}");
        assert!(result == 3749);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let data_as_string = get_test_data_part1();

        let input_data = extract_elements_from_string(data_as_string)?;

        let result = part2(&input_data)?;

        println!("{result}");
        assert!(result == 11387);
        Ok(())
    }

}
