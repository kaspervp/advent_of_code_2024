use std::error::Error;
use std::num::ParseIntError;

fn import_data() -> Result<(Vec<(i32, i32)>, Vec<Vec<i32>>), Box<dyn Error>> {
    let data = std::fs::read_to_string("data/day5.txt")?;

    let split_data = data.split("\n\n").collect::<Vec<&str>>();
    let page_ordering_as_string = split_data[0];
    let updates_pages_as_string = split_data[1];

    let pages_ordering = page_ordering_as_string
        .split("\n")
        .map(|s| {
            s.split("|")
                .map(|i| i.parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>()
        })
        .map(|v| match v {
            Ok(v) => Ok((v[0], v[1])),
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<(i32, i32)>, ParseIntError>>()?;

    let updates_pages = updates_pages_as_string
        .lines()
        .map(|line| line.split(",").map(|s| s.parse::<i32>()).collect())
        .collect::<Result<Vec<Vec<i32>>, ParseIntError>>()?;

    Ok((pages_ordering, updates_pages))
}

fn part1(
    page_ordering: &Vec<(i32, i32)>,
    updates_pages: &Vec<Vec<i32>>,
) -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    for update_pages in updates_pages {
        let is_valid = page_ordering
            .iter()
            .map(|(pre, post)| {
                (
                    update_pages.iter().position(|page| page == pre),
                    update_pages.iter().position(|page| page == post),
                )
            })
            .filter_map(|(pre_position, post_position)| pre_position.zip(post_position))
            .all(|(pre_position, post_position)| pre_position < post_position);

        if is_valid {
            sum += update_pages[update_pages.len() / 2]
        }
    }

    Ok(sum)
}

fn part2(
    page_ordering: &Vec<(i32, i32)>,
    updates_pages: &Vec<Vec<i32>>,
) -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    for update_pages in updates_pages {
        let mut copy_of_update_pages = update_pages.clone();
        let mut is_valid = false;

        while !is_valid {
            let indices_to_swap: Vec<(usize, usize)> = page_ordering
                .iter()
                .map(|(pre, post)| {
                    (
                        copy_of_update_pages.iter().position(|page| page == pre),
                        copy_of_update_pages.iter().position(|page| page == post),
                    )
                })
                .filter_map(|(pre_position, post_position)| pre_position.zip(post_position))
                .filter(|(pre_position, post_position)| pre_position > post_position)
                .collect::<Vec<(usize, usize)>>();

            if indices_to_swap.is_empty() {
                is_valid = true;
            } else {
                copy_of_update_pages.swap(indices_to_swap[0].0, indices_to_swap[0].1)
            }
        }

        sum += copy_of_update_pages[copy_of_update_pages.len() / 2];
    }
    Ok(sum - part1(page_ordering, updates_pages)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (page_ordering, updates_pages) = import_data()?;

    let result_part1 = part1(&page_ordering, &updates_pages)?;
    let result_part2 = part2(&page_ordering, &updates_pages)?;

    println!("Result of part1 is: {result_part1}");
    println!("Result of part2 is: {result_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data_part1() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
        let page_ordering = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];

        let updates_pages = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        (page_ordering, updates_pages)
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let (page_ordering, updates_pages) = get_test_data_part1();

        let result = part1(&page_ordering, &updates_pages)?;

        assert!(result == 143);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let (page_ordering, updates_pages) = get_test_data_part1();

        let result = part2(&page_ordering, &updates_pages)?;

        assert!(result == 123);
        Ok(())
    }
}
