use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::str::FromStr;

/* ============================ config ============================ */

const HAND_SIZE: usize = 9;
const MAX_SCORE: u32 = 52;
const MERCY_SCORE: u32 = 32;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameMode {
    Hokm4,
    Hokm6,
    Baloot,
}

fn read_mode() -> GameMode {
    loop {
        println!("Select game mode:");
        println!("  1) Hokm (4 players)");
        println!("  2) Hokm (6 players)");
        println!("  3) Baloot");
        print!("Enter 1 / 2 / 3: ");
        io::stdout().flush().ok();

        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_ok() {
            match s.trim() {
                "1" => return GameMode::Hokm4,
                "2" => return GameMode::Hokm6,
                "3" => return GameMode::Baloot,
                _ => eprintln!("Invalid choice. Try again."),
            }
        }
    }
}

/* ============================ cards ============================ */

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    BJoker, // Black Joker
    RJoker, // Red Joker
}
impl Rank {
    fn is_joker(self) -> bool {
        matches!(self, Rank::BJoker | Rank::RJoker)
    }
    fn num_value(self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
            Rank::BJoker | Rank::RJoker => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
impl FromStr for Suit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "clubs" | "club" | "c" => Ok(Suit::Clubs),
            "diamonds" | "diamond" | "d" => Ok(Suit::Diamonds),
            "hearts" | "heart" | "h" => Ok(Suit::Hearts),
            "spades" | "spade" | "s" => Ok(Suit::Spades),
            _ => Err(format!("Unknown suit: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    rank: Rank,
    suit: Suit,
    trump: bool,
}
impl Card {
    fn new(id: u32, rank: Rank, suit: Suit) -> Self {
        Self { id, rank, suit, trump: false }
    }
    fn is_joker(&self) -> bool { self.rank.is_joker() }
    fn describe(&self) -> String {
        let name = if self.is_joker() {
            match self.rank {
                Rank::BJoker => "Black Joker".to_string(),
                Rank::RJoker => "Red Joker".to_string(),
                _ => unreachable!(),
            }
        } else {
            format!("{:?} of {:?}", self.rank, self.suit)
        };
        format!(
            "#{:02} {}{}",
            self.id,
            name,
            if self.trump { " (Trump)" } else { "" }
        )
    }
    // Black Joker > Red Joker > trump > lead-suit by rank > others.
    fn compare_trick(&self, other: &Card, lead: Suit, trump: Suit, _black_highest: bool) -> Ordering {
        fn joker_score(rank: Rank) -> u8 {
            match rank {
                Rank::BJoker => 2,
                Rank::RJoker => 1,
                _ => 0,
            }
        }
        fn key(c: &Card, lead: Suit, trump: Suit) -> (u8, u8, u8, u8) {
            let j = joker_score(c.rank);
            let t = if j == 0 && c.suit == trump { 1 } else { 0 };
            let f = if j == 0 && t == 0 && c.suit == lead { 1 } else { 0 };
            let rv = if j == 0 { c.rank.num_value() } else { 0 };
            (j, t, f, rv)
        }
        key(self, lead, trump).cmp(&key(other, lead, trump))
    }
    fn beats(&self, other: &Card, lead: Suit, trump: Suit, black_highest: bool) -> bool {
        self.compare_trick(other, lead, trump, black_highest) == Ordering::Greater
    }
}

/* ============================ deck ============================ */

#[derive(Debug, Clone)]
struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    fn new_for_mode(mode: GameMode) -> Self {
        let mut cards = Vec::new();
        let mut id = 1u32;

        let include_rank = |r: Rank| -> bool {
            match mode {
                GameMode::Hokm4 => {
                    // 6..Ace + Jokers
                    matches!(r, Rank::Six|Rank::Seven|Rank::Eight|Rank::Nine|Rank::Ten|Rank::Jack|Rank::Queen|Rank::King|Rank::Ace|Rank::BJoker|Rank::RJoker)
                }
                GameMode::Hokm6 => {
                    // all suited ranks; jokers added later
                    !matches!(r, Rank::BJoker|Rank::RJoker)
                }
                GameMode::Baloot => {
                    // 6..Ace, NO jokers
                    matches!(r, Rank::Six|Rank::Seven|Rank::Eight|Rank::Nine|Rank::Ten|Rank::Jack|Rank::Queen|Rank::King|Rank::Ace)
                }
            }
        };

        // Suited cards
        for &suit in &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for r in 2..=14 {
                let rank = match r {
                    2 => Rank::Two, 3 => Rank::Three, 4 => Rank::Four, 5 => Rank::Five,
                    6 => Rank::Six, 7 => Rank::Seven, 8 => Rank::Eight, 9 => Rank::Nine,
                    10 => Rank::Ten, 11 => Rank::Jack, 12 => Rank::Queen, 13 => Rank::King, 14 => Rank::Ace,
                    _ => unreachable!(),
                };
                if include_rank(rank) {
                    // Hokm(4): remove 6♦ and 6♣
                    if matches!(mode, GameMode::Hokm4)
                        && rank == Rank::Six
                        && (suit == Suit::Diamonds || suit == Suit::Clubs)
                    {
                        continue;
                    }
                    cards.push(Card::new(id, rank, suit));
                    id += 1;
                }
            }
        }

        // Jokers as needed
        match mode {
            GameMode::Hokm4 | GameMode::Hokm6 => {
                cards.push(Card::new(id, Rank::BJoker, Suit::Spades)); id += 1;
                cards.push(Card::new(id, Rank::RJoker, Suit::Hearts));
            }
            GameMode::Baloot => { /* no jokers */ }
        }

        Self { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn mark_trump(&mut self, trump: Suit) {
        for c in &mut self.cards { c.trump = c.suit == trump; }
    }
}

/* ============================ bidding ============================ */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bid {
    Pass,
    Five, // dealer only
    Six,
    Seven,
    Eight,
    Nine,
    Reshuffle, // dealer only if all pass
}
impl Bid {
    fn value(self) -> u8 {
        match self {
            Bid::Five => 5, Bid::Six => 6, Bid::Seven => 7, Bid::Eight => 8, Bid::Nine => 9,
            Bid::Pass | Bid::Reshuffle => 0,
        }
    }
    fn from_input(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "5" | "five" => Some(Bid::Five),
            "6" | "six" => Some(Bid::Six),
            "7" | "seven" => Some(Bid::Seven),
            "8" | "eight" => Some(Bid::Eight),
            "9" | "nine" => Some(Bid::Nine),
            "p" | "pass" => Some(Bid::Pass),
            "r" | "reshuffle" => Some(Bid::Reshuffle),
            _ => None,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Bid::Pass => "pass", Bid::Five => "5", Bid::Six => "6", Bid::Seven => "7",
            Bid::Eight => "8", Bid::Nine => "9", Bid::Reshuffle => "reshuffle",
        }
    }
}

/* ============================ players & game ============================ */

#[derive(Debug, Clone)]
struct Player {
    id: usize,
    hand: Vec<Card>,
}
impl Player {
    fn has_joker(&self) -> bool {
        self.hand.iter().any(|c| matches!(c.rank, Rank::BJoker | Rank::RJoker))
    }
    fn has_black_joker(&self) -> bool {
        self.hand.iter().any(|c| matches!(c.rank, Rank::BJoker))
    }
    fn has_red_joker(&self) -> bool {
        self.hand.iter().any(|c| matches!(c.rank, Rank::RJoker))
    }
}

#[derive(Debug)]
struct Game {
    mode: GameMode,
    num_players: usize,
    deck: Deck,
    players: Vec<Player>,
    dealer: usize,     // 0..num_players-1
    trump: Option<Suit>,
    scores: [u32; 2],  // team0: even seats, team1: odd seats
    // per-hand state
    red_played: bool,
    black_played: bool,
    black_becomes_highest_trump: bool, // reserved for future tweak
}

impl Game {
    fn new(mode: GameMode, dealer: usize) -> Self {
        let num_players = match mode {
            GameMode::Hokm4 | GameMode::Baloot => 4,
            GameMode::Hokm6 => 6,
        };
        let mut deck = Deck::new_for_mode(mode);
        deck.shuffle();
        let mut players = Vec::with_capacity(num_players);
        for i in 0..num_players {
            players.push(Player { id: i, hand: vec![] });
        }
        Self {
            mode,
            num_players,
            deck,
            players,
            dealer: dealer % num_players,
            trump: None,
            scores: [0, 0],
            red_played: false,
            black_played: false,
            black_becomes_highest_trump: false,
        }
    }

    fn reset_hand_state(&mut self) {
        self.trump = None;
        self.red_played = false;
        self.black_played = false;
        self.black_becomes_highest_trump = false;
        self.deck = Deck::new_for_mode(self.mode);
        self.deck.shuffle();
        for p in &mut self.players {
            p.hand.clear();
        }
    }

    fn team_of(&self, player: usize) -> usize { player % 2 }
    fn team_name(team: usize) -> &'static str {
        match team { 0 => "Team 0", 1 => "Team 1", _ => "Unknown" }
    }

    /// Deal anticlockwise in 3-card packets starting to the right of dealer and ending on dealer, until HAND_SIZE.
    fn deal(&mut self) {
        let mut order = Vec::with_capacity(self.num_players);
        for i in 1..=self.num_players {
            order.push((self.dealer + i) % self.num_players);
        }
        for _round in 0..(HAND_SIZE / 3) {
            for &p in &order {
                for _ in 0..3 {
                    let card = self.deck.cards.pop().expect("deck has enough cards");
                    self.players[p].hand.push(card);
                }
            }
        }
    }

    fn set_trump(&mut self, suit: Suit) {
        self.trump = Some(suit);
        for pl in &mut self.players {
            for c in &mut pl.hand {
                c.trump = c.suit == suit;
            }
        }
    }

    /* ===== Trick play validations (joker rules) ===== */

    fn validate_play(
        &self,
        player_idx: usize,
        chosen: &Card,
        lead_card_opt: Option<&Card>,
        bidder: usize,
        bidder_bid: Bid,
        trick_no: usize,
    ) -> Result<(), String> {
        // Jokers can be played ANYTIME, but Red-before-Black has constraints.
        if chosen.is_joker() {
            if matches!(chosen.rank, Rank::RJoker) && !self.black_played {
                if bidder_bid.value() >= 7
                    && self.players[bidder].has_black_joker()
                    && self.players[bidder].has_red_joker()
                    && trick_no <= 3
                {
                    return Ok(());
                } else {
                    return Err("Red Joker cannot be played before Black Joker unless the bidder (≥7) holds both, within first 3 tricks.".into());
                }
            }
            return Ok(());
        }

        // Non-jokers: must follow suit if possible
        if let Some(lead_card) = lead_card_opt {
            let lead_suit = lead_card.suit;
            let has_lead = self.players[player_idx]
                .hand
                .iter()
                .any(|c| !c.is_joker() && c.suit == lead_suit);
            if has_lead && chosen.suit != lead_suit {
                return Err(format!("You must follow suit: {:?}", lead_suit));
            }
        }

        Ok(())
    }

    fn on_card_played(&mut self, player_idx: usize, card: &Card, bidder: usize, bidder_bid: Bid, trick_no: usize) {
        if matches!(card.rank, Rank::BJoker) { self.black_played = true; }
        if matches!(card.rank, Rank::RJoker) {
            let allow_exception = bidder_bid.value() >= 7
                && self.players[bidder].has_black_joker()
                && self.players[bidder].has_red_joker()
                && trick_no <= 3;
            if allow_exception && !self.black_played {
                self.black_becomes_highest_trump = true;
            }
            self.red_played = true;
        }

        // Remove the played card from player's hand
        if let Some(pos) = self.players[player_idx]
            .hand
            .iter()
            .position(|c| c.id == card.id)
        {
            self.players[player_idx].hand.remove(pos);
        }
    }
}

/* ============================ helpers / I/O ============================ */

fn suit_order(s: Suit) -> u8 {
    match s { Suit::Clubs => 0, Suit::Diamonds => 1, Suit::Hearts => 2, Suit::Spades => 3 }
}
fn sort_hand(hand: &mut Vec<Card>) {
    // Jokers first, then by suit (C,D,H,S), then by rank (low..high)
    hand.sort_by(|a, b| {
        let aj = a.is_joker() as u8;
        let bj = b.is_joker() as u8;
        bj.cmp(&aj)
            .then(suit_order(a.suit).cmp(&suit_order(b.suit)))
            .then(a.rank.num_value().cmp(&b.rank.num_value()))
    });
}
fn joker_count(hand: &Vec<Card>) -> usize {
    hand.iter().filter(|c| c.is_joker()).count()
}

fn read_trump_from_user(who: usize) -> Suit {
    loop {
        print!("Player {}: choose trump (Clubs/Diamonds/Hearts/Spades): ", who);
        io::stdout().flush().ok();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            match Suit::from_str(&input) {
                Ok(suit) => return suit,
                Err(e) => eprintln!("{} — try again.", e),
            }
        } else {
            eprintln!("Failed to read input — try again.");
        }
    }
}

fn read_usize(prompt: &str) -> usize {
    loop {
        print!("{prompt}");
        io::stdout().flush().ok();
        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_ok() {
            if let Ok(v) = s.trim().parse::<usize>() { return v; }
        }
        eprintln!("Invalid number. Try again.");
    }
}

fn choose_card_from_hand(
    game: &Game,
    player_idx: usize,
    lead_card: Option<&Card>,
    bidder: usize,
    bidder_bid: Bid,
    trick_no: usize,
) -> Card {
    loop {
        // show hand
        let mut hand = game.players[player_idx].hand.clone();
        sort_hand(&mut hand);
        println!("Player {} [{}] hand:", player_idx, Game::team_name(game.team_of(player_idx)));
        for (i, c) in hand.iter().enumerate() {
            println!("  {}: {}", i, c.describe());
        }
        let idx = read_usize("Pick a card index: ");

        if idx >= hand.len() {
            eprintln!("Out of range.");
            continue;
        }
        let card = hand[idx].clone();

        // Find real card in current (unsorted) hand by id
        let real_pos = game.players[player_idx]
            .hand
            .iter()
            .position(|c| c.id == card.id);
        if real_pos.is_none() {
            eprintln!("Selection error, try again.");
            continue;
        }

        // Validate rules
        if let Err(msg) = game.validate_play(player_idx, &card, lead_card, bidder, bidder_bid, trick_no) {
            eprintln!("{}", msg);
            continue;
        }

        return card;
    }
}

fn show_ground(plays: &[(usize, Card)]) {
    if plays.is_empty() {
        println!("cards on ground: (none)");
    } else {
        let s = plays
            .iter()
            .map(|(_, c)| c.describe())
            .collect::<Vec<_>>()
            .join(", ");
        println!("cards on ground:\n{}", s);
    }
}

/* ============================ bidding ============================ */

impl Game {
    /// Progressive bidding in anticlockwise order (right of dealer → … → dealer).
    /// If the first (num_players-1) all pass, dealer gets the extended menu once.
    fn prompt_bid(
        &self,
        player_idx: usize,
        dealer_turn: bool,
        allow_reshuffle: bool,
        current_highest: Option<Bid>,
    ) -> Bid {
        let opposite_has_joker = if matches!(self.mode, GameMode::Hokm4) {
            let opp = (player_idx + (self.num_players / 2)) % self.num_players;
            self.players[opp].has_joker()
        } else { false };

        loop {
            print!("Player {} bid (", player_idx);
            if dealer_turn { print!("5/"); }
            print!("6/7/8/9/pass");
            if allow_reshuffle && dealer_turn { print!("/reshuffle"); }
            if let Some(h) = current_highest { print!(")  [current highest: {}] : ", h.label()); }
            else { print!("): "); }
            io::stdout().flush().ok();

            let mut s = String::new();
            if io::stdin().read_line(&mut s).is_err() { continue; }
            let mut bid = match Bid::from_input(&s) {
                Some(b) => b,
                None => { eprintln!("Invalid input."); continue; }
            };

            // Availability
            if bid == Bid::Reshuffle && !(dealer_turn && allow_reshuffle) {
                eprintln!("Reshuffle is only available to dealer when all players passed.");
                continue;
            }
            if bid == Bid::Five && !dealer_turn {
                eprintln!("Only dealer can pick 5.");
                continue;
            }
            if matches!(self.mode, GameMode::Hokm4 | GameMode::Hokm6) &&
               (bid == Bid::Eight || bid == Bid::Nine) &&
               !self.players[player_idx].has_joker()
            {
                eprintln!("To bid 8 or 9 in Hokm, you must hold at least one joker.");
                continue;
            }

            // Auto-upgrade 8 -> 9 only in Hokm(4p) if opposite teammate has a joker
            if matches!(self.mode, GameMode::Hokm4) && bid == Bid::Eight && opposite_has_joker {
                println!("Opposite teammate has a joker — upgrading bid from 8 to 9.");
                bid = Bid::Nine;
            }

            // Progressive: must outbid current highest (or pass)
            if let Some(high) = current_highest {
                if bid == Bid::Pass { return Bid::Pass; }
                if bid.value() <= high.value() {
                    eprintln!("You must bid higher than {} or pass.", high.label());
                    continue;
                }
            } else {
                if bid == Bid::Pass { return Bid::Pass; }
            }

            return bid;
        }
    }

    fn bidding_phase(&mut self) -> Option<(usize, Bid)> {
        let mut order = Vec::with_capacity(self.num_players);
        for i in 1..=self.num_players {
            order.push((self.dealer + i) % self.num_players);
        }

        let mut current_highest: Option<(usize, Bid)> = None;

        for (turn_i, &p) in order.iter().enumerate() {
            let dealer_turn = p == self.dealer;
            let allow_reshuffle_now =
                dealer_turn && current_highest.is_none() && turn_i == order.len() - 1;

            let bid = self.prompt_bid(p, dealer_turn, allow_reshuffle_now, current_highest.map(|(_, b)| b));

            if bid == Bid::Reshuffle {
                let enemy = 1 - self.team_of(self.dealer);
                self.scores[enemy] = self.scores[enemy].saturating_add(1);
                println!("Dealer reshuffled. +1 point to {}.", Self::team_name(enemy));
                return None;
            }
            if bid == Bid::Pass {
                if allow_reshuffle_now {
                    println!("Dealer passed after all-pass — hand aborted.");
                    return None;
                }
                continue;
            }

            match current_highest {
                None => current_highest = Some((p, bid)),
                Some((_, h)) if bid.value() > h.value() => current_highest = Some((p, bid)),
                _ => {}
            }
        }

        current_highest
    }
}

/* ============================ one hand (deal->bid->tricks->score) ============================ */

fn play_one_hand(game: &mut Game) {
    // Deal & show hands
    game.deal();

    println!("\nHands dealt anti-clockwise (3x3 -> 9 each). Dealer = Player {}.", game.dealer);
    for p in 0..game.num_players {
        sort_hand(&mut game.players[p].hand);
        let j = joker_count(&game.players[p].hand);
        println!(
            "Player {} [{}] ({} cards, {} joker{}):",
            p,
            Game::team_name(game.team_of(p)),
            game.players[p].hand.len(),
            j,
            if j == 1 { "" } else { "s" }
        );
        for c in &game.players[p].hand {
            println!("  {}", c.describe());
        }
    }

    /* ------------------ BIDDING ------------------ */
    let Some((bidder, bid)) = game.bidding_phase() else {
        println!(
            "Hand aborted. Scores: Team0={} Team1={}",
            game.scores[0], game.scores[1]
        );
        return;
    };
    println!(
        "Bid winner: Player {} ({}) -> {}",
        bidder,
        Game::team_name(game.team_of(bidder)),
        bid.label()
    );

    /* ------------------ TRUMP (chosen by bid winner) ------------------ */
    let trump = read_trump_from_user(bidder);
    game.set_trump(trump);
    game.deck.mark_trump(trump);
    println!("\nTrump is {:?}.", trump);

    /* ------------------ PLAY UP TO 9 TRICKS (with early end) ------------------ */
    // Leader is the player to the RIGHT (anticlockwise next) of the trump-chooser
    let mut leader = (bidder + 1) % game.num_players;

    let mut tricks_won = [0u8, 0u8];
    let mut black_played_by_round3 = false;
    let bid_target = bid.value() as u8;
    let bidder_team = game.team_of(bidder);

    // early_end signals we stopped due to math certainty
    let mut early_end = false;

    for trick_no in 1usize..=9 {
        println!("\n=== Trick {}/9 ===", trick_no);

        let mut order = Vec::with_capacity(game.num_players);
        for i in 0..game.num_players {
            order.push((leader + i) % game.num_players);
        }

        let mut lead_card: Option<Card> = None;
        let mut plays: Vec<(usize, Card)> = Vec::with_capacity(game.num_players);

        for &p in &order {
            println!("\nTurn: Player {} [{}]", p, Game::team_name(game.team_of(p)));

            // Show what's already on the table BEFORE this player acts
            show_ground(&plays);

            let chosen = choose_card_from_hand(&game, p, lead_card.as_ref(), bidder, bid, trick_no);
            game.on_card_played(p, &chosen, bidder, bid, trick_no);

            if game.black_played && trick_no <= 3 {
                black_played_by_round3 = true; // mark as soon as Black is seen in first 3 tricks
            }

            // Announce the play immediately so the next player sees it in the ground list
            println!("Player {} played {}", p, chosen.describe());

            if lead_card.is_none() {
                lead_card = Some(chosen.clone());
            }
            plays.push((p, chosen));
        }

        // Decide winner of the trick
        let lead_suit = plays[0].1.suit;
        let trump_suit = game.trump.expect("trump set");
        let mut win_idx = 0usize;
        for i in 1..plays.len() {
            if plays[i].1.beats(&plays[win_idx].1, lead_suit, trump_suit, game.black_becomes_highest_trump) {
                win_idx = i;
            }
        }
        let winner_player = plays[win_idx].0;
        let winner_team = game.team_of(winner_player);
        tricks_won[winner_team] += 1;

        // SHOW whole trick after completion
        println!("\nPlayed this trick (leader first):");
        for (p, c) in &plays {
            println!("  Player {} ({}): {}", p, Game::team_name(game.team_of(*p)), c.describe());
        }

        println!(
            "\nTrick winner: Player {} ({}) with {}",
            winner_player,
            Game::team_name(winner_team),
            plays[win_idx].1.describe()
        );

        println!(
            "[Round Status] Bid: {} ({} needs {}) | Tricks so far: Team0={} Team1={} | Tricks remaining: {}",
            bid.label(),
            Game::team_name(bidder_team),
            bid_target,
            tricks_won[0],
            tricks_won[1],
            9 - trick_no
        );

        // Early end logic:
        let remaining: u8 = (9 - trick_no) as u8;

        // 1) Bidder already met the target
        if tricks_won[bidder_team] >= bid_target {
            println!(
                "\n[Auto End] {} reached their bid of {} tricks and wins the hand early!",
                Game::team_name(bidder_team),
                bid_target
            );
            early_end = true;
            break;
        }
        // 2) Bidder cannot reach target anymore (not enough tricks left)
        if tricks_won[bidder_team] + remaining < bid_target {
            println!(
                "\n[Auto End] {} can no longer reach {} tricks. Opponents win the hand!",
                Game::team_name(bidder_team),
                bid_target
            );
            early_end = true;
            break;
        }

        // Next trick leader is the winner
        leader = winner_player;
    }

    /* ------------------ SCORING & PENALTIES ------------------ */
    let hand_winner_team = if tricks_won[0] > tricks_won[1] { 0 } else { 1 };
    let bidder_tricks = tricks_won[bidder_team];
    let x = bid.value();

    // Penalty: Black Joker not played by end of trick 3
    if !black_played_by_round3 {
        game.scores[hand_winner_team] =
            game.scores[hand_winner_team].saturating_add(15);
        println!("Penalty: Black Joker not played by end of trick 3 -> +15 to {}.", Game::team_name(hand_winner_team));
    }

    // Main scoring (meet bid)
    if bidder_tricks >= x as u8 {
        game.scores[bidder_team] = game.scores[bidder_team].saturating_add(x as u32);
        println!("{} met the bid {} -> +{}", Game::team_name(bidder_team), x, x);
    } else {
        let opp = 1 - bidder_team;
        game.scores[opp] = game.scores[opp].saturating_add((2 * x) as u32);
        println!("{} failed the bid {} -> {} +{}", Game::team_name(bidder_team), x, Game::team_name(opp), 2 * x);
    }

    for t in 0..2 {
        if game.scores[t] > MAX_SCORE { game.scores[t] = MAX_SCORE; }
    }

    println!(
        "\nFINAL TRICK TALLY — Team 0: {} | Team 1: {}{}",
        tricks_won[0], tricks_won[1],
        if early_end { "  (ended early)" } else { "" }
    );
    println!(
        "SCORES — Team 0: {} | Team 1: {}",
        game.scores[0], game.scores[1]
    );
}

/* ============================ full match loop with dealer rotation ============================ */

fn main() {
    let mode = read_mode();
    let num_players = match mode {
        GameMode::Hokm4 | GameMode::Baloot => 4,
        GameMode::Hokm6 => 6,
    };

    // Randomize dealer for the FIRST hand only
    let mut dealer = thread_rng().gen_range(0..num_players);

    let mut game = Game::new(mode, dealer);

    println!(
        "\nMode: {:?} | Players: {}",
        game.mode, game.num_players
    );
    match game.num_players {
        4 => println!("Teams: Team 0 = Players 0 & 2 | Team 1 = Players 1 & 3"),
        6 => println!("Teams: Team 0 = Players 0,2,4 | Team 1 = Players 1,3,5"),
        _ => {}
    }

    loop {
        println!("\n================ HAND START (Dealer = Player {}) ================", dealer);

        // Reset per-hand state & use current dealer
        game.dealer = dealer;
        game.reset_hand_state();

        // Play a hand
        play_one_hand(&mut game);

        // End conditions
        if game.scores[0] >= MAX_SCORE || game.scores[1] >= MAX_SCORE {
            println!("\nGAME OVER — Final Scores: Team 0: {} | Team 1: {}", game.scores[0], game.scores[1]);
            break;
        }
        if (game.scores[0] == MERCY_SCORE && game.scores[1] == 0) ||
           (game.scores[1] == MERCY_SCORE && game.scores[0] == 0) {
            println!("\nGAME OVER (Mercy) — Final Scores: Team 0: {} | Team 1: {}", game.scores[0], game.scores[1]);
            break;
        }

        // Dealer rotation rules based on TOTAL scores:
        // If the dealer's team now leads in total points, dealer moves to the right (anticlockwise).
        // Otherwise the same player remains the dealer.
        let dealer_team = game.team_of(dealer);
        let other_team = 1 - dealer_team;
        if game.scores[dealer_team] > game.scores[other_team] {
            dealer = (dealer + 1) % num_players; // move right (anticlockwise)
            println!("Dealer’s team leads. Dealer moves to Player {} (to the right).", dealer);
        } else {
            println!("Dealer’s team does not lead. Dealer stays Player {}.", dealer);
        }
    }
}

