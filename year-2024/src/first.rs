use std::collections::BinaryHeap;

pub fn first_december() {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for line in include_str!("first-part1.txt").lines() {
        let mut numbers = line
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok());

        let [first, second] = [numbers.next().unwrap(), numbers.next().unwrap()];

        left_heap.push(first);
        right_heap.push(second);
    }

    let mut distance_sum = 0;

    while let (Some(smallest_left), Some(smallest_right)) = (left_heap.pop(), right_heap.pop()) {
        let distance = smallest_left.abs_diff(smallest_right);

        distance_sum += distance;
    }

    dbg!(distance_sum);
}
