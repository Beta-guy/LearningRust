#[derive(Debug)]
struct Deck{

    cards: Vec<String>,

}


fn main() {
    let suits = ["Hearts", "Spades", "Dianmods", "Clubs"];
    let values = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];
    let mut cards = vec![];
    for suit in suits{
        for value in values{
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }
    let deck: Deck = Deck{cards};
    println!("Here is your deck: {:#?}.", deck);
}
