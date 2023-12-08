use std::fs;

pub fn solve() -> (u64, u64){
    let mut sol1 = 1;
    let input = fs::read_to_string("inputs/6.txt").unwrap();
    let mut lines = input.lines().into_iter();
    let (_, time_row_str) = lines.next().unwrap().split_once(':').unwrap();
    let (_, dist_row_str) = lines.next().unwrap().split_once(':').unwrap();
    let time_row:Vec<u64> = time_row_str
        .split_whitespace()
        .map(|x| x.parse::<u64>())
        .filter_map(Result::ok)
        .collect();
    
    let dist_row:Vec<u64> = dist_row_str
        .split_whitespace()
        .map(|x| x.parse::<u64>())
        .filter_map(Result::ok)
        .collect();
    
    for (goal, time) in dist_row.into_iter().zip(time_row.into_iter()){

        let mut loc_win_count = 0;
        for speed in 0..time{
            let run_time = time - speed;
            if speed * run_time > goal{
                loc_win_count +=1;
            }
        }
        sol1 *= loc_win_count;
    }

    let t_str:String = time_row_str.chars().filter(|c| !c.is_whitespace()).collect();  
    let t = t_str.parse::<u64>().unwrap();  
    let d_str:String = dist_row_str.chars().filter(|c| !c.is_whitespace()).collect();  
    let d = d_str.parse::<u64>().unwrap();  


    let sqrt = (((t * t - 4 * d) as f64)).sqrt();
    let mf1 = (sqrt - (t as f64))/(-2.0); 
    let mf2 = (-sqrt - (t as f64))/(-2.0);
    let solmf = (mf2 -mf1).floor();
    let sol2 = solmf as u64;
    // let mut win_count:u64 = 0;
    // for speed in 0..t{
    //     let run_time = t - speed;
    //     if speed * run_time > d{
    //         win_count += 1;
    //     }
    // }
    // sol2 = win_count;
    return (sol1, sol2);
}