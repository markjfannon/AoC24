use std::{
    collections::HashMap,
    fs,
    iter::zip,
};

fn main() {
    let lines = fs::read_to_string("input.txt").expect("Unable to read the file!");
    let lines = lines.split("\n");

    let (list1, list2) : (Vec<i32>, Vec<i32>) = lines.into_iter()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()))
        .unzip();

    println!("{}", part1(&list1, &list2));
    println!("{}", part2(&list1, &list2));
}

fn part1(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    zip(list1, list2).map(|(x, y)| (x - y).abs()).sum()
}

fn part2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    for item in list2 {
        map.entry(*item)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    list1.iter().fold(0, |acc, x| match map.get(x) {
        Some(num) => acc + (x * num),
        None => acc,
    })
}
