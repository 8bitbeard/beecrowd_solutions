use std::io;

fn main() {
    let mut hand_cards = String::new();
    io::stdin().read_line(&mut hand_cards).expect("Failed to read line");
    let mut hand_cards: Vec<u32> = hand_cards.trim().split(' ').map(|x| x.parse::<u32>().unwrap()).collect();

    hand_cards.sort();

    if hand_cards[0] == hand_cards[1] {
        println!("{}", hand_cards[0]);
    } else {
        println!("{}", hand_cards[1]);
    }
}
