import random
from collections import Counter

# Define the deck of cards
suits = ['hearts', 'diamonds', 'clubs', 'spades']
ranks = list(range(2, 15))  # 2-10, Jack=11, Queen=12, King=13, Ace=14
deck = [(rank, suit) for suit in suits for rank in ranks]

def is_flush(hand, player_suit):
    suit_counts = Counter(suit for rank, suit in hand if suit == player_suit)
    return max(suit_counts.values()) >= 5

def is_gapper(hand):
    suited_cards = [rank for rank, suit in hand if suit == hand[0][1]]
    suited_cards.sort()
    for i in range(len(suited_cards) - 2):
        if suited_cards[i+2] - suited_cards[i] == 2:
            return True
    return False

def simulate_hand(hand_type):
    random.shuffle(deck)
    if hand_type == 'gapper':
        suit = random.choice(suits)
        middle_rank = random.choice(ranks[1:-1])  # Exclude the lowest and highest ranks to have room for a gapper
        player_hand = [(middle_rank - 1, suit), (middle_rank + 1, suit)]
    else:  # any two suited cards
        suit = random.choice(suits)
        player_hand = [(rank, suit) for rank in random.sample(ranks, 2)]
    community_cards = [card for card in deck if card not in player_hand][:5]
    all_cards = player_hand + community_cards
    return is_flush(all_cards, suit)

def estimate_probabilities(n=1_000_000):
    gapper_flush_counts = 0
    suited_flush_counts = 0
    for _ in range(n):
        if simulate_hand('gapper'):
            gapper_flush_counts += 1
        if simulate_hand('suited'):
            suited_flush_counts += 1
    return gapper_flush_counts / n, suited_flush_counts / n

gapper_flush_prob, suited_flush_prob = estimate_probabilities()
print(f"Gapper flush probability: {gapper_flush_prob}")
print(f"Suited flush probability: {suited_flush_prob}")
