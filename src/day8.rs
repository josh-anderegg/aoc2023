use std::{fs, collections::{HashMap, HashSet}};
use num::Integer;

pub fn solve() -> (u64, u64){
    let input = fs::read_to_string("inputs/8.txt").unwrap();
    let mut lines = input.lines().into_iter();
    let directions = lines.next().unwrap();
    // Skip the empty line
    lines.next();
    let mut tree = HashMap::new();
    let mut starting_nodes = Vec::new();
    let mut ending_nodes = HashSet::new();
    for line in lines{
        let (from, to) = line.split_once(" = ").unwrap();
        if let Some(i) = to.find(','){
            let l = to[1..i].trim().to_string();
            let r = to[i+1..to.len()-1].trim().to_string();
            tree.insert(from.to_string(), (l,r));
        }
        match from.chars().nth(2).unwrap(){
            'A' => {starting_nodes.push(from.to_string());()},
            'Z' => {ending_nodes.insert(from.to_string());()},
            _ => (),
        }
    }   
    
    let mut i = 0;
    let modulo = directions.len();
    let goal = "ZZZ".to_string();
    let mut cur = "AAA".to_string();
    while cur != goal {
        let (l, r) = tree.get(&cur).unwrap();
        match directions.chars().nth(i%modulo){
            Some('L') => String::clone_from(&mut cur, l),
            Some('R') => String::clone_from(&mut cur, r),
            _ => panic!("unexpected"),
        }
        i+=1;
    }
    let sol1 = i as u64;

    let mut steps_coll = Vec::new();
    for node in starting_nodes{
        let mut cur = String::new();
        String::clone_from(&mut cur, &node);
        let mut step = 0;
        loop {
            let (l, r) = tree.get(&cur).unwrap();
            let dir = directions.chars().nth(step%modulo).unwrap();
            // println!("Before assigning {} {}", l, r);
            match dir{
                'L' => String::clone_from(&mut cur, l),
                'R' => String::clone_from(&mut cur, r),
                _ => panic!("unexpected"),
            }
            // println!("After assigning {} {}", l, r);
            
            if ending_nodes.contains(&cur){   
                steps_coll.push(step+1);
                break;
            }
            step += 1;
        }
        
    }
    let mut sol2 = 1;
    for n in steps_coll.iter(){
        sol2 = sol2.lcm(n);
    }
    let sol2 = sol2.try_into().unwrap();
    return (sol1, sol2);
}
