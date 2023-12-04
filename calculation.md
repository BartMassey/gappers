# Suited Flush Probability
Bart Massey 2023

The probability of a flush in Texas Hold-Em given a suited
player hand held until the river is calculated as follows:

* There are 50 cards unobserved by the player after the
  initial deal. Any of these could become the community
  cards. There are therefore *d = choose(50, 5)* possible
  community deals.

* The community deck contains 11 cards of the player's suit:
  without loss of generality we will call this suit
  spades. The player needs at least three spades in
  the community hand. There are three cases:
  
  * Exactly three spades in the community hand. The hands
    of this form consist of three spades and two non-spades.
    There are *n3 = choose(11, 3) × choose(39, 2)* such hands.

  * Exactly four spades in the community hand. The hands
    of this form consist of four spades and one non-spade.
    There are *n4 = choose(11, 4) × choose(39, 1)* such hands.

  * Exactly five spades in the community hand. The hands
    of this form consist of five spades.
    There are *n5 = choose(11, 5)* such hands.

* The number of community deals that give spade flushes when
  the player holds spades is *n = n3 + n4 + n5*.

* The overall probability of a player flush is *n / d*, which
  works out to about 6.38%.

* The preceding calculation neglects as irrelevant the
  probability that there will be a flush in some other suit:
  there are *n0 = 3 * choose(13, 5)* such hands. *n0 / d* is
  about 0.18%, so *(n + n0) / d* is about 6.57%.
  
It is interesting that the numbers calculated here differ
slightly from reports in the literature. 6.5% is commonly
reported, but 6.4% or 6.6% are more accurate estimates,
depending on whether irrelevant flushes are counted. This
could be a bug in these calculations, but it seems unlikely.

This calculation has no dependency on which two suited cards
(or on which suit) the player holds.
