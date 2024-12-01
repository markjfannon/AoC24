use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
    iter::zip,
};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Unable to read the file!");

    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

fn part1(contents: &str) -> i32 {
    let lines = contents.split("\n");

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for item in lines {
        let nums: Vec<i32> = item
            .split_whitespace()
            .map(|num| num.parse().expect("Item should be an integer"))
            .collect();

        list1.push(nums[0]);
        list2.push(nums[1]);
    }

    list1.sort();
    list2.sort();

    zip(list1, list2).fold(0, |acc, (x, y)| acc + (max(x, y) - min(x, y)))
}

fn part2(contents: &str) -> i32 {
    let x = contents.split("\n");

    let mut list: Vec<i32> = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();

    for item in x {
        let nums: Vec<i32> = item
            .split_whitespace()
            .map(|num| num.parse().expect("Item should be an integer"))
            .collect();

        list.push(nums[0]);

        map.entry(nums[1])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let sums = list.iter().fold(0, |acc, x| match map.get(x) {
        Some(num) => acc + (x * num),
        None => acc,
    });

    sums
}
