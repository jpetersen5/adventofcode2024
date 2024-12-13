advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in input.lines() {
        if let Some((left, right)) = line.split_once("   ") {
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<u32>(), right.parse::<u32>()) {
                left_numbers.push(left_num);
                right_numbers.push(right_num);
            }
        }
    }

    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    let sum: u32 = left_numbers.iter()
        .zip(right_numbers.iter())
        .map(|(left, right)| {
            if left > right {
                left - right
            } else {
                right - left
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in input.lines() {
        if let Some((left, right)) = line.split_once("   ") {
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<u32>(), right.parse::<u32>()) {
                left_numbers.push(left_num);
                right_numbers.push(right_num);
            }
        }
    }

    let similarity_score: u32 = left_numbers.iter()
        .map(|&left_num| {
            let count = right_numbers.iter()
                .filter(|&&right_num| right_num == left_num)
                .count() as u32;
            left_num * count
        })
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let expected = 11;
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let expected = 31;
        assert_eq!(result, Some(expected));
    }
}
