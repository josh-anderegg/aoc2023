use std::{fs, collections::HashMap, fmt};
#[derive(Debug)]
struct Hand {
    kind_rank1: u32,
    kind_rank2: u32, // Rank with joker utilized
    cards: Vec<u32>, 
    cards_j : Vec<u32>, // cards with joker=1
    bid: u32,
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hand_str_norm:String = self.cards.iter().map(|v| card_char(*v)).collect();
        let hand_str_joker:String = self.cards_j.iter().map(|v| card_char(*v)).collect();
        write!(f, "Normal rules: {hand_str_norm} with rank: {}\nJoker rules {hand_str_joker}: with rank {}", self.kind_rank1, self.kind_rank2)
    }
}
impl  Hand {
    fn new(cards:Vec<u32>, cards_j:Vec<u32>, bid: u32) -> Hand {
        let kind_rank1 = Hand::rank_from_cards(&cards, false);
        let kind_rank2 = Hand::rank_from_cards(&cards, true);
        return Hand{kind_rank1, kind_rank2, cards, cards_j, bid};
    }
    
    fn rank_from_cards(cards:&Vec<u32>, joker: bool) -> u32{
        let mut occ_count:HashMap<u32, u32> = HashMap::new();
        let mut j_count = 0;
        for i in 0..5 {
            if joker && cards[i] == 11 {
                j_count += 1;
                continue;
            }   
            *occ_count.entry(cards[i]).or_insert(0) += 1;
        }
        let mut rank = 0;
        for (_, count) in occ_count{
            rank = match (count, rank) {
                (1, _) => rank,
                (2, 0) => 1,
                (2, 1) => 2,
                (2, 3) => 4,
                (3, 0) => 3,
                (3, 1) => 4,
                (4, 0) => 5,
                (5, 0) => 6,
                _ => panic!("thought impossible"), 
            };
        
        }
        return match (joker, j_count, rank) {
            (false, _, _) => rank,
            (true, 0, _) => rank,
            (true, 1, 0) => 1,
            (true, 1, 1) => 3,
            (true, 1, 2) => 4,
            (true, 1, 3) => 5,
            (true, 1, 5) => 6,
            (true, 2, 0) => 3,
            (true, 2, 1) => 5,
            (true, 2, 3) => 6,
            (true, 3, 0) => 5,
            (true, 3, 1) => 6,
            (true, 4, 0) => 6,
            (true, 5, 0) => 6, 
            _ => panic!("thought impossible: j_count {j_count}, {rank} {:?}", cards),
        };
    }
}
pub fn solve() -> (u32, u32){
    let mut sol1 = 0;
    let mut sol2 = 0;
    let input = fs::read_to_string("inputs/7.txt").unwrap();

    let mut hands:Vec<Hand> = Vec::new();
    for line in input.lines(){
        let (cards_str, bid_str) = line.split_once(' ').unwrap();
        let cards:Vec<u32> = cards_str.chars().map(|c| card_val(c, false)).collect(); // Cards with J = 11
        let cards_j:Vec<u32> = cards_str.chars().map(|c| card_val(c, true)).collect(); // Cards with J = 1
        let bid = bid_str.parse::<u32>().unwrap();
        let hand = Hand::new(cards, cards_j, bid);
        hands.push(hand); 
    }
    hands.sort_by(|h1, h2| {h1.kind_rank1.cmp(&h2.kind_rank1)
        .then_with(|| h1.cards.cmp(&h2.cards))
    });
    for (i, h) in hands.iter().enumerate() {
        sol1 += (i as u32 + 1) * h.bid;
    }
    hands.sort_by(|h1, h2| {h1.kind_rank2.cmp(&h2.kind_rank2)
        .then_with(|| h1.cards_j.cmp(&h2.cards_j))});
    
    for (i, h) in hands.iter().enumerate() {
        sol2 += (i as u32 + 1) * h.bid;
    }
    return (sol1, sol2);
}

fn card_val (c:char, j:bool) -> u32 {
    match c.is_digit(10) {
        true => c.to_digit(10).unwrap(),
        false => match c {
            'T' => 10,
            'J' => if !j {11} else{1},
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("unexpected card type") 
        }
    }
}

fn card_char (i:u32) -> char {
    match i  {
        1 => 'J',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'T',
        11 => 'P',
        12 => 'Q',
        13 => 'K',
        14 => 'A',
        _ => panic!("Not a valid card number {i}"),
    }
}

