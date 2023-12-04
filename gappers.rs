use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Suit {
    Hearts, Diamonds, Clubs, Spades,
}

enum HandType {
    Gapper,
    Suited,
}

fn is_flush(hand: &Vec<(Rank, Suit)>, suit: &Suit) -> bool {
    hand.iter().filter(|&card| card.1 == *suit).count() >= 5
}

fn simulate_hand(hand_type: HandType) -> bool {
    let mut rng = rand::thread_rng();
    let suits = vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
    let ranks: Vec<Rank> = vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
    let mut deck: Vec<(Rank, Suit)> = ranks.iter().flat_map(|&rank| suits.iter().map(move |&suit| (rank, suit))).collect();
    deck.shuffle(&mut rng);
    let suit = suits.choose(&mut rng).unwrap();
    let player_hand;
    match hand_type {
        HandType::Gapper => {
            let middle_rank_index = rng.gen_range(1..ranks.len() - 1);
            player_hand = vec![(ranks[middle_rank_index - 1], *suit), (ranks[middle_rank_index + 1], *suit)];
        },
        HandType::Suited => {
            let player_ranks: Vec<Rank> = ranks.choose_multiple(&mut rng, 2).cloned().collect();
            player_hand = player_ranks.into_iter().map(|rank| (rank, *suit)).collect();
        },
    }
    let community_cards: Vec<(Rank, Suit)> = deck.into_iter().filter(|card| !player_hand.contains(card)).take(5).collect();
    let mut all_cards = player_hand.clone();
    all_cards.extend(community_cards);
    is_flush(&all_cards, suit)
}

fn estimate_probabilities(n: i32) -> (f64, f64) {
    let mut gapper_flush_counts = 0;
    let mut suited_flush_counts = 0;
    for _ in 0..n {
        if simulate_hand(HandType::Gapper) {
            gapper_flush_counts += 1;
        }
        if simulate_hand(HandType::Suited) {
            suited_flush_counts += 1;
        }
    }
    (gapper_flush_counts as f64 / n as f64, suited_flush_counts as f64 / n as f64)
}

fn main() {
    let (gapper_flush_prob, suited_flush_prob) = estimate_probabilities(100_000_000);
    println!("Gapper flush probability: {}", gapper_flush_prob);
    println!("Suited flush probability: {}", suited_flush_prob);
}
