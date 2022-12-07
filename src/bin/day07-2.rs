use aoc2022::read_input::read_file_to_string_vec;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    solve("data/day07.txt");
}

fn solve(s: &str) {
    let root = parse_data(s);
    let mut tot = 0;
    get_size(Rc::clone(&root), &mut tot);
    println!("{}", tot);
    let free = 70000000 - root.borrow().depthsize.unwrap();
    let need = 30000000 - free;
    let mut smallest = 70000000;
    check(Rc::clone(&root), &mut smallest, need);
    println!("{}", smallest);
}

fn get_size(node: Rc<RefCell<Dir>>, tot: &mut usize) -> usize {
    let mut n = node.borrow_mut();
    match n.depthsize {
        Some(u) => return u,
        None => {
            let mut size: usize = 0;
            size = size + n.filesize;
            for i in 0..n.children.len() {
                let child = Rc::clone(&n.children[i]);
                size = size + get_size(child, tot);
            }
            n.depthsize = Some(size);
            if size <= 100000 {
                *tot += size;
            }
            return size;
        }
    }
}

fn check(node: Rc<RefCell<Dir>>, smallest: &mut usize, need: usize) {
    let n = node.borrow();
    let depthsize = n.depthsize.unwrap();
    if depthsize > need && depthsize < *smallest {
        *smallest = depthsize;
    }
    for i in 0..n.children.len() {
        let child = Rc::clone(&n.children[i]);
        check(child, smallest, need);
    }
}
fn parse_data(s: &str) -> Rc<RefCell<Dir>> {
    let lines = read_file_to_string_vec(s);
    let root = Rc::new(RefCell::new(Dir::new(String::from("root"), None)));
    let mut current_ref = Rc::clone(&root);

    for line in lines[1..].iter() {
        match parse_line(&line) {
            LineParse::Cd(str) => {
                if str == ".." {
                    let parent = Rc::clone(&current_ref.borrow().parent.as_ref().unwrap());
                    current_ref = parent;
                } else {
                    let child = Rc::new(RefCell::new(Dir::new(String::from(str), None)));
                    {
                        let mut current = current_ref.borrow_mut();
                        current.children.push(Rc::clone(&child));
                    }
                    child.borrow_mut().parent = Some(Rc::clone(&current_ref));
                    current_ref = Rc::clone(&child);
                }
            }
            LineParse::File(size) => {
                current_ref.borrow_mut().filesize += size;
            }
            LineParse::Dir(_) => {} //Might as well ignore this
            LineParse::Ls => {}     // And this
        };
    }

    return root;
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
    name: String,
    children: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
    filesize: usize,
    depthsize: Option<usize>,
}

impl Dir {
    fn new(name: String, parent: Option<Rc<RefCell<Dir>>>) -> Self {
        return Self {
            name: name,
            parent: parent,
            children: Vec::new(),
            filesize: 0,
            depthsize: None,
        };
    }
}

enum LineParse {
    Cd(String),
    File(usize),
    Dir(String),
    Ls,
}
