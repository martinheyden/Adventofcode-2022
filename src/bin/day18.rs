use aoc2022::read_input::read_file_to_string_vec;
use itertools::Itertools;
use std::collections::BTreeSet as Set;
use std::collections::HashMap as Map;

type Cord = (isize, isize, isize);

#[derive(Debug)]
struct Bounds {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl Bounds {
    fn new() -> Self {
        return Self {
            min_x: isize::MAX,
            max_x: isize::MIN,
            min_y: isize::MAX,
            max_y: isize::MIN,
            min_z: isize::MAX,
            max_z: isize::MIN,
        };
    }

    fn update(&mut self, cord: &Cord) {
        if cord.0 > self.max_x {
            self.max_x = cord.0;
        }
        if cord.0 < self.min_x {
            self.min_x = cord.0;
        }
        if cord.1 > self.max_y {
            self.max_y = cord.1;
        }
        if cord.1 < self.min_y {
            self.min_y = cord.1;
        }
        if cord.2 > self.max_z {
            self.max_z = cord.2;
        }
        if cord.2 < self.min_z {
            self.min_z = cord.2;
        }
    }

    fn within(&self, cord: &Cord) -> bool {
        return !(cord.0 <= self.min_x - 1
            || cord.0 >= self.max_x + 1
            || cord.1 <= self.min_y - 1
            || cord.1 >= self.max_y + 1
            || cord.2 <= self.min_z - 1
            || cord.2 >= self.max_z + 1);
    }
}

fn main() {
    let cubes = parse_strings(read_file_to_string_vec("data/day18.txt"));
    let mut sides_data = Map::<Cord, Vec<Cord>>::new();
    let mut cubes_set: Set<Cord> = Set::new();
    let mut bounds = Bounds::new();
    for cube in cubes {
        cubes_set.insert(cube.clone());
        bounds.update(&cube);
        let mut sides = get_sides(&cube);
        if sides_data.contains_key(&cube) {
            for neigh in sides_data.get(&cube).unwrap() {
                sides.remove(neigh);
            }
            sides_data.remove_entry(&cube);
        }
        for side in sides {
            match sides_data.get_mut(&side) {
                Some(v) => v.push(cube),
                None => {
                    sides_data.insert(side, vec![cube]);
                }
            }
        }
    }

    let len = sides_data
        .iter()
        .map(|entry| entry.1.len())
        .fold(0, |acc, val| return acc + val);
    println!("Ans: {}", len);

    let mut count = 0;
    let mut memory = Map::<Cord, bool>::new();
    for (cube, v) in &sides_data {
        let mut checked = Set::<Cord>::new();
        let res = check_outside(cube, &mut memory, &bounds, &cubes_set, &mut checked);
        for cord in checked {
            memory.insert(cord, res);
        }
        if res {
            count += v.len();
        }
    }
    println!("Ans: {}", count);
}

fn check_outside(
    cube: &Cord,
    memory: &mut Map<Cord, bool>,
    bounds: &Bounds,
    cubes_set: &Set<Cord>,
    checked: &mut Set<Cord>,
) -> bool {
    if cubes_set.contains(cube) {
        return false;
    }
    if !bounds.within(&cube) {
        return true;
    }
    if memory.contains_key(cube) {
        return memory[cube];
    }
    checked.insert(cube.clone());
    let mut res = false;
    // Check neighbours
    for neigh in get_sides(cube) {
        if !checked.contains(&neigh) {
            if check_outside(&neigh, memory, bounds, cubes_set, checked) {
                res = true;
                break;
            }
        }
    }
    return res;
}

fn parse_strings(lines: Vec<String>) -> Vec<Cord> {
    let mut cubes: Vec<Cord> = Vec::new();
    for line in lines {
        cubes.push(
            line.trim()
                .split(",")
                .map(|s| s.parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }
    return cubes;
}

fn get_sides(cube: &Cord) -> Set<Cord> {
    let mut sides = Set::new();
    for dx in [-1, 1] {
        sides.insert((cube.0 + dx, cube.1, cube.2));
    }
    for dy in [-1, 1] {
        sides.insert((cube.0, cube.1 + dy, cube.2));
    }
    for dz in [-1, 1] {
        sides.insert((cube.0, cube.1, cube.2 + dz));
    }
    return sides;
}
