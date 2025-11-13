use rand::seq::SliceRandom;
use rand::thread_rng;
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
    // Trick strength ordering
    fn compare_trick(&self, other: &Card, lead: Suit, trump: Suit) -> Ordering {
        // (is_red_joker, is_black_joker, is_trump, follows_lead, rank_value)
        fn key(c: &Card, lead: Suit, trump: Suit) -> (u8, u8, u8, u8, u8) {
            let is_red = matches!(c.rank, Rank::RJoker) as u8;
            let is_black = matches!(c.rank, Rank::BJoker) as u8;
            let is_trump = (c.suit == trump) as u8;
            let follows = (c.suit == lead) as u8;
            let rankv = if c.is_joker() { 0 } else { c.rank.num_value() } as u8;
            (is_red, is_black, is_trump, follows, rankv)
        }
        key(self, lead, trump).cmp(&key(other, lead, trump))
    }
    fn beats(&self, other: &Card, lead: Suit, trump: Suit) -> bool {
        self.compare_trick(other, lead, trump) == Ordering::Greater
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
                    // only 6..Ace + Jokers
                    matches!(r, Rank::Six|Rank::Seven|Rank::Eight|Rank::Nine|Rank::Ten|Rank::Jack|Rank::Queen|Rank::King|Rank::Ace|Rank::BJoker|Rank::RJoker)
                }
                GameMode::Hokm6 => {
                    // all ranks + Jokers
                    !matches!(r, Rank::BJoker|Rank::RJoker) // add jokers later explicitly
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
                    2 => Rank::Two,
                    3 => Rank::Three,
                    4 => Rank::Four,
                    5 => Rank::Five,
                    6 => Rank::Six,
                    7 => Rank::Seven,
                    8 => Rank::Eight,
                    9 => Rank::Nine,
                    10 => Rank::Ten,
                    11 => Rank::Jack,
                    12 => Rank::Queen,
                    13 => Rank::King,
                    14 => Rank::Ace,
                    _ => unreachable!(),
                };
                if include_rank(rank) {
                    // Extra exclusions for Hokm (4p): remove 6♦ and 6♣
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
            GameMode::Hokm4 => {
                cards.push(Card::new(id, Rank::BJoker, Suit::Spades)); id += 1;
                cards.push(Card::new(id, Rank::RJoker, Suit::Hearts));
            }
            GameMode::Hokm6 => {
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
            Bid::Five => 5,
            Bid::Six => 6,
            Bid::Seven => 7,
            Bid::Eight => 8,
            Bid::Nine => 9,
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
            Bid::Pass => "pass",
            Bid::Five => "5",
            Bid::Six => "6",
            Bid::Seven => "7",
            Bid::Eight => "8",
            Bid::Nine => "9",
            Bid::Reshuffle => "reshuffle",
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
    trump: Suit,
    scores: [u32; 2],  // team0: even seats, team1: odd seats
    trick_no: usize,   // 1..=9
    red_played: bool,
    black_played: bool,
    black_holder_team: Option<usize>,
    black_becomes_highest_trump: bool,
}

impl Game {
    fn new(mode: GameMode, dealer: usize, trump: Suit) -> Self {
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
            trump,
            scores: [0, 0],
            trick_no: 0,
            red_played: false,
            black_played: false,
            black_holder_team: None,
            black_becomes_highest_trump: false,
        }
    }

    fn team_of(&self, player: usize) -> usize { player % 2 } // even indices team0, odd team1
    fn team_name(team: usize) -> &'static str {
        match team {
            0 => "Team 0",
            1 => "Team 1",
            _ => "Unknown",
        }
    }

    /// Deal anticlockwise in 3-card packets starting to the right of dealer and ending on dealer, until HAND_SIZE.
    fn deal(&mut self) {
        // Build anticlockwise order: right of dealer first, wrap around, dealer last
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

        // Track black-joker holder team (if a joker exists in this mode)
        let holder = self.players.iter().position(|pl| pl.has_black_joker());
        self.black_holder_team = holder.map(|idx| self.team_of(idx));
    }

    /// Ask for a bid; must be strictly higher than current_highest (or pass).
    /// Dealer can bid 5 only if there is no current_highest (i.e., first bid).
    /// 8/9 require the bidder to hold ≥1 joker.
    /// In Hokm (4p) only: if bid == 8 and the opposite teammate has a joker, auto-upgrade to 9.
    fn prompt_bid(
        &self,
        player_idx: usize,
        dealer_turn: bool,
        allowed_reshuffle: bool,
        current_highest: Option<Bid>,
    ) -> Bid {
        // Opposite teammate only meaningful in 4p Hokm
        let teammate_has_joker = if matches!(self.mode, GameMode::Hokm4) {
            let teammate_idx = (player_idx + (self.num_players / 2)) % self.num_players; // opposite seat
            self.players[teammate_idx].has_joker()
        } else {
            false
        };

        loop {
            print!("Player {} bid (", player_idx);
            if dealer_turn { print!("5/"); }
            print!("6/7/8/9/pass");
            if allowed_reshuffle && dealer_turn { print!("/reshuffle"); }
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
            if bid == Bid::Reshuffle && !(dealer_turn && allowed_reshuffle) {
                eprintln!("Reshuffle is only available to dealer when all players passed.");
                continue;
            }
            if bid == Bid::Five && !dealer_turn {
                eprintln!("Only dealer can pick 5.");
                continue;
            }
            if (bid == Bid::Eight || bid == Bid::Nine) && !self.players[player_idx].has_joker() {
                eprintln!("To bid 8 or 9, you must hold at least one joker.");
                continue;
            }

            // Auto-upgrade 8 -> 9 only in Hokm(4p) if opposite teammate has a joker
            if matches!(self.mode, GameMode::Hokm4) && bid == Bid::Eight && teammate_has_joker {
                println!("Opposite teammate has a joker — upgrading bid from 8 to 9.");
                bid = Bid::Nine;
            }

            // Progressive rule: must outbid current highest (or pass)
            if let Some(high) = current_highest {
                if bid == Bid::Pass { return Bid::Pass; }
                if bid.value() <= high.value() {
                    eprintln!("You must bid higher than {} or pass.", high.label());
                    continue;
                }
            } else {
                // First bid of the round
                if bid == Bid::Pass { return Bid::Pass; }
            }

            return bid;
        }
    }

    /// Progressive single-round bidding in anticlockwise order: right of dealer ... dealer.
    /// If all pass, dealer may choose 5/6/7/8/9/pass/reshuffle (reshuffle gives +1 to enemy team and aborts hand).
    fn bidding_phase(&mut self) -> Option<(usize, Bid)> {
        // order of players to ask
        let mut order = Vec::with_capacity(self.num_players);
        for i in 1..=self.num_players {
            order.push((self.dealer + i) % self.num_players);
        }

        let mut current_highest: Option<(usize, Bid)> = None;
        for &p in &order {
            let dealer_turn = p == self.dealer;
            let bid = self.prompt_bid(p, dealer_turn, false, current_highest.map(|(_, b)| b));
            if bid != Bid::Pass {
                match current_highest {
                    None => current_highest = Some((p, bid)),
                    Some((_, h)) if bid.value() > h.value() => current_highest = Some((p, bid)),
                    _ => { /* guarded by prompt */ }
                }
            }
        }

        if current_highest.is_none() {
            // Everyone passed: dealer special menu
            println!("All players passed. Dealer may choose 5/6/7/8/9/pass/reshuffle");
            let bid = self.prompt_bid(self.dealer, true, true, None);
            return match bid {
                Bid::Reshuffle => {
                    let enemy = 1 - self.team_of(self.dealer);
                    self.scores[enemy] = self.scores[enemy].saturating_add(1);
                    println!("Dealer reshuffled. +1 point to {}.", Self::team_name(enemy));
                    None
                }
                Bid::Pass => {
                    println!("Dealer passed after all-pass — round aborted.");
                    None
                }
                _ => Some((self.dealer, bid)),
            };
        }

        current_highest
    }

    /* ===== Joker validations & scoring hooks (as before) ===== */

    fn validate_play(
        &self,
        _player_idx: usize,
        card: &Card,
        lead_card_opt: Option<&Card>,
        bidder: usize,
        bidder_bid: Bid,
    ) -> Result<(), String> {
        // Cannot lead jokers
        if lead_card_opt.is_none() && card.is_joker() {
            return Err("Jokers cannot be led as first card of a trick.".into());
        }
        // Red cannot precede Black unless bidder >=7 and holds both jokers
        if matches!(card.rank, Rank::RJoker) && !self.black_played {
            if bidder_bid.value() >= 7 {
                let bidder_has_both =
                    self.players[bidder].has_black_joker() && self.players[bidder].has_red_joker();
                if bidder_has_both { return Ok(()); }
            }
            return Err("Red Joker cannot be played before Black Joker (unless bidder ≥7 and holds both).".into());
        }
        Ok(())
    }

    fn apply_red_before_black_exception(&mut self) {
        self.black_becomes_highest_trump = true;
    }

    fn on_card_played(&mut self, card: &Card, _team_of_player: usize) {
        if matches!(card.rank, Rank::BJoker) { self.black_played = true; }
        if matches!(card.rank, Rank::RJoker) { self.red_played = true; }
    }

    fn apply_scoring(
        &mut self,
        bidder: usize,
        bid: Bid,
        bidder_tricks_won: u8,
        bound_called: bool,
        bound_success: bool,
        black_played_by_round3: bool,
        red_forced_first_last_card: bool,
        winners_of_hand_team: usize,
    ) {
        let bidder_team = self.team_of(bidder);
        let opp_team = 1 - bidder_team;

        if !black_played_by_round3 {
            self.scores[winners_of_hand_team] =
                self.scores[winners_of_hand_team].saturating_add(15);
            println!("Penalty: Black Joker not played in first 3 tricks -> +15 to {}.", Self::team_name(winners_of_hand_team));
        }
        if red_forced_first_last_card {
            self.scores[opp_team] = self.scores[opp_team].saturating_add(15);
            println!("Penalty: Red Joker forced to lead as last card -> +15 to {}.", Self::team_name(opp_team));
        }

        if bound_called {
            if bound_success {
                self.scores[self.team_of(bidder)] = MAX_SCORE;
                println!("Bound success! {} now at {}", Self::team_name(self.team_of(bidder)), self.scores[self.team_of(bidder)]);
                return;
            } else {
                self.scores[opp_team] = self.scores[opp_team].saturating_add(18);
                println!("Bound failed. {} +18 to {}", Self::team_name(opp_team), self.scores[opp_team]);
                return;
            }
        }

        let x = bid.value();
        if x >= 5 {
            if bidder_tricks_won >= x {
                self.scores[bidder_team] = self.scores[bidder_team].saturating_add(x as u32);
                println!("{} made {} -> +{}", Self::team_name(bidder_team), x, x);
            } else {
                self.scores[opp_team] = self.scores[opp_team].saturating_add((2 * x) as u32);
                println!("{} failed {} -> {} +{}", Self::team_name(bidder_team), x, Self::team_name(opp_team), 2 * x);
            }
        }

        for t in 0..2 {
            if self.scores[t] > MAX_SCORE { self.scores[t] = MAX_SCORE; }
        }
        if (self.scores[0] == MERCY_SCORE && self.scores[1] == 0) ||
           (self.scores[1] == MERCY_SCORE && self.scores[0] == 0) {
            println!("Mercy rule: {}–0 @32 -> game ends.", MERCY_SCORE);
        }
    }
}

/* ============================ utils ============================ */

fn read_trump_from_user() -> Suit {
    loop {
        print!("Choose trump (Clubs/Diamonds/Hearts/Spades): ");
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

/* ============================ demo main ============================ */

fn main() {
    let mode = read_mode();
    let num_players = match mode {
        GameMode::Hokm4 | GameMode::Baloot => 4,
        GameMode::Hokm6 => 6,
    };

    let dealer = 0; // rotate each hand in a real loop
    let trump = read_trump_from_user();

    let mut game = Game::new(mode, dealer, trump);
    game.deal();

    // Teams
    println!(
        "\nMode: {:?} | Players: {} | Dealer: {} | Trump: {:?}",
        game.mode, game.num_players, game.dealer, game.trump
    );
    match game.num_players {
        4 => println!("Teams: Team 0 = Players 0 & 2 | Team 1 = Players 1 & 3"),
        6 => println!("Teams: Team 0 = Players 0,2,4 | Team 1 = Players 1,3,5"),
        _ => {}
    }

    // Hand sizes + joker info
    println!("\nHands dealt anti-clockwise (3x3 -> 9 each).");
    for p in 0..num_players {
        println!(
            "Player {} [{}] ({} cards){}",
            p,
            Game::team_name(game.team_of(p)),
            game.players[p].hand.len(),
            if game.players[p].has_joker() { " [has joker]" } else { "" }
        );
    }

    // Progressive bidding
    let Some((bidder, bid)) = game.bidding_phase() else {
        println!("Round ended due to all-pass w/ dealer decision. Scores: Team0={} Team1={}", game.scores[0], game.scores[1]);
        return;
    };
    println!("Bid winner: Player {} ({}) -> {}", bidder, Game::team_name(game.team_of(bidder)), bid.label());

    // If bidder >=7 and holds both jokers, the red-before-black exception can apply during play.
    if bid.value() >= 7 && game.players[bidder].has_black_joker() && game.players[bidder].has_red_joker() {
        println!("Bidder holds both jokers and bid ≥7: red-before-black exception can apply during play.");
    }

    // ---- Trick loop is still a stub; prompt results to exercise scoring ----
    let bidder_tricks_won = read_usize("\nEnter bidder team tricks won (0..=9): ") as u8;

    let mut bound_called = false;
    let mut bound_success = false;
    if bid == Bid::Seven {
        println!("Did bidder call 'bound' with 3 rounds left? (y/n)");
        let mut s = String::new();
        io::stdin().read_line(&mut s).ok();
        bound_called = s.trim().eq_ignore_ascii_case("y");
        if bound_called {
            println!("Did 'bound' succeed (won all remaining)? (y/n)");
            s.clear();
            io::stdin().read_line(&mut s).ok();
            bound_success = s.trim().eq_ignore_ascii_case("y");
        }
    }

    println!("Was Black Joker played by end of trick 3? (y/n)");
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let black_by3 = s.trim().eq_ignore_ascii_case("y");

    println!("Was Red Joker forced to be led as the last card? (y/n)");
    s.clear();
    io::stdin().read_line(&mut s).ok();
    let red_forced_first_last = s.trim().eq_ignore_ascii_case("y");

    println!("Which team won the hand (0 or 1)?");
    s.clear();
    io::stdin().read_line(&mut s).ok();
    let winners_team = s.trim().parse::<usize>().unwrap_or(0).min(1);

    game.apply_scoring(
        bidder,
        bid,
        bidder_tricks_won,
        bound_called,
        bound_success,
        black_by3,
        red_forced_first_last,
        winners_team,
    );

    println!("\nScores now: Team0 = {}, Team1 = {}", game.scores[0], game.scores[1]);
}
