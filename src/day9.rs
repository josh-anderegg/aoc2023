use std::fs;

pub fn solve() -> (u64, u64){
    let mut sol1 = 0;
    let mut sol2 = 0;
    let input = fs::read_to_string("inputs/9.txt").unwrap();
    for line in input.lines(){
        let history:Vec<i64> = line.split_whitespace().map(|nr| nr.parse::<i64>().unwrap()).collect();
        let (res1, res2) = interpolate(&history);
        sol1 += res1;
        sol2 += res2;
    }
    let sol1 = sol1 as u64;
    let sol2 = sol2 as u64;

    return (sol1, sol2);
}

fn interpolate(history:&Vec<i64>) -> (i64, i64){
    let mut cur = Vec::clone(&history);
    let mut vals = Vec::new();
    while cur.iter().fold(false, |acc, nr| *nr != 0 || acc){
        vals.push((*cur.last().unwrap(), *cur.first().unwrap()));
        cur = next_layer(&cur);
    }
    let mut res1 = 0;
    let mut res2 = 0;
    for (i,j) in vals.iter().rev(){
        res1 = *i + res1;
        res2 = *j -res2;
    }
    return (res1, res2);
}

fn next_layer(history:&Vec<i64>) -> Vec<i64>{
    let mut ret = Vec::new();
    for i in 0..history.len()-1{
        let diff = history[i+1] - history[i];
        ret.push(diff);
    }
    
    return ret
}