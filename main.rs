use rand::seq::SliceRandom;
use std::cmp::Ordering;

fn linear_search(arr: &[i32], target: i32) -> usize {
    let mut steps = 0;
    for &item in arr {
        steps += 1;
        if item == target {
            break;
        }
    }
    steps
}

fn binary_search(arr: &[i32], target: i32) -> usize {
    let mut steps = 0;
    let (mut left, mut right) = (0, arr.len() as isize - 1);

    while left <= right {
        steps += 1;
        let mid = left + (right - left) / 2;
        steps += 2;
        match arr[mid as usize].cmp(&target) {
            Ordering::Equal => return steps,
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid - 1,
        }
    }
    steps
}

/// Generate a random sorted array of given size
fn generate_sorted_array(size: usize) -> Vec<i32> {
    let mut arr: Vec<i32> = (1..=size as i32 * 2).collect();
    arr.shuffle(&mut rand::rng());
    arr.truncate(size);
    arr.sort();
    arr
}

fn main() {
    let sizes = [1000, 2000, 3000, 4000]; // Input sizes

    println!(
        "{:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
        "Size", "LS Best", "LS Avg", "LS Worst", "BS Best", "BS Avg", "BS Worst"
    );
    println!("{}", "-".repeat(80));

    for &size in &sizes {
        // Generate a sorted array
        let sorted_array = generate_sorted_array(size);

        // Define search targets for best, average, and worst cases
        let best_case_target = sorted_array[0]; // First element
        let avg_case_target = sorted_array[(size - 1) / 2]; // Middle element
        let worst_case_target = sorted_array[size - 1]; // Last element

        // Measure step counts for Linear Search
        let steps_linear_best = linear_search(&sorted_array, best_case_target);
        let steps_linear_avg = linear_search(&sorted_array, avg_case_target);
        let steps_linear_worst = linear_search(&sorted_array, worst_case_target);

        // Measure step counts for Binary Search
        let steps_binary_best = binary_search(&sorted_array, avg_case_target);
        let steps_binary_avg = binary_search(&sorted_array, worst_case_target);
        let steps_binary_worst = binary_search(&sorted_array, worst_case_target);

        println!(
            "{:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
            size,
            steps_linear_best,
            steps_linear_avg,
            steps_linear_worst,
            steps_binary_best,
            steps_binary_avg,
            steps_binary_worst
        );
    }
}
