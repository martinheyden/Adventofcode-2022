use std::{cell::RefCell, num::ParseIntError};

use aoc2022::read_input::read_file_to_string_vec;

fn main() {
    let mut monkeys = parse_data("data/day11.txt");
    solve(&mut monkeys, 20, Box::new(|x| x / 3));

    let mut monkeys = parse_data("data/day11.txt");
    let mut test = 1;
    for m in &monkeys {
        test *= m.borrow().divisor;
    }
    solve(&mut monkeys, 10000, Box::new(move |x| x % test));
}

fn solve(monkeys: &Vec<RefCell<Monkey>>, nbr_itr: usize, op: Box<dyn Fn(usize) -> usize>) {
    for _ in 0..nbr_itr {
        for monkey_cell in monkeys {
            let mut monkey = (*monkey_cell).borrow_mut();
            while !monkey.items.is_empty() {
                monkey.inspections += 1;
                let mut item = monkey.items.remove(0);
                match monkey.operation {
                    Op::Addition => item += monkey.operation_value.unwrap(),
                    Op::Multiplication => match monkey.operation_value {
                        Some(v) => item *= v,
                        None => item *= item,
                    },
                }
                item = op(item);

                let test = item % monkey.divisor == 0;
                if test {
                    monkeys[monkey.target_true].borrow_mut().items.push(item);
                } else {
                    monkeys[monkey.target_false].borrow_mut().items.push(item);
                }
            }
        }
    }
    let mut throws = monkeys
        .iter()
        .map(|m| m.borrow().inspections)
        .collect::<Vec<usize>>();
    throws.sort();
    println!(
        "ans {}",
        throws[throws.len() - 1] * throws[throws.len() - 2]
    );
}

fn parse_data(file: &str) -> Vec<RefCell<Monkey>> {
    let lines = read_file_to_string_vec(file);
    let mut i = 0;
    let mut monkeys = Vec::<RefCell<Monkey>>::new();
    while i < lines.len() {
        let temp_vec = Vec::<usize>::new(); //todo remove
        let mut monkey = Monkey {
            items: temp_vec,
            operation: Op::Addition,
            operation_value: None,
            divisor: 0,
            target_true: 0,
            target_false: 0,
            inspections: 0,
        };
        i += 1;
        parse_starting_items(&lines[i], &mut monkey);
        i += 1;
        parse_operator(&lines[i], &mut monkey);
        i += 1;
        parse_test(&lines[i..i + 3], &mut monkey);
        i = i + 4;
        monkeys.push(RefCell::new(monkey));
    }
    return monkeys;
}

fn parse_starting_items(line: &str, monkey: &mut Monkey) {
    let slice = line[line.find(":").unwrap() + 1..].trim();
    monkey.items = slice
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();
}

fn parse_operator(line: &str, monkey: &mut Monkey) {
    let mult = line.contains("*");
    if mult {
        monkey.operation = Op::Multiplication;
    } else {
        monkey.operation = Op::Addition;
    }
    match extract_number(line) {
        Ok(v) => monkey.operation_value = Some(v),
        Err(_) => monkey.operation_value = None,
    }
    //Never old+old
}

fn parse_test(lines: &[String], monkey: &mut Monkey) {
    monkey.divisor = extract_number(&lines[0]).unwrap();
    monkey.target_true = extract_number(&lines[1]).unwrap();
    monkey.target_false = extract_number(&lines[2]).unwrap();
}

fn extract_number(s: &str) -> Result<usize, ParseIntError> {
    s.trim_matches(|c: char| !c.is_numeric()).parse::<usize>()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Op,
    operation_value: Option<usize>,
    divisor: usize,
    target_true: usize,
    target_false: usize,
    inspections: usize,
}

#[derive(Debug)]
enum Op {
    Addition,
    Multiplication,
}
