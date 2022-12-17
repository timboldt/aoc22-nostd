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
    character::complete::{char, line_ending, u32},
    combinator::opt,
    multi::fold_many1,
    sequence::separated_pair,
    IResult,
};
use panic_semihosting as _;

struct Assignment {
    low: u32,
    high: u32,
}

impl Assignment {
    fn contained_within(&self, other: &Assignment) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn overlapping(&self, other: &Assignment) -> bool {
        (self.low >= other.low && self.low <= other.high)
            || (self.high >= other.low && self.high <= other.high)
            || (self.low < other.low && self.high > other.high)
    }
}

fn parse_assignment_pair(i: &str) -> IResult<&str, (Assignment, Assignment)> {
    let (i, ((first_low, first_high), (second_low, second_high))) = separated_pair(
        separated_pair(u32, char('-'), u32),
        char(','),
        separated_pair(u32, char('-'), u32),
    )(i)?;
    let (i, _) = opt(line_ending)(i)?;
    Ok((
        i,
        (
            Assignment {
                low: first_low,
                high: first_high,
            },
            Assignment {
                low: second_low,
                high: second_high,
            },
        ),
    ))
}

fn part1(i: &str) -> u32 {
    let (_, total) = fold_many1(
        parse_assignment_pair,
        || 0,
        |mut result, (first, second)| {
            if first.contained_within(&second) || second.contained_within(&first) {
                result += 1;
            }
            result
        },
    )(i)
    .unwrap();
    total
}

fn part2(i: &str) -> u32 {
    let (_, total) = fold_many1(
        parse_assignment_pair,
        || 0,
        |mut result, (first, second)| {
            if first.overlapping(&second) {
                result += 1;
            }
            result
        },
    )(i)
    .unwrap();
    total
}

#[entry]
fn main() -> ! {
    let input = include_str!("../../input/04.txt");

    let p1 = part1(input);
    hprintln!("Part 1: {:?}", p1).unwrap();

    let p2 = part2(input);
    hprintln!("Part 2: {:?}", p2).unwrap();

    // Exit QEMU.
    debug::exit(debug::EXIT_SUCCESS);
    unreachable!()
}
