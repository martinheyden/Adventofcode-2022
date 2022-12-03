use aoc2022::read_input;
use std::collections::BTreeSet as Set;

fn main() {
    println!("part a {}", part_a("data/day03.txt"));
    println!("part b {}", part_b("data/day03.txt"));
    println!("part b {}", part_b2("data/day03.txt"));
}

fn part_a(file: &str) -> usize {
    let sacks = read_input::read_file_to_string_vec(file);
    return sacks
        .iter()
        .map(find_item)
        .map(calc_score)
        .fold(0, |acc, x| return acc + x as usize);
}

fn find_item(s: &String) -> char {
    let (s1, s2) = s.split_at(s.len() / 2);
    return find_common(s1, s2);
}

fn find_common(s1: &str, s2: &str) -> char {
    let v1: Vec<char> = s1.chars().into_iter().collect();
    for c in s2.chars() {
        if v1.contains(&c) {
            return c;
        }
    }
    panic!();
}

fn calc_score(c: char) -> u8 {
    if c.is_uppercase() {
        return c as u8 - 64 + 26;
    } else {
        return c as u8 - 96;
    }
}

fn part_b(file: &str) -> usize {
    let sacks = read_input::read_file_to_string_vec(file);
    let mut i = 0;
    let mut score = 0;
    while i < sacks.len() {
        let s1: Set<char> = sacks[i].chars().into_iter().collect();
        let s2: Set<char> = s1
            .intersection(&sacks[i + 1].chars().into_iter().collect())
            .map(|ch| *ch)
            .collect();
        let s3: Set<char> = s2
            .intersection(&sacks[i + 2].chars().into_iter().collect())
            .map(|ch| *ch)
            .collect();
        let iter = s3.into_iter();
        score = score + calc_score(iter.last().unwrap()) as usize;
        i = i + 3
    }

    return score;
}

fn to_set(s: &String) -> Set<char> {
    return s.chars().into_iter().collect();
}

fn part_b2(file: &str) -> usize {
    let sacks = read_input::read_file_to_string_vec(file);
    return sacks
        .chunks(3)
        .map(calc_chunk)
        .fold(0, |acc, x| return acc + x);
}

fn calc_chunk(s: &[String]) -> usize {
    let x = s
        .iter()
        .map(to_set)
        .reduce(|acc, x| return acc.intersection(&x).map(|ch| *ch).collect());
    return calc_score(*x.unwrap().iter().last().unwrap()) as usize;
}
