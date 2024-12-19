use std::{error::Error, num::ParseIntError};



fn import_data() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {

    let data = std::fs::read_to_string("data/day2.txt")?
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<i32>())
                .collect()
        }).collect::<Result<Vec<Vec<i32>>, ParseIntError>>()?;
    Ok(data)

}

fn is_report_completely_safe(report: &Vec<i32>) -> bool {
    let safely_increasing = report
        .windows(2)
        .map(|levels| levels[1] - levels[0])
        .all(|diff| diff > 0 && diff <= 3);
    
    let safely_decreasing = report
        .windows(2)
        .map(|levels| levels[1] - levels[0])
        .all(|diff| diff < 0 && diff >= -3);

    safely_increasing || safely_decreasing
}

fn part1(data: &[Vec<i32>]) -> i32 {
    let number_of_safe_reports = data
        .iter()
        .filter(|report| is_report_completely_safe(report))
        .count() as i32;        
    number_of_safe_reports
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut is_safe = false;
    for excl_idx in 0..report.len() {
        let filtered_report: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != excl_idx)
            .map(|(_, &level)| level)
            .collect();
        if is_report_completely_safe(&filtered_report) {
            is_safe = true;
        };
    };
    is_safe
    
}

fn part2(data: &[Vec<i32>]) -> i32 {
    let number_of_safe_reports = data
        .iter()
        .filter(|report| is_report_safe(report))
        .count() as i32;        
    number_of_safe_reports
}


fn main() -> Result<(), Box<dyn Error>> {

    let data = import_data()?;

    let result_part1 = part1(&data);
    let result_part2 = part2(&data);

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<Vec<i32>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn test_part1() {
        let data = get_test_data();

        let result = part1(&data);
        println!("{result}");

        assert!(result == 2);
    }

    #[test]
    fn test_part2() {
        let data = get_test_data();

        let result = part2(&data);
        println!("{result}");

        assert!(result == 4);
    }
}


    
