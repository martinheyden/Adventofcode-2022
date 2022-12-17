use aoc2022::read_input::read_file_to_string;
use std::collections::HashSet as Set;
use std::collections::HashMap as Map;

const WALL_LEFT: isize = -1;
const WALL_RIGHT: isize = 7;
const SPAWN_LEFT: isize = 2;

type RockSet = Set<Cord>;
type PreviousState = (usize, Vec<isize>); // jet index, rock index, board relative to min

#[derive(Hash)]
struct Mem {
    jets_ind: usize,
    rock_ind: usize,
    board: Vec<usize>,
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let jets = parse_input("data/day17.txt");
    solve(2022, &jets);
    solve(1000000000000, &jets);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn solve(limit: usize, jets: &Vec<isize>) {
    let mut jets_ind = 0;
    let mut rock_set: RockSet = Set::new();
    let mut max = 0;

    let mut memory: Map<PreviousState, (isize, usize)> = Map::new(); //value: height, i
    let mut extra_height = 0;
    let mut jumped = false;
    for i in 0..=6{
        rock_set.insert(Cord::new(i, 0));
    }
    let mut mem: Vec<isize> = vec![0;7]; 
    let mut i: usize= 0;
    while i != limit {
        //Only check for period on #### piece to make sure the memory is sufficient
        if !jumped &&  i%5== 0 && memory.contains_key(&(jets_ind,  mem.clone()))    {
            jumped = true; // No need to do more than one jump
            let (old_heihgt, old_i)  = memory.get(&(jets_ind, mem.clone())).unwrap();
            let cycle_len = i -old_i;
            let nbr_cycles = (limit -i) / cycle_len;
            i += cycle_len*nbr_cycles;
            extra_height = (nbr_cycles)*(max-old_heihgt) as usize;
        } else {
            if i%5 == 0 && !jumped {
                memory.insert((jets_ind, mem.clone()), (max,i));
            }
            let mut rock = Rock::new(i%5, max);
            loop {
                let dir = Cord::new(jets[jets_ind], 0);
                jets_ind = (jets_ind+1) % jets.len();
                if rock.can_move(&dir, &rock_set) {
                    rock.move_x(dir.x);
                }

                let dir = Cord::new(0,-1);
                if rock.can_move(&dir, &rock_set) {
                    rock.move_y();
                } else {
                    update_layer(&mut rock_set, &rock, &mut max);
                    break;
                }
            } 

            mem = prune_and_get_mem(&mut rock_set);
            i += 1;
        }    
    }
    let high = rock_set.iter().map(|c| c.y).fold(0, |acc, i| if i> acc {return i} else {return acc});
    println!("ans {}", high + extra_height as isize);


}


fn prune_and_get_mem(rockset: &mut RockSet) -> Vec<isize> {
    let mut min = vec!(0;7);
    for cord in rockset.iter() {
        if min[cord.x as usize] < cord.y {
            min[cord.x as usize] = cord.y;
        }
    }
    let min = *min.iter().min().unwrap();
    rockset.retain(|cord|  cord.y >= min);
    let mut mem = vec![0;7];
    for cord in rockset.iter() {
        if mem[cord.x as usize] < cord.y-min {
            mem[cord.x as usize] = cord.y-min;
        }
    }
    return mem;
}

//Useful for debugging
fn visualize_rocks(rockSet: &RockSet, max: isize) {
    let mut matrix = vec![vec!['.';8];max as usize+2];
    for cord in rockSet {
        matrix[cord.y as usize][cord.x as usize] = '#';
    }
    for i in 0..max+1 {
        println!{"{:?}", matrix[((max)-i) as usize].iter().collect::<String>()};
    }

}

fn update_layer(upper_layer: &mut RockSet, rock: &Rock, max: &mut isize) {
    for dp in &rock.parts {
        let x = rock.center.x+dp.x;
        let y = rock.center.y+dp.y;
        upper_layer.insert(Cord::new(x,y));
        if y> *max {
            *max +=1;
        }
    }
}


fn parse_input(file_name: &str) ->Vec<isize> {
    let mut jets = Vec::new();
    let line = read_file_to_string(file_name);
    for ch in line.trim().chars() {
        if ch == '>' {
            jets.push(1);
        } else if ch == '<' {
            jets.push(-1);
        } else {
            panic!("invalid jet direction");
        }
    }
    return jets
}




// Rock described by central point, and (dx,dy) from that point
//  Central ponit is defined as lowest point, and if there are ties, lowest left

// o###

// .#.
// ###
// .o.

// ..#
// ..#
// o##

// #
// #
// #
// o

// ##
// o#

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Cord {
    x: isize,
    y: isize,
}


#[derive(Debug)]
struct Rock{
    center: Cord,
    parts: Vec<Cord>
}

impl Cord {
    fn new(x:isize, y:isize) -> Self{
        Self{x:x, y: y}
    }
}

impl Rock {
    fn get_parts(nbr: usize) -> Vec<Cord> {
        if nbr == 0 {
            return vec![Cord::new(0,0), Cord::new(1,0), Cord::new(2,0), Cord::new(3,0 )];
        }else if nbr ==1{
            return vec![Cord::new(0,0), Cord::new(-1, 1), Cord::new(0,1), Cord::new(1, 1), Cord::new(0, 2)];
        }else if nbr ==2{
            return vec![Cord::new(0,0), Cord::new(1,0), Cord::new(2,0), Cord::new(2,1 ), Cord::new(2,2 )];
        }else if nbr ==3{
            return vec![Cord::new(0,0), Cord::new(0, 1), Cord::new(0,2), Cord::new(0, 3)];
        }else if nbr ==4{
            return vec![Cord::new(0,0), Cord::new(0, 1), Cord::new(1,0), Cord::new(1, 1)];
        } else {
            panic!("unknown shape");
        }
    }

    fn new(nbr: usize, bottom_edge: isize) -> Self {
        let parts = Self::get_parts(nbr);

        let center = Cord::new(1,bottom_edge+4);
        let mut rock = Self{center: center, parts: parts};
        while rock.get_left_edge() != SPAWN_LEFT{
            rock.move_x(1);
        }
        return rock;
    }

    fn get_left_edge(&self) -> isize {
        let mut xmin = WALL_RIGHT;
        for dp in &self.parts {
            if self.center.x + dp.x < xmin {
                xmin = self.center.x + dp.x
            }
        }

        return xmin
    }

    fn get_right_edge(&self) -> isize {
        let mut xmax = WALL_LEFT;
        for dp in &self.parts {
            if self.center.x + dp.x > xmax {
                xmax = self.center.x + dp.x
            }
        }

        return xmax
    }

    fn move_x(&mut self, dir:isize) {
        self.center.x += dir;
    }

    fn move_y(&mut self) {
        self.center.y += -1;
    }

    fn can_move(&self, dir: &Cord, upperLayer: &RockSet) -> bool {
        if dir.x!=0 && dir.y !=0 {
            panic!{"unsported move dir"};
        }
        if dir.x ==1 && self.get_right_edge() == WALL_RIGHT-1 {
            return false;
        }
        if dir.x == -1 && self.get_left_edge() == WALL_LEFT + 1 {
            return false;
        }
        for dp in &self.parts {
            let x = self.center.x+dp.x+dir.x;
            let y = self.center.y+dp.y+dir.y;
            if upperLayer.contains(&Cord::new(x,y)) {
                return false;
            }
        }
        return true
    }

}