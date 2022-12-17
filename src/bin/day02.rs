//  Copyright 2022 Google LLC
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

#![warn(clippy::all)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use nom::{
    character::complete::{line_ending, one_of, space1},
    combinator::{map, opt},
    multi::fold_many1,
    sequence::separated_pair,
    IResult,
};
use panic_semihosting as _;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct GameRound {
    them: Hand,
    us: Hand,
}

fn score(r: &GameRound) -> u32 {
    match r.us {
        Hand::Rock => match r.them {
            Hand::Rock => 1 + 3,
            Hand::Paper => 1,
            Hand::Scissors => 1 + 6,
        },
        Hand::Paper => match r.them {
            Hand::Rock => 2 + 6,
            Hand::Paper => 2 + 3,
            Hand::Scissors => 2,
        },
        Hand::Scissors => match r.them {
            Hand::Rock => 3,
            Hand::Paper => 3 + 6,
            Hand::Scissors => 3 + 3,
        },
    }
}

fn part2_hand(them: Hand, us: char) -> Hand {
    match us {
        'X' => match them {
            // We want to lose.
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        },
        'Y' => them,
        'Z' => match them {
            // We want to win.
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        },
        _ => unreachable!(),
    }
}

fn parse_part1_round(i: &str) -> IResult<&str, GameRound> {
    let (i, round) = separated_pair(
        map(one_of("ABC"), |c| match c {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => unreachable!(),
        }),
        space1,
        map(one_of("XYZ"), |c| match c {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => unreachable!(),
        }),
    )(i)?;
    let (i, _) = opt(line_ending)(i)?;
    Ok((
        i,
        GameRound {
            them: round.0,
            us: round.1,
        },
    ))
}

fn parse_part2_round(i: &str) -> IResult<&str, GameRound> {
    let (i, round) = separated_pair(
        map(one_of("ABC"), |c| match c {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => unreachable!(),
        }),
        space1,
        one_of("XYZ"),
    )(i)?;
    let (i, _) = opt(line_ending)(i)?;
    Ok((
        i,
        GameRound {
            them: round.0,
            us: part2_hand(round.0, round.1),
        },
    ))
}

fn part1(i: &str) -> u32 {
    let (_, total) = fold_many1(
        parse_part1_round,
        || 0,
        |mut sum, r| {
            sum += score(&r);
            sum
        },
    )(i)
    .unwrap();
    total
}

fn part2(i: &str) -> u32 {
    let (_, total) = fold_many1(
        parse_part2_round,
        || 0,
        |mut sum, r| {
            sum += score(&r);
            sum
        },
    )(i)
    .unwrap();
    total
}

#[entry]
fn main() -> ! {
    let input = include_str!("../../input/02.txt");

    let p1 = part1(input);
    hprintln!("Part 1: {:?}", p1).unwrap();

    let p2 = part2(input);
    hprintln!("Part 2: {:?}", p2).unwrap();

    // Exit QEMU.
    debug::exit(debug::EXIT_SUCCESS);
    unreachable!()
}
