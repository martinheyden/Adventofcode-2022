use aoc2022::read_input::read_file_to_string_vec;
use std::collections::BTreeMap as Map;

fn main() {
    solve("data/day07.txt");
}

fn solve(s: &str) {
    let file_map = parse_data(s);
    let mut size_map: Map<String, usize> = Map::new();
    let mut tot = 0;
    get_size("root", &file_map, &mut size_map);
    for (_, s) in &size_map {
        if *s <= 100000 {
            tot = tot + *s;
        }
    }
    println!("{}", tot);
    let free = 70000000 - size_map["root"];
    let need = 30000000 - free;
    let mut smallest = 70000000;
    for (_, s) in &size_map {
        if *s < smallest && *s > need {
            smallest = *s;
        }
    }
    println!("Smallest {}", smallest);
}

fn get_size(s: &str, file_map: &Map<String, Dir>, size_map: &mut Map<String, usize>) -> usize {
    match size_map.get(s) {
        Some(u) => return *u,
        None => {
            let mut size: usize = 0;
            size = size + file_map[s].filesize;
            for str in file_map[s].children.clone() {
                size = size + get_size(&str, file_map, size_map);
            }
            size_map.insert(String::from(s), size);
            return size;
        }
    }
}

fn parse_data(s: &str) -> Map<String, Dir> {
    let mut map: Map<String, Dir> = Map::new();
    let lines = read_file_to_string_vec(s);
    let root = Dir::new(String::from("NONE"));
    map.insert(String::from("root"), root);
    let mut current = String::from("root");

    for line in lines[1..].iter() {
        match parse_line(&line) {
            LineParse::Cd(str) => {
                if str == ".." {
                    current = map[&current].parent.clone();
                } else {
                    current = current + "/" + &str;
                }
            }
            LineParse::File(size) => {
                map.get_mut(&current).unwrap().filesize = map[&current].filesize + size;
            }
            LineParse::Dir(s) => {
                let s2 = current.clone() + "/" + &s;
                map.get_mut(&current).unwrap().children.push(s2.clone());
                map.insert(s2.clone(), Dir::new(current.clone()));
            }
            LineParse::Ls => {}
        };
    }

    return map;
}

fn parse_line(s: &str) -> LineParse {
    if s.contains("$ ls") {
        return LineParse::Ls;
    }
    if &s[0..3] == "dir" {
        return LineParse::Dir(String::from(&s[4..s.len() - 1]));
    }
    if s.contains("$ cd ..") {
        return LineParse::Cd(String::from(".."));
    } else if s.contains("$ cd") {
        return LineParse::Cd(String::from(&s[5..s.len() - 1]));
    }
    // Only file left now
    let x: usize = usize::from_str_radix(&s[0..s.find(' ').unwrap()], 10).unwrap();
    return LineParse::File(x);
}

#[derive(Debug)]
struct Dir {
    parent: String,
    children: Vec<String>,
    filesize: usize,
}

impl Dir {
    fn new(parent: String) -> Self {
        return Self {
            parent: parent,
            children: Vec::new(),
            filesize: 0,
        };
    }
}

enum LineParse {
    Cd(String),
    File(usize),
    Dir(String),
    Ls,
}
