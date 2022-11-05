use std::io;

struct Card {
    attributes: Vec<u32>,
}

impl Card {
    fn get_attribute(self: &Self, idx: usize) -> u32 {
        match self.attributes.get(idx) {
            Some(x) => *x,
            None => 0,
        }
    }
}

struct Player {
    name: String,
    cards: Vec<Card>,
}

impl Player {
    fn add_card(self: &mut Self, card: Card) {
        self.cards.push(card);
    }

    fn get_card(self: &Self, idx: usize) -> &Card {
        &self.cards[idx]
    }
}

fn main() {
    loop {
        let mut marcos = Player {
            name: "Marcos".to_string(),
            cards: Vec::new(),
        };
        let mut leonardo = Player {
            name: "Leonardo".to_string(),
            cards: Vec::new(),
        };

        let mut att_qty = String::new();
        io::stdin()
            .read_line(&mut att_qty)
            .expect("Failed to read the line");

        let _att_qty: u32 = match att_qty.trim() {
            "" => break,
            x => x.parse::<u32>().unwrap(),
        };

        let mut cards_amount = String::new();
        io::stdin()
            .read_line(&mut cards_amount)
            .expect("Failed to read the line!");
        let cards_amount: Vec<u32> = cards_amount
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for _i in 0..cards_amount[0] {
            let mut card_data = String::new();
            io::stdin()
                .read_line(&mut card_data)
                .expect("Failed to read the line!");
            let card_data: Vec<u32> = card_data
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            let card = Card {
                attributes: card_data,
            };
            marcos.add_card(card)
        }

        for _i in 0..cards_amount[1] {
            let mut card_data = String::new();
            io::stdin()
                .read_line(&mut card_data)
                .expect("Failed to read the line!");
            let card_data: Vec<u32> = card_data
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            let card = Card {
                attributes: card_data,
            };
            leonardo.add_card(card)
        }

        let mut choosed_cards = String::new();
        io::stdin()
            .read_line(&mut choosed_cards)
            .expect("Failed to read the line");
        let choosed_cards: Vec<usize> = choosed_cards
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();

        let marcos_card = marcos.get_card(choosed_cards[0]);
        let leonardo_card = leonardo.get_card(choosed_cards[1]);

        let mut choosed_attr = String::new();
        io::stdin()
            .read_line(&mut choosed_attr)
            .expect("Failed to read the line");
        let choosed_attr: usize = choosed_attr.trim().parse::<usize>().unwrap() - 1;

        match compare(
            &marcos_card.get_attribute(choosed_attr),
            &leonardo_card.get_attribute(choosed_attr),
        ) {
            -1 => println!("{}", leonardo.name),
            0 => println!("Empate"),
            _ => println!("{}", marcos.name),
        }
    }
}

fn compare(a: &u32, b: &u32) -> i32 {
    if *a < *b {
        -1
    } else if *a == *b {
        0
    } else {
        1
    }
}
