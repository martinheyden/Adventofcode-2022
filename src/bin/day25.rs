use core::panic;

use aoc2022::read_input::read_file_to_string_vec;

fn main() {
    let lines = read_file_to_string_vec("data/day25.txt");
    let mut sum = 0;
    for line in &lines {
        sum += convert_to_decimal(line);
    }
    println!("sum in normal {}", sum);
    println!("isize max {}", isize::MAX);
    let weird_sum = convert_to_SNAFU(sum);
    println!("sum in SNAFU {}", weird_sum);
}



fn convert_to_decimal(str: &String) -> isize {
    let mut val = 0;
    for (i,ch) in str.trim().chars().rev().enumerate() {
        let dig = match ch {
            '1' => 1,
            '2' => 2,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unsupported digit"),
        };
        val += dig * 5isize.pow(i as u32);
    }
    return val;
}


fn convert_to_SNAFU(mut val:isize) -> String {
    let mut s = String::new();
    let mut rest = false;
    while val != 0 {
        match val % 5 {
            0 => s.push('0'),
            1 => s.push('1'),
            2 => s.push('2'),
            4 => {
                s.push('-');
                rest = true;
            },
            3 => {
                s.push('=');
                rest = true;
            },
            _ => panic!("unreachable"),
        };
        if rest {
            val = val+5;
        }
        val = val/5;
        rest = false;
    }
    return s.chars().rev().collect();
}