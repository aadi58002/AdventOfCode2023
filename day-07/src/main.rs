use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // Part1
    // static ref MAPPING: HashMap<char, usize> = HashMap::from([
    //     ('A', 14),
    //     ('K', 13),
    //     ('Q', 12),
    //     ('J', 11),
    //     ('T', 10),
    //     ('9', 9),
    //     ('8', 8),
    //     ('7', 7),
    //     ('6', 6),
    //     ('5', 5),
    //     ('4', 4),
    //     ('3', 3),
    //     ('2', 2),
    // ]);

    // Part2
    static ref MAPPING: HashMap<char, usize> = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);
}

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
    commons: Vec<usize>,
    commons_sum: usize,
}

impl Hand {
    fn new(cards: &str, bid: usize) -> Self {
        let mut freq = HashMap::new();
        let mut value = Hand {
            cards: cards.chars().map(|val| MAPPING[&val]).collect(),
            bid,
            commons: vec![],
            commons_sum: 0,
        };
        cards
            .chars()
            .for_each(|val| *freq.entry(val).or_insert(0) += 1);
        // Part1
        // for ch in cards.chars() {
        //     let freq = freq[&ch];
        //     value.commons_sum += freq;
        //     value.commons.push(freq);
        // }

        // Part2
        let max_freq = freq
            .iter()
            .filter(|(k, _)| **k != 'J')
            .max_by_key(|(_, val)| *val)
            .map(|(key, val)| (*key, *val));
        if freq.get(&'J').is_some() && max_freq.is_some() {
            let (ch, fq) = max_freq.unwrap();
            let freq_j = *freq.get(&'J').unwrap();
            *freq.get_mut(&ch).unwrap() += freq_j;
            *freq.get_mut(&'J').unwrap() += fq;
        }
        for ch in cards.chars() {
            let freq = freq[&ch];
            value.commons_sum += freq;
            value.commons.push(freq);
        }
        value
    }
}

fn part(input: &str) -> usize {
    let mut hands = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let cards = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<usize>().unwrap();
        hands.push(Hand::new(cards, bid));
    }

    hands.sort_by(|a, b| {
        let cmp = a.commons_sum.cmp(&b.commons_sum);
        match cmp {
            std::cmp::Ordering::Equal => {
                for (a, b) in a.cards.iter().zip(b.cards.iter()) {
                    let cmp = a.cmp(b);
                    match cmp {
                        std::cmp::Ordering::Equal => {}
                        _ => return cmp,
                    }
                }
                unreachable!("All cards are the same hence ranks can't be assigned");
            }
            _ => return cmp,
        }
    });
    for hand in hands.iter() {
        for card in &hand.cards {
            print!("{card:02} ");
        }
        println!();
        for commons in &hand.commons {
            print!("{commons:02} ");
        }
        println!();
    }
    let mut sum = 0;
    for (index, hand) in hands.iter().enumerate() {
        sum += hand.bid * (index + 1)
    }
    sum
}

fn main() {
    let input = include_str!("./part1-input.txt");
    let output = part(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let output = part(input);
        assert_eq!(output, 6440);
    }

    #[test]
    fn part_2_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let output = part(input);
        assert_eq!(output, 5905);
    }
}
