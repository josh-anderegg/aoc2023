use std::fs;

pub fn solve() -> (u64, u64){
    let mut sol1 = 0;
    let mut sol2 = 0;

    let input = fs::read_to_string("inputs/4.txt").expect("File not found"); 
    let mut dp:Vec<u64> = Vec::new();
    for line in input.lines(){
        let (_, clean_line) = line.split_once(':').unwrap();
        let (wnumbs_str , ynumbs_str) = clean_line.split_once('|').unwrap();
        let ynumbs = parse_list(ynumbs_str);
        let wnumbs = parse_list(wnumbs_str);
        
        let mut nr_matches = 0;
        let mut end = 0;
        'outer: for y in ynumbs.iter(){
            for w in wnumbs.iter(){
                if *y == *w {
                    nr_matches +=1;
                    if end == 0 {
                        end = 1;
                    } else{
                        end *= 2;
                    }
                    continue 'outer;
                }
            }

        }
        dp.push(nr_matches);
        sol1 += end;
    }
    let mut register:Vec<u64> = vec![1; dp.len()]; 
    for i in 0..dp.len(){
        let j = dp[i];
        let r:usize = j.try_into().unwrap();
        let r =  r + i;
        for k in i+1..r+1{
            if k >= register.len(){
                break;
            }
            register[k] += register[i];
        }
        sol2 += register[i];
    }
    return (sol1, sol2);
}

fn parse_list(list:&str) -> Vec<u64>{
    let mut ret:Vec<u64> = Vec::new(); 
    let nr_list = list.split_whitespace();
    for nr_str in nr_list{
        let val = nr_str.parse::<u64>().unwrap();
        ret.push(val);
    }
    return ret;
}