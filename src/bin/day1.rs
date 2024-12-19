use std::{error::Error, num::ParseIntError};

fn part1(list1: &[i32], list2: &[i32]) -> i32 {
    let (sorted_list1, sorted_list2) = {
        let (mut c1, mut c2) = (list1.to_vec(), list2.to_vec());
        c1.sort();
        c2.sort();
        (c1, c2)
    };

    sorted_list1
        .into_iter()
        .zip(sorted_list2)
        .map(|(e1, e2)| (e1 - e2).abs())
        .sum()
}

fn part2(list1: &[i32], list2: &[i32]) -> i32 {
    list1.iter().fold(0, |acc, &x1| {
        let score_for_x1 = list2.iter().filter(|&&x2| x1 == x2).sum::<i32>();
        acc + score_for_x1
    })
}

fn import_data() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let data = std::fs::read_to_string("data/day1.txt")?
        .lines()
        .map(|line| line.split("   ").map(|s| s.parse::<i32>()).collect())
        .collect::<Result<Vec<Vec<i32>>, ParseIntError>>()?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data()?;

    let (list1, list2): (Vec<i32>, Vec<i32>) = data.iter().map(|l| (l[0], l[1])).unzip();

    let result_part1 = part1(&list1, &list2);
    let result_part2 = part2(&list1, &list2);

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> (Vec<i32>, Vec<i32>) {
        (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    }

    #[test]
    fn test_part1() {
        let (list1, list2) = get_test_data();

        let result = part1(&list1, &list2);

        assert!(result == 11);
    }

    #[test]
    fn test_part2() {
        let (list1, list2) = get_test_data();

        let result = part2(&list1, &list2);

        assert!(result == 31);
    }
}
