use std::fs;
const SPELLING_LIST: [(&str, char);9] = [("one", '1'), ("two",'2'), ("three",'3'), ("four",'4'), ("five",'5'), ("six",'6'), ("seven",'7'), ("eight",'8'), ("nine",'9')];

pub fn solve() -> (u64, u64){
    let input = fs::read_to_string("inputs/1.txt")
        .expect("File doesn't exist");

    // Part 1
    let mut sum1 = 0;
    for line in input.lines(){
        let mut fst = ' ';
        let mut snd = ' ';
        
        for c in line.chars(){
            if c.is_digit(10) {
                if fst == ' '{
                    fst = c;
                }
                snd = c;
            }
        }
        let numb = fst.to_digit(10).unwrap() * 10 + snd.to_digit(10).unwrap();
        sum1 += numb;
              
    }

    // Part 2
    let mut sum2 = 0;
    for line in input.lines(){
        let mut fst = ' ';
        let mut snd = ' ';

        for (i, c) in line.char_indices() {
            if c.is_digit(10){
                if fst == ' '{
                    fst = c;
                }
                snd = c;
            } else {
                match is_spelled(&line[i..]) {
                    Some(c) => {if fst == ' ' {fst = c;}; snd = c;},
                    None => ()
                }
            }
        }
        let numb = fst.to_digit(10).unwrap() * 10 + snd.to_digit(10).unwrap();
        sum2 += numb;
    }
    let sum1 = sum1 as u64;
    let sum2 = sum2 as u64;
    return (sum1, sum2);
}
fn is_spelled(str: &str) -> Option<char> {
    for (numb, lit) in SPELLING_LIST{
        if str.starts_with(numb){
            return Some(lit)
        }
    }
    return None;
}
