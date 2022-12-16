use aoc2022::read_input::read_file_to_string_vec;

use std::collections::HashMap as Map;

static base:usize = 2;

fn main () {
    let (paths, flows, start) = parse_input("data/day16.txt");
    let (new_paths, new_flows, new_start) = reduce_graph(&paths,&flows,start);

    println!("{:?}", new_paths);
    let mut open_new = vec![false;new_flows.len()];
    let mut memory = Map::<MemData2,usize>::new();
    println!("new start {}", new_start);
    println!("{}", find_preassure(new_start + (30<<6) + (1<<44) + base.pow(16), new_start, &new_paths, &new_flows, &mut memory));

    let mut memory = Map::<MemData2,usize>::new();
    println!("{}", find_preassure(new_start + (26<<6) + (1<<44), new_start, &new_paths, &new_flows, &mut memory));

}

//BITMAP approximately:
// First 6 bits, position
// bit 7..16 time left
// 17 bit last move , 0 not last move, 1 last move
// bit 18 ... describe each valve. 1 open, zero false
// bit 40..45 last position, set to high if OK to return

//TODO avoid going back where came from if no valve has been opened last turn

//type Mem_Data = (usize, usize, Vec<bool>);
type MemData2 = usize;

static loglen: usize = 0;

fn find_preassure(mut mem: MemData2, start: usize, paths: &Vec<Vec<(usize, usize)>>, flows: &Vec<usize>, memory: &mut Map<MemData2, usize>) -> usize {
    // println!("{}", min_left);
    let pos = mem % 64;
    let min_left = (mem >> 6)%64;
    let last_pos = (mem >> 39);
    
    match memory.get(&mem) {
        Some(v) => return *v,
        None => {}
    }
    if memory.len() < loglen {
        println!("Entering {} with {} left, valve open is {} ", pos, min_left, check_valve_open(mem, pos));
    }

    let mut alt: Vec<usize>  = Vec::new();

    if flows[pos] > 0  && !check_valve_open(mem, pos)  && min_left> 0 {
        let mut new_mem = mem + (1<< (17+pos)); //Open valve
        new_mem =  new_mem - base.pow(6);//reduce time by one
        new_mem = new_mem - (last_pos<<39) + (1<<44);//set ok to return
        if memory.len() < loglen {
            println!("opening valve in {}, valve open is {}", pos, check_valve_open(new_mem, pos));
        }
        
        alt.push((min_left-1)*flows[pos] + find_preassure(new_mem, start, paths, flows, memory));
    }
    // Go to neighbour
    for n in &paths[pos] {
        if min_left >= n.1 && last_pos!=n.0{
            let mut new_mem = mem;
            new_mem = new_mem - base.pow(6) * n.1; //Update time
            new_mem = new_mem - new_mem %64 + n.0;// Update position 
            new_mem = new_mem - (last_pos<<39) + (pos<<39);//set last position
            if memory.len() < loglen {
                println!("going from {} to neigh {}, ", pos, new_mem%64);
            }
            alt.push(find_preassure(new_mem, start, paths, flows, memory));
        }
    }
    let v: usize;
    if alt.len() == 0 {
        //Check if last move bit is 0
        if mem&base.pow(16)== 0 {
            if memory.len() < loglen {
                println!("going into last move, ");
            }
            
            let mut new_mem = mem + (26<<6) - (min_left<<6);//rest time
            new_mem = new_mem-new_mem%64 + start;//reset position
            new_mem = new_mem  + base.pow(16); //lsat move bit = 1
            v = find_preassure(new_mem, start, paths, flows, memory);
        } else {
            v = 0;
        }
    } else {
        v = *alt.iter().max().unwrap();
    }

    memory.insert(mem, v);
    return v;
}

fn get_min_left(mem: usize) -> usize {
    return ((mem &16) >>6);
}

fn check_valve_open(mem: usize, pos:usize) -> bool {
    mem & 1<< (17+pos) > 0
}


fn reduce_graph(paths: &Vec<Vec<usize>>, flows: &Vec<usize>, start:usize) -> (Vec<Vec<(usize,usize)>>, Vec<(usize)>, usize) {
    let mut map = Map::<usize,usize>::new();
    let mut new_flows = Vec::new();
    let mut new_paths = Vec::new();
    let mut count = 0;
    let mut new_start = 0;
    for i in 0..paths.len() {
        if paths[i].len()!=2 || flows[i]!= 0 {
            map.insert(i, count);
            new_flows.push(flows[i]);
            new_paths.push(Vec::new());
            if i == start{
                new_start = count;
            }
            count += 1;
        }
    }
    for i in 0..paths.len() {
        if paths[i].len()!=2 || flows[i]!= 0 {
            for neigh in &paths[i] {
                let mut start = i;
                let mut end: usize;
                end = *neigh;
                let mut len = 1;
                while !map.contains_key(&end) {
                    for end_neigh in &paths[end] {
                        if *end_neigh != start {
                            start = end;
                            end = *end_neigh;
                            break;
                        } 
                    }
                    len += 1;
                }
                new_paths[map[&i]].push((map[&end],len));
             //   new_paths[map[&end]].push((map[&i],len));

            }
        }
    }

    
    println!("{:?}", map);
    return (new_paths, new_flows, new_start);
}


fn parse_input(file: &str) -> (Vec<Vec<usize>>, Vec<usize>, usize) {
    let lines = read_file_to_string_vec(file);
    let mut paths = vec![Vec::new(); lines.len()];
    let mut flows = vec![0;lines.len()];
    let mut map: Map<String,usize> = Map::new();
    let mut count = 0;
    let mut start = 0;
    for line in lines {
        let source_str = &line[6..=7];
        if &source_str == &"AA" {
            start = get_or_insert(source_str.trim(), &mut count, &mut map);
    //        println!("setting start to {}", start);
        }
        let flow = &line.trim()["Valve AA has flow rate=".len().. line.find(";").unwrap()];
        let dest_str = &line.trim()["Valve AA has flow rate=0; tunnels lead to valves ".len()..];
        let dest_vec = dest_str.split(", ");
        let source = get_or_insert(source_str.trim(), &mut count, &mut map);
        flows[source] = flow.parse::<usize>().unwrap();
        for dest_str in dest_vec {
            paths[source].push(get_or_insert(dest_str.trim(), &mut count, &mut map));
        }
    }
    println!("{:?}", map);
    return (paths, flows, start);
}


fn get_or_insert(key: &str,  count: &mut usize, map: &mut Map<String, usize>) -> usize {
    if !map.contains_key(key) {
        map.insert(String::from(key), *count);
        *count += 1;
    }
    return  map[key];
}