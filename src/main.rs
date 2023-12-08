use std::{env, time::Instant};
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

type SolverFn = fn() -> (u64, u64);
const SOLVERS: [SolverFn; 25] = [day1::solve, day2::solve, day3::solve, day4::solve, day5::solve, day6::solve, day7::solve, day8::solve, day9::solve, day10::solve, day11::solve, day12::solve,
                                 day13::solve, day14::solve, day15::solve, day16::solve, day17::solve, day18::solve, day19::solve, day20::solve, day21::solve, day22::solve, day23::solve, day24::solve,day25::solve]; 
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Solving all days:");
        for (day_nr, solver) in SOLVERS.iter().enumerate(){
            let now = Instant::now();
            let (sol1, sol2) = solver();
            let elapsed = now.elapsed();
            println!("Solution for day {}: ({}, {}) calculated in {:.2?}", day_nr+1, sol1, sol2, elapsed);
        }
        return;
    }
    let day_ind:usize = args[1].parse::<usize>().expect("Invalid argument type, exptected int");
    let now = Instant::now();
    let (sol1, sol2) = SOLVERS.get(day_ind-1).expect("Invalid argument size, days range from 1-25")();
    let elapsed = now.elapsed();
    println!("Solution for day {}: ({}, {}) calculated in {:.2?}",args[1] , sol1, sol2, elapsed);

}



