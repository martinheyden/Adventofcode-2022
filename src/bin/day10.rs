use aoc2022::read_input::read_file_to_string_vec;

fn main() {
    solve("data/day10.txt");
}

const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

fn solve(file_name: &str) {
    let lines = read_file_to_string_vec(file_name);
    let mut x: isize = 1;
    let mut clock: usize = 1;
    let mut signal_string: isize = 0;
    let mut add_next_cycle = false;
    let mut to_add = 0;
    let mut instr_index = 0;
    let mut screen: Vec<Vec<char>> = vec![vec![' '; 40]; 6];
    loop {
        if CYCLES.contains(&clock) {
            signal_string += x * clock as isize;
        }
        update_screen(&mut screen, clock, x);
        if add_next_cycle {
            x += to_add;
            add_next_cycle = false;
        } else {
            if lines.len() == instr_index {
                break;
            }
            let instr = parse_line(lines[instr_index].trim());
            instr_index += 1;
            match instr {
                Instruction::Noop => {}
                Instruction::Addx(val) => {
                    add_next_cycle = true;
                    to_add = val;
                }
            }
        }
        clock += 1;
    }
    println!("{}", signal_string);
    for v in screen {
        let s: String = v.iter().collect();
        println!("{:?}", s);
    }
}

fn update_screen(screen: &mut Vec<Vec<char>>, clock: usize, x: isize) {
    let clock_i = clock as isize - 1;
    let row = clock_i / 40;
    if x < 0 {
        return;
    }

    let pos = clock_i % 40;
    if (pos - x % 40).abs() > 1 {
        return
    }
    screen[row as usize][pos as usize] = '#'
}

fn parse_line(line: &str) -> Instruction {
    if &line[0..4] == "noop" {
        return Instruction::Noop;
    } else {
        return Instruction::Addx(*(&line[5..].parse().unwrap()));
    }
}

enum Instruction {
    Noop,
    Addx(isize),
}
