use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}
#[derive(Debug, Copy, Clone)]
enum Type {
    HighCard(Card),
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct HandAndBid {
    hand: Vec<Card>,
    kind: Type,
    bid: u32,
    primary_strength: u32,
}

impl HandAndBid {
    fn extra_points(&self) -> u32 {
        // calculate extra points for the hand: first card gets most points, second card gets second most points, etc.
        // points are summed together.
        let mut extra_points = 0;
        for (index, card) in self.hand.iter().enumerate() {
            let value = *card as u32;
            let shift = 4 - index as u32;
            extra_points += value << (shift * 4); // "hexadecimals"
        }
        extra_points
    }
}

// test cases for extra_points:
#[test]
fn test_extra_points() {
    let hand = HandAndBid {
        hand: vec![Card::King, Card::Ace, Card::Nine, Card::Nine, Card::Nine],
        bid: 1,
        kind: Type::FiveOfAKind,
        primary_strength: 18,
    };
    let hand2 = HandAndBid {
        hand: vec![Card::Ace, Card::Two, Card::Eight, Card::Ace, Card::Ace],
        bid: 1,
        kind: Type::FourOfAKind,
        primary_strength: 17,
    };
    println!("hand: extra points {}", hand.extra_points());
    println!("hand2: extra points {}", hand2.extra_points());
    assert!(hand2.extra_points() > hand.extra_points());
}

impl PartialEq for HandAndBid {
    fn eq(&self, other: &Self) -> bool {
        self.primary_strength == other.primary_strength
            && self.extra_points() == other.extra_points()
    }
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.primary_strength != other.primary_strength {
            return self.primary_strength.partial_cmp(&other.primary_strength);
        }
        if self.primary_strength == other.primary_strength {
            return self.extra_points().partial_cmp(&other.extra_points());
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.primary_strength != other.primary_strength {
            return self
                .primary_strength
                .partial_cmp(&other.primary_strength)
                .unwrap();
        }
        if self.primary_strength == other.primary_strength {
            return self
                .extra_points()
                .partial_cmp(&other.extra_points())
                .unwrap();
        }
        std::cmp::Ordering::Equal
    }
}

impl Eq for HandAndBid {}

fn determine_hand(hand: &Vec<Card>) -> Type {
    // 5 same cards gets 6 points:
    let all_are_the_same = hand.iter().all(|&card| card == hand[0]);
    if all_are_the_same {
        return Type::FiveOfAKind;
    }

    let counts = hand.iter().fold([0; 13], |mut counts, &card| {
        match card {
            Card::Two => counts[0] += 1,
            Card::Three => counts[1] += 1,
            Card::Four => counts[2] += 1,
            Card::Five => counts[3] += 1,
            Card::Six => counts[4] += 1,
            Card::Seven => counts[5] += 1,
            Card::Eight => counts[6] += 1,
            Card::Nine => counts[7] += 1,
            Card::Ten => counts[8] += 1,
            Card::Jack => counts[9] += 1,
            Card::Queen => counts[10] += 1,
            Card::King => counts[11] += 1,
            Card::Ace => counts[12] += 1,
        }
        counts
    });

    // 4 same cards
    for count in counts.iter() {
        if *count == 4 {
            return Type::FourOfAKind;
        }
    }

    // full house: 3 same and 2 same
    let mut has_three = false;
    let mut has_two = 0;
    for count in counts.iter() {
        if *count == 3 {
            has_three = true;
        }
        if *count == 2 {
            has_two += 1;
        }
    }
    if has_three && has_two == 1 {
        return Type::FullHouse;
    }

    // 3 same
    if has_three {
        return Type::ThreeOfAKind;
    }

    // 2 pairs
    if has_two == 2 {
        return Type::TwoPairs;
    }

    // 1 pair
    if has_two == 1 {
        return Type::Pair;
    }

    // high card: determine the most valuable card
    let mut sorted_hand = hand.clone();
    sorted_hand.sort();
    let highest_card = sorted_hand.last().unwrap();

    Type::HighCard(*highest_card)
}

fn calculate_strength(kind: &Type) -> u32 {
    match kind {
        Type::HighCard(_card) => 12,
        Type::Pair => 14,
        Type::TwoPairs => 15,
        Type::ThreeOfAKind => 16,
        Type::FullHouse => 17,
        Type::FourOfAKind => 18,
        Type::FiveOfAKind => 19,
    }
}

fn parse_file(filename: &str) -> Result<Vec<HandAndBid>, std::io::Error> {
    let mut hands = Vec::new();
    let contents = fs::read_to_string(filename)?;
    for line in contents.lines() {
        // each line is a list of cards (not separated by spaces) and a numeric bid (separated from the hand by a space)
        let parts = line.split(" ").collect::<Vec<&str>>();

        // parse the hand:
        let hand = parts[0].chars().collect::<Vec<char>>();
        let mut hand_vec = Vec::new();
        for card in hand {
            match card {
                '2' => hand_vec.push(Card::Two),
                '3' => hand_vec.push(Card::Three),
                '4' => hand_vec.push(Card::Four),
                '5' => hand_vec.push(Card::Five),
                '6' => hand_vec.push(Card::Six),
                '7' => hand_vec.push(Card::Seven),
                '8' => hand_vec.push(Card::Eight),
                '9' => hand_vec.push(Card::Nine),
                'T' => hand_vec.push(Card::Ten),
                'J' => hand_vec.push(Card::Jack),
                'Q' => hand_vec.push(Card::Queen),
                'K' => hand_vec.push(Card::King),
                'A' => hand_vec.push(Card::Ace),
                _ => panic!("Invalid card: {}", card),
            }
        }
        // parse the bid:
        let bid = parts[1].parse::<u32>().unwrap();

        // all cards but not Jack:
        let replacement_cards = vec![
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::Queen,
            Card::King,
            Card::Ace,
        ];

        let kind = determine_hand(&hand_vec);

        // get the indexes of Jacks in the hand:
        let mut jacks = Vec::new();
        for (index, card) in hand_vec.iter().enumerate() {
            if *card == Card::Jack {
                jacks.push(index);
            }
        }
        let mut best_strength = calculate_strength(&kind);
        let mut best_kind = kind;

        if !jacks.is_empty() {
            // jacks vector contains the indexes of Jacks in the hand.
            // Length of the jacks vector tells us how many combinations
            // of replacement we need to check:

            let jacks_count = jacks.len();

            if jacks_count == 4 {
                for card1 in replacement_cards.iter() {
                    for card2 in replacement_cards.iter() {
                        for card3 in replacement_cards.iter() {
                            for card4 in replacement_cards.iter() {
                                let mut hand_vec2 = hand_vec.clone();
                                hand_vec2[jacks[0]] = *card1;
                                hand_vec2[jacks[1]] = *card2;
                                hand_vec2[jacks[2]] = *card3;
                                hand_vec2[jacks[3]] = *card4;
                                let better_kind = &determine_hand(&hand_vec2);
                                let strength2 = calculate_strength(&better_kind);
                                //println!("testing: {:?} is {:?}", hand_vec2, better_kind);
                                if strength2 > best_strength {
                                    best_strength = strength2;
                                    best_kind = *better_kind;
                                    //println!("better hand: {:?} is {:?}", hand_vec2, better_kind);
                                }
                            }
                        }
                    }
                }
            } else if jacks_count == 3 {
                for card1 in replacement_cards.iter() {
                    for card2 in replacement_cards.iter() {
                        for card3 in replacement_cards.iter() {
                            let mut hand_vec2 = hand_vec.clone();
                            hand_vec2[jacks[0]] = *card1;
                            hand_vec2[jacks[1]] = *card2;
                            hand_vec2[jacks[2]] = *card3;
                            let better_kind = &determine_hand(&hand_vec2);
                            let strength2 = calculate_strength(&better_kind);
                            //println!("testing: {:?} is {:?}", hand_vec2, better_kind);
                            if strength2 > best_strength {
                                best_strength = strength2;
                                best_kind = *better_kind;
                                //println!("better hand: {:?} is {:?}", hand_vec2, better_kind);
                            }
                        }
                    }
                }
            } else if jacks_count == 2 {
                for card1 in replacement_cards.iter() {
                    for card2 in replacement_cards.iter() {
                        let mut hand_vec2 = hand_vec.clone();
                        hand_vec2[jacks[0]] = *card1;
                        hand_vec2[jacks[1]] = *card2;
                        let better_kind = &determine_hand(&hand_vec2);
                        let strength2 = calculate_strength(&better_kind);
                        //println!("testing: {:?} is {:?}", hand_vec2, better_kind);
                        if strength2 > best_strength {
                            best_strength = strength2;
                            best_kind = *better_kind;
                            //println!("better hand: {:?} is {:?}", hand_vec2, better_kind);
                        }
                    }
                }
            } else if jacks_count == 1 {
                for card1 in replacement_cards.iter() {
                    let mut hand_vec2 = hand_vec.clone();
                    hand_vec2[jacks[0]] = *card1;
                    let better_kind = &determine_hand(&hand_vec2);
                    let strength2 = calculate_strength(&better_kind);
                    //println!("testing: {:?} is {:?}", hand_vec2, better_kind);
                    if strength2 > best_strength {
                        best_strength = strength2;
                        best_kind = *better_kind;
                        //println!("better hand: {:?} is {:?}", hand_vec2, better_kind);
                    }
                }
            }
        }

        //println!("done");

        hands.push(HandAndBid {
            hand: hand_vec,
            bid: bid,
            kind: best_kind,
            primary_strength: best_strength,
        });
    }
    Ok(hands)
}

fn calculate_points(filename: &str) -> Result<u64, std::io::Error> {
    let mut hands = parse_file(filename)?;

    hands.sort();
    hands.reverse();

    let count = hands.len();

    println!("total : {} hands", count);
    let mut sum: u64 = 0;
    for (index, hand) in hands.iter().enumerate() {
        let rank = (count - index) as u32;
        let points = hand.bid * rank;

        println!(
            "rank {} Hand: {:?} is {:?}, bid: {}, points: {}",
            rank, hand.hand, hand.kind, hand.bid, points
        );

        sum += points as u64;
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let sum = calculate_points("input.txt")?;
    println!("Total points: {}", sum);

    let sum2 = calculate_points("input_full.txt")?;
    println!("Total points: {}", sum2);
    Ok(())
}
