use aoc2022::read_input;
use regex::Regex;

//

fn main() {
    println!("part a {}", part_a("data/day04.txt"));
    println!("part b {}", part_b("data/day04.txt"));
}

fn part_a(file: &str) -> usize {
    return count(contains, file);
}

fn part_b(file: &str) -> usize {
    return count(overlaps, file);
}

fn count(predicate: fn(&(Range, Range)) -> bool, file: &str) -> usize {
    let lines = read_input::read_file_to_string_vec(file);
    return lines
        .iter()
        .map(string_to_pair)
        .filter(|p| predicate(p))
        .count();
}

fn string_to_pair(s: &String) -> (Range, Range) {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let captures = re.captures(s).unwrap();
    let r1 = Range::new(&captures[1], &captures[2]);
    let r2 = Range::new(&captures[3], &captures[4]);
    return (r1, r2);
}

fn contains(p: &(Range, Range)) -> bool {
    return p.0.contains(&p.1) || p.1.contains(&p.0);
}

fn overlaps(p: &(Range, Range)) -> bool {
    return p.0.overlaps(&p.1) || p.1.overlaps(&p.0);
}

#[derive(Debug)]
struct Range {
    low: usize,
    high: usize,
}

impl Range {
    fn new(low: &str, high: &str) -> Self {
        Self {
            low: low.parse().unwrap(),
            high: high.parse().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        return other.low >= self.low && other.high <= self.high;
    }

    fn overlaps(&self, other: &Range) -> bool {
        return (other.low >= self.low && other.low <= self.high)
            || (other.high >= self.low && other.high <= self.high);
    }
}
