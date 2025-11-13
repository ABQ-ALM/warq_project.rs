enum Rank
{
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
    Bjoker = 15,
    Rjoker = 16,
}
enum Suite
{
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
} 
struct Card
{
    face: bool,
    id: u32,
    rank: Rank,
    suite: Suite,
    Trump: bool
}
struct Deck
{
    deck: Vec<Card> 
}
impl Cards
{
    // write new functions you think should be add and I will think that logic 
    fn new(face: bool, id: u32, rank: Rank, suite: Suite, Trump: bool) -> Self 
    {
        Self {face, id, rank, suite, trump};
    }
    fn from_id(id: u32) -> Self
    fn flip(&mut self)
    fn reveal(&mut self)
    fn hide(&mut self)
    fn describe(&mut self)
    fn color
    fn is_face_up
    fn is_joker
    fn equals
    fn beats
    fn same_suit
}
impl Deck
{
    fn new() -> Self
    {
        let mut deck = Vec::new();
    }
/* List of all the cards in the deck*/
/* List of all the clubs cards */
let two_clubs    = Card { face: false, id: 1,  rank: Rank::Two,   suit: Suit::Clubs,    trump: false };
let three_clubs  = Card { face: false, id: 2,  rank: Rank::Three, suit: Suit::Clubs,    trump: false };
let four_clubs   = Card { face: false, id: 3,  rank: Rank::Four,  suit: Suit::Clubs,    trump: false };
let five_clubs   = Card { face: false, id: 4,  rank: Rank::Five,  suit: Suit::Clubs,    trump: false };
let six_clubs    = Card { face: false, id: 5,  rank: Rank::Six,   suit: Suit::Clubs,    trump: false };
let seven_clubs  = Card { face: false, id: 6,  rank: Rank::Seven, suit: Suit::Clubs,    trump: false };
let eight_clubs  = Card { face: false, id: 7,  rank: Rank::Eight, suit: Suit::Clubs,    trump: false };
let nine_clubs   = Card { face: false, id: 8,  rank: Rank::Nine,  suit: Suit::Clubs,    trump: false };
let ten_clubs    = Card { face: false, id: 9,  rank: Rank::Ten,   suit: Suit::Clubs,    trump: false };
let jack_clubs   = Card { face: false, id: 10, rank: Rank::Jack,  suit: Suit::Clubs,    trump: false };
let queen_clubs  = Card { face: false, id: 11, rank: Rank::Queen, suit: Suit::Clubs,    trump: false };
let king_clubs   = Card { face: false, id: 12, rank: Rank::King,  suit: Suit::Clubs,    trump: false };
let ace_clubs    = Card { face: false, id: 13, rank: Rank::Ace,   suit: Suit::Clubs,    trump: false };
/* List of all the diamonds cards */
let two_diamonds    = Card { face: false, id: 14, rank: Rank::Two,   suit: Suit::Diamonds, trump: false };
let three_diamonds  = Card { face: false, id: 15, rank: Rank::Three, suit: Suit::Diamonds, trump: false };
let four_diamonds   = Card { face: false, id: 16, rank: Rank::Four,  suit: Suit::Diamonds, trump: false };
let five_diamonds   = Card { face: false, id: 17, rank: Rank::Five,  suit: Suit::Diamonds, trump: false };
let six_diamonds    = Card { face: false, id: 18, rank: Rank::Six,   suit: Suit::Diamonds, trump: false };
let seven_diamonds  = Card { face: false, id: 19, rank: Rank::Seven, suit: Suit::Diamonds, trump: false };
let eight_diamonds  = Card { face: false, id: 20, rank: Rank::Eight, suit: Suit::Diamonds, trump: false };
let nine_diamonds   = Card { face: false, id: 21, rank: Rank::Nine,  suit: Suit::Diamonds, trump: false };
let ten_diamonds    = Card { face: false, id: 22, rank: Rank::Ten,   suit: Suit::Diamonds, trump: false };
let jack_diamonds   = Card { face: false, id: 23, rank: Rank::Jack,  suit: Suit::Diamonds, trump: false };
let queen_diamonds  = Card { face: false, id: 24, rank: Rank::Queen, suit: Suit::Diamonds, trump: false };
let king_diamonds   = Card { face: false, id: 25, rank: Rank::King,  suit: Suit::Diamonds, trump: false };
let ace_diamonds    = Card { face: false, id: 26, rank: Rank::Ace,   suit: Suit::Diamonds, trump: false };
/* List of all the hearts cards */
let two_hearts    = Card { face: false, id: 27, rank: Rank::Two,   suit: Suit::Hearts,    trump: false };
let three_hearts  = Card { face: false, id: 28, rank: Rank::Three, suit: Suit::Hearts,    trump: false };
let four_hearts   = Card { face: false, id: 29, rank: Rank::Four,  suit: Suit::Hearts,    trump: false };
let five_hearts   = Card { face: false, id: 30, rank: Rank::Five,  suit: Suit::Hearts,    trump: false };
let six_hearts    = Card { face: false, id: 31, rank: Rank::Six,   suit: Suit::Hearts,    trump: false };
let seven_hearts  = Card { face: false, id: 32, rank: Rank::Seven, suit: Suit::Hearts,    trump: false };
let eight_hearts  = Card { face: false, id: 33, rank: Rank::Eight, suit: Suit::Hearts,    trump: false };
let nine_hearts   = Card { face: false, id: 34, rank: Rank::Nine,  suit: Suit::Hearts,    trump: false };
let ten_hearts    = Card { face: false, id: 35, rank: Rank::Ten,   suit: Suit::Hearts,    trump: false };
let jack_hearts   = Card { face: false, id: 36, rank: Rank::Jack,  suit: Suit::Hearts,    trump: false };
let queen_hearts  = Card { face: false, id: 37, rank: Rank::Queen, suit: Suit::Hearts,    trump: false };
let king_hearts   = Card { face: false, id: 38, rank: Rank::King,  suit: Suit::Hearts,    trump: false };
let ace_hearts    = Card { face: false, id: 39, rank: Rank::Ace,   suit: Suit::Hearts,    trump: false };
/* List of all the spades cards */
let two_spades    = Card { face: false, id: 40, rank: Rank::Two,   suit: Suit::Spades,    trump: false };
let three_spades  = Card { face: false, id: 41, rank: Rank::Three, suit: Suit::Spades,    trump: false };
let four_spades   = Card { face: false, id: 42, rank: Rank::Four,  suit: Suit::Spades,    trump: false };
let five_spades   = Card { face: false, id: 43, rank: Rank::Five,  suit: Suit::Spades,    trump: false };
let six_spades    = Card { face: false, id: 44, rank: Rank::Six,   suit: Suit::Spades,    trump: false };
let seven_spades  = Card { face: false, id: 45, rank: Rank::Seven, suit: Suit::Spades,    trump: false };
let eight_spades  = Card { face: false, id: 46, rank: Rank::Eight, suit: Suit::Spades,    trump: false };
let nine_spades   = Card { face: false, id: 47, rank: Rank::Nine,  suit: Suit::Spades,    trump: false };
let ten_spades    = Card { face: false, id: 48, rank: Rank::Ten,   suit: Suit::Spades,    trump: false };
let jack_spades   = Card { face: false, id: 49, rank: Rank::Jack,  suit: Suit::Spades,    trump: false };
let queen_spades  = Card { face: false, id: 50, rank: Rank::Queen, suit: Suit::Spades,    trump: false };
let king_spades   = Card { face: false, id: 51, rank: Rank::King,  suit: Suit::Spades,    trump: false };
let ace_spades    = Card { face: false, id: 52, rank: Rank::Ace,   suit: Suit::Spades,    trump: false };
/* List of the joker cards */
let black_joker = Card { face: false, id: 53, rank: Rank::BJoker, suit: Suit::Spades,  trump: false };
let red_joker   = Card { face: false, id: 54, rank: Rank::RJoker, suit: Suit::Hearts,  trump: false };source "$HOME/.cargo/env"

/* A function that shuffles the deck */
fn shuffle()
}

