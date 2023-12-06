use std::fs;
#[derive(Debug)]
struct Map {
    domain: Range,
    offset: i64
}
#[derive(Debug)]
struct Range {
    from: i64,
    to: i64
}

pub fn solve() -> (u32, u32){
    let mut sol1:i64 = 1<<62;
    let mut sol2:i64 = 1<<62;
    let input = fs::read_to_string("inputs/5.txt").expect("File not found");
    let mut seed_l:Vec<i64> = Vec::new(); 
    let mut layers:Vec<Vec<Map>> = Vec::new();
    for (line_nr, line) in input.lines().enumerate(){
        if line_nr == 0 {
            let (_,seed_str) = line.split_once(':').unwrap();
            for seed in seed_str.split_whitespace(){
                let val = seed.parse::<i64>().unwrap();
                seed_l.push(val);
            }
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        if line.chars().next().unwrap().is_alphabetic(){
            let vec = Vec::new();
            layers.push(vec);
            continue;
        }
        let last_index = layers.len()-1;
        let cur_map = layers.get_mut(last_index).unwrap();
        let mut val_strs = line.split_whitespace();
        let dst = val_strs.next().unwrap().parse::<i64>().unwrap();
        let src = val_strs.next().unwrap().parse::<i64>().unwrap();
        let range = val_strs.next().unwrap().parse::<i64>().unwrap();
        cur_map.push(Map{domain: Range{from:src, to:src+range -1},offset: dst-src});
    }
    
    let mut cur_ranges = Vec::new();
    let l = seed_l.chunks(2);
    for pair in l {
        cur_ranges.push(Range{from: pair[0], to: pair[0]+pair[1]-1});
    }

    // Part 1 calc
    for seed in seed_l {
        let mut cur_seed = seed;
        'layer: for layer in layers.iter(){
            for map in layer.iter() {
                match map_val(map, cur_seed){
                    Some(n) => {cur_seed=n; continue 'layer;}
                    None => ()
                }
            }   
        }
        if cur_seed < sol1{
            sol1 = cur_seed;
        }
    }

    for layer in layers.iter(){
        let mut next_ranges = Vec::new();
        while cur_ranges.len() != 0{
            let cur_range = cur_ranges.pop().unwrap();
            let mut no_match = true;
            for map in layer{
                match intersect(&cur_range, &map.domain){
                    (None, _) => (),
                    (Some(inter), rest) => {
                        no_match = false;
                        next_ranges.push(map_range(map, &inter));
                        for r in rest{
                            cur_ranges.push(r);
                        }
                    },
                }
            }            
            if no_match {
                next_ranges.push(cur_range);
            }        
        }
        cur_ranges = next_ranges;
    }
    for r in cur_ranges{
        if r.from < sol2{
            sol2 = r.from;
        }
    }
    let min = sol1.try_into().unwrap();
    let sol2 = sol2.try_into().unwrap();
    return (min, sol2);
}

fn map_val(map:&Map, val:i64) -> Option<i64> {
    if in_domain(map, val){
        return Some(val + map.offset);
    } else{
        return None;
    }
}

fn map_range(map:&Map, range:&Range) -> Range {
    return Range{from: range.from+map.offset, to: range.to + map.offset};
}

fn in_domain(map:&Map, val:i64) -> bool {
    return val >= map.domain.from && val <= map.domain.to 
}

fn intersect(src_range:&Range, map_range:&Range)-> (Option<Range>, Vec<Range>){
    let mut rest = Vec::new();
    if src_range.to < map_range.from || map_range.to < src_range.from { // Maps have no intersection
        return (None, rest);
    }
    let intersection = Range{
        from: std::cmp::max(src_range.from, map_range.from),
        to: std::cmp::min(src_range.to, map_range.to)
    };
    if intersection.from > src_range.from{
        rest.push(Range{from: src_range.from, to: intersection.from -1 });
    }
    if intersection.to < src_range.to {
        rest.push(Range{from: intersection.to+1, to: src_range.to});
    }
    return (Some(intersection), rest);
}


