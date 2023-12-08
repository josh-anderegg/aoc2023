use std::{fs::{self}, usize, collections::{HashMap, HashSet}};
#[derive(Debug)]
enum Elem {
    N(char),
    S,
    E,
    G,
}

pub fn solve() -> (u64, u64) {
    let input = fs::read_to_string("inputs/3.txt").expect("File not found");
    let mut sol1 = 0;
    let mut sol2 = 0;
    let mut grid: Vec<Vec<Elem>> = Vec::new();
    let mut gear_list: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    for line in input.lines(){
        let mut row:Vec<Elem> = Vec::new();
        for c in line.chars(){
            row.push(
                match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => Elem::N(c),
                '.' => Elem::E,
                '*' => Elem::G,
                _ => Elem::S,
            })
            
        }
        row.push(Elem::E);
        grid.push(row);
    }
    
    for (i, row) in grid.iter().enumerate(){
        let mut acc = String::new();
        let mut is_adj = false;
        let mut adj_gears = HashSet::new();
        for (j, elem) in row.iter().enumerate(){
            match elem {
                Elem::N(c) => {
                    is_adj = is_adj || is_adj_to_sym(&grid, (i,j));
                    let next = add_adj_gears(&grid, (i,j));
                    for t in next{
                        adj_gears.insert(t);
                    }
                    acc.push(*c);
                },
                _ => {
                    if  acc.len() > 0 && is_adj{
                        let val= acc.parse::<u64>().unwrap_or(0);
                        for gear in adj_gears.iter(){
                            let vec = gear_list.entry(*gear).or_insert(vec![]);
                            vec.push(val);
                        }
                        sol1 += val;
                    } 
                    adj_gears = HashSet::new();
                    acc = String::new();
                    is_adj = false;
                }
            }
        }
    }

    for vec in gear_list.values(){
        if vec.len() == 2 {
            sol2 += vec[0] * vec[1];
        }
    }
    return (sol1,sol2);
}

fn is_adj_to_sym (grid:&Vec<Vec<Elem>>, (x,y):(usize,usize)) -> bool {
    let n:i32 = grid.len().try_into().unwrap();
    let m:i32 = grid[0].len().try_into().unwrap();
    for l in (-1)..2{
        for k in (-1)..2{
            let t1:i32 = x.try_into().unwrap();
            let t2:i32 = y.try_into().unwrap();
            let t1 = t1+l;
            let t2 = t2+k;
            if t1 >= 0 && t1 < n && t2 >= 0 && t2 < m{
                let t1:usize = t1.try_into().unwrap();
                let t2:usize = t2.try_into().unwrap(); 
                if t1 == 0 && t2 == 0 {continue;}

                match grid[t1][t2] {
                    Elem::S | Elem::G => return true,
                    _ => ()
                }
            }
        }
    }
    return false;
}

fn add_adj_gears (grid:&Vec<Vec<Elem>>, (x,y):(usize,usize)) -> Vec<(usize,usize)>{
    let n:i32 = grid.len().try_into().unwrap();
    let m:i32 = grid[0].len().try_into().unwrap();
    let mut gears = vec![];
    for l in (-1)..2{
        for k in (-1)..2{
            let t1:i32 = x.try_into().unwrap();
            let t2:i32 = y.try_into().unwrap();
            let t1 = t1+l;
            let t2 = t2+k;
            if t1 >= 0 && t1 < n && t2 >= 0 && t2 < m{
                let t1:usize = t1.try_into().unwrap();
                let t2:usize = t2.try_into().unwrap(); 
                if t1 == 0 && t2 == 0 {continue;}

                match grid[t1][t2] {
                    Elem::G => gears.push((t1,t2)),
                    _ => ()
                }
            }
        }
    }
    return gears;
}

