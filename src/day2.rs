use std::fs;

pub fn solve() -> (u32, u32)  {
    let input = fs::read_to_string("inputs/2.txt")
    .expect("File not found");
    let mut sol1 = 0;
    let mut sol2 = 0;
    
    let max = [12, 13, 14]; // (R, G, B)
    for (id, line) in input.lines().enumerate() {
        let (_,line_clean) = line.split_once(':').unwrap();
        let mut all_true = true;
        let mut min = [0,0,0];

        let showcases = line_clean.split(';');
        for show in showcases {
            let mut rgb = [0,0,0];
            let colors = show.split(',');
            for color in colors {
                let mut clean_color = color.split_whitespace();
                let num_str = clean_color.next().unwrap();
                let color_str = clean_color.next().unwrap();

                let tup_pos = match color_str {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("unexpected")
                };
                let num = num_str.parse::<u32>().unwrap();
                rgb[tup_pos] = num;      
            }
            for i in 0..3{
                if max[i] < rgb[i]{
                    all_true = false;
                }
                if rgb[i] > min[i]{
                    min[i] = rgb[i];
                }
            }
        }    
        if all_true {
            sol1 += u32::try_from(id + 1).unwrap();
        } 
        sol2 += min[0] * min[1] * min[2];   
    }
    
    return (sol1, sol2)
}