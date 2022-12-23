use aoc2022::read_input::read_file_to_string_vec;
use std::collections::HashSet as Set;
use std::collections::HashMap as Map;

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    East,
    South,
    West
}
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    x: isize,
    y: isize
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self{x: x, y: y}
    }

    fn get_neighbours(&self, dir: Dir) -> Vec<Position> {
        match dir {
            Dir::North => vec![Position::new(self.x-1, self.y-1), Position::new(self.x, self.y-1), Position::new(self.x+1, self.y-1)],
            Dir::East => vec![Position::new(self.x+1, self.y-1), Position::new(self.x+1, self.y), Position::new(self.x+1, self.y+1)],
            Dir::South => vec![Position::new(self.x-1, self.y+1), Position::new(self.x, self.y+1), Position::new(self.x+1, self.y+1)],
            Dir::West => vec![Position::new(self.x-1, self.y-1), Position::new(self.x-1, self.y), Position::new(self.x-1, self.y+1)]
        }
    }

    fn propose_position(&self, elves: &Set<Position>, dir_order: &Vec<Dir>) -> Option<Position> {
        if !self.has_neighbour(elves) {
            // println!("{:?} has no neigh", self);
            return None;
        }
        for dir in dir_order {
            let mut possible = true;
            for p in self.get_neighbours(*dir) {
                if elves.contains(&p) {
                    possible = false;
                    break;
                }
            }
            if possible {
                return Some(self.go_dir(*dir));
            }
        }
        return None
    }

    fn go_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::North => return Position::new(self.x, self.y - 1),
            Dir::East => return Position::new(self.x + 1, self.y),
            Dir::South => return Position::new(self.x, self.y + 1),
            Dir::West => return Position::new(self.x - 1, self.y)
        }
    }

    fn has_neighbour(&self, elves: &Set<Position>) -> bool {
        for dir in [Dir::North, Dir::South, Dir::West, Dir::East] {
            for p in &self.get_neighbours(dir) {
                if elves.contains(p) {
                    return true
                }
            }
        }
        return false
    }


}


fn main() {
    let mut elves = parse_input("data/day23.txt");
    let mut dir_order = vec![Dir::North, Dir::South, Dir::West, Dir::East];
    let mut i = 0;
    let mut ans = 0;
    loop {
        let mut moved = false;
        i = i + 1;
        let mut proposed_positions = Map::<Position, Vec<Position>>::new();
        for p in &elves {
            match p.propose_position(&elves, &dir_order) {
                None => {},
                Some(new_p) => {
                    match  proposed_positions.get_mut(&new_p) {
                       Some(v) => v.push(new_p),
                       None => {proposed_positions.insert(new_p, vec![*p]);},
                    }
                }
            }
        }
        for (target_pos, source_vec) in &proposed_positions {
            if source_vec.len() != 1 {
                continue;
            }
            moved = true;
            elves.remove(&source_vec[0]);
            elves.insert(*target_pos);
        }
        dir_order.rotate_left(1);
        if i == 10 {
            let xmin = elves.iter().fold(isize::MAX, |acc, val| if val.x<acc { return val.x} else {return acc});
            let xmax = elves.iter().fold(isize::MIN, |acc, val| if val.x>acc { return val.x} else {return acc});
            let ymin = elves.iter().fold(isize::MAX, |acc, val| if val.y<acc { return val.y} else {return acc});
            let ymax = elves.iter().fold(isize::MIN, |acc, val| if val.y>acc { return val.y} else {return acc});
            ans = (xmax +1 - xmin) * (ymax +1 - ymin) - elves.len() as isize;
        }
        if !moved {
            break;
        }
    }
    
    println!("Ans a is {}", ans);
    println!("Ans b is {}", i);
}


fn visualize(elves: &Set<Position>) {
    let xmin = elves.iter().fold(isize::MAX, |acc, val| if val.x<acc { return val.x} else {return acc});
    let xmax = elves.iter().fold(isize::MIN, |acc, val| if val.x>acc { return val.x} else {return acc});
    let ymin = elves.iter().fold(isize::MAX, |acc, val| if val.y<acc { return val.y} else {return acc});
    let ymax = elves.iter().fold(isize::MIN, |acc, val| if val.y>acc { return val.y} else {return acc});
    let mut board = vec![vec!['.';(xmax+1-xmin)as usize];(ymax+1-ymin) as usize];
    for p in elves {
        board[(p.y-ymin) as usize][(p.x-xmin) as usize] = '#';
    }
    for l in board {
        println!("{}", l.iter().collect::<String>());
    }
}

fn parse_input(file_name: &str) -> Set<Position> {
    let mut elves = Set::new();
    let lines = read_file_to_string_vec(file_name);
    for (y,line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {elves.insert(Position::new(x as isize, y as isize));},
                _ => {},
            }
        } 
    }

    return elves
}