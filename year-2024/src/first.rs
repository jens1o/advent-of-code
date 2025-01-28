use std::collections::{BinaryHeap, HashMap};

#[allow(unused)]
pub fn first_december() {
    dbg!(calculate_total_distance());
    dbg!(calculate_similarity_score());
}

fn calculate_total_distance() -> u32 {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for line in include_str!("first-part1.txt").lines() {
        let mut numbers = line
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok());

        let [first, second] = [numbers.next().unwrap(), numbers.next().unwrap()];

        left_heap.push(first);
        right_heap.push(second);
    }

    let mut distance_sum = 0;

    // The challenge talks about comparing the two smallest elements to each other, but the ordering
    // doesn't matter. So we compare the two biggest elements to each other and it still works. :)
    while let (Some(biggest_left), Some(biggest_right)) = (left_heap.pop(), right_heap.pop()) {
        let distance = biggest_left.abs_diff(biggest_right);

        distance_sum += distance;
    }

    distance_sum
}

fn calculate_similarity_score() -> u32 {
    let mut left_numbers = Vec::new();
    let mut right_occurence_count: HashMap<u32, u32> = HashMap::new();

    for line in include_str!("first-part1.txt").lines() {
        let mut numbers = line
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok());

        let [first, second] = [numbers.next().unwrap(), numbers.next().unwrap()];

        left_numbers.push(first);
        *right_occurence_count.entry(second).or_default() += 1;
    }

    let mut similarity_score = 0;

    for left_number in left_numbers {
        let occurence_count = right_occurence_count.get(&left_number).copied();

        similarity_score += left_number * occurence_count.unwrap_or(0);
    }

    similarity_score
}
