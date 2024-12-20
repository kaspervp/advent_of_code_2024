use std::error::Error;

fn import_data() -> Result<String, Box<dyn Error>> {
    let data = std::fs::read_to_string("data/day4.txt")?;

    Ok(data)
}

fn translate_xmas_to_1234(data_with_xmas: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let data_with_1234 = data_with_xmas
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|char| match char {
                    'X' => Ok(1),
                    'M' => Ok(2),
                    'A' => Ok(3),
                    'S' => Ok(4),
                    _ => Err(format!("Char not in XMAS. Got {}", char)),
                })
                .collect::<Result<Vec<i32>, String>>()
        })
        .collect::<Result<Vec<Vec<i32>>, String>>()?;
    Ok(data_with_1234)
}

fn part1(data: &str) -> Result<i32, Box<dyn Error>> {
    let data_with_1234 = translate_xmas_to_1234(data)?;
    let number_of_rows = (data_with_1234).len();
    let number_of_columns = (data_with_1234[0]).len();

    let mut xmas_count = 0;
    for idx1 in 0..number_of_rows {
        for idx2 in 0..(number_of_columns - 3) {
            let partial_vector = (0..4)
                .map(|i| data_with_1234[idx1][idx2 + i])
                .collect::<Vec<i32>>();
            if partial_vector == vec![1, 2, 3, 4] || partial_vector == vec![4, 3, 2, 1] {
                xmas_count += 1;
            }
        }
    }

    for idx1 in 0..(number_of_rows - 3) {
        for idx2 in 0..number_of_columns {
            let partial_vector = (0..4)
                .map(|i| data_with_1234[idx1 + i][idx2])
                .collect::<Vec<i32>>();
            if partial_vector == vec![1, 2, 3, 4] || partial_vector == vec![4, 3, 2, 1] {
                xmas_count += 1;
            }
        }
    }

    for idx1 in 0..(number_of_rows - 3) {
        for idx2 in 0..(number_of_columns - 3) {
            let partial_vector_diagonally_down = (0..4)
                .map(|i| data_with_1234[idx1 + i][idx2 + i])
                .collect::<Vec<i32>>();
            if partial_vector_diagonally_down == vec![1, 2, 3, 4]
                || partial_vector_diagonally_down == vec![4, 3, 2, 1]
            {
                xmas_count += 1;
            }
            let partial_vector_diagonally_up = (0..4)
                .map(|i| data_with_1234[idx1 + 3 - i][idx2 + i])
                .collect::<Vec<i32>>();
            if partial_vector_diagonally_up == vec![1, 2, 3, 4]
                || partial_vector_diagonally_up == vec![4, 3, 2, 1]
            {
                xmas_count += 1;
            }
        }
    }

    Ok(xmas_count)
}

fn part2(data: &str) -> Result<i32, Box<dyn Error>> {
    let data_with_1234 = translate_xmas_to_1234(data)?;
    let number_of_rows = (data_with_1234).len();
    let number_of_columns = (data_with_1234[0]).len();

    let mut xmas_count = 0;
    for idx1 in 0..(number_of_rows - 2) {
        for idx2 in 0..(number_of_columns - 2) {
            let diagonally_up = (0..3)
                .map(|i| data_with_1234[idx1 + i][idx2 + i])
                .collect::<Vec<i32>>();
            let diagonally_down = (0..3)
                .map(|i| data_with_1234[idx1 + 2 - i][idx2 + i])
                .collect::<Vec<i32>>();
            if (diagonally_up == vec![2, 3, 4] || diagonally_up == vec![4, 3, 2])
                && (diagonally_down == vec![2, 3, 4] || diagonally_down == vec![4, 3, 2])
            {
                xmas_count += 1;
            }
        }
    }
    println!("{xmas_count}");

    Ok(xmas_count)
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
        "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"
            .trim_start_matches("\n")
            .replace(" ", "")
            .to_string()
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let data = get_test_data_part1();

        let result = part1(&data)?;

        assert!(result == 18);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let data = get_test_data_part1();

        let result = part2(&data)?;

        assert!(result == 9);
        Ok(())
    }
}
