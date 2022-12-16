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
use heapless::binary_heap::{BinaryHeap, Max};
use nom::{
    character::complete::{line_ending, u32},
    combinator::opt,
    multi::fold_many1,
    sequence::terminated,
    IResult,
};
use panic_halt as _;

type Elf = u32;

fn parse_elf(i: &str) -> IResult<&str, Elf> {
    let (i, _) = opt(line_ending)(i)?;
    let (i, elf) = fold_many1(
        terminated(u32, line_ending),
        || 0,
        |mut sum, val| {
            sum += val;
            sum
        },
    )(i)?;
    Ok((i, elf))
}

fn part1(i: &str) -> Elf {
    let (_, most) = fold_many1(
        parse_elf,
        || 0,
        |most, val| {
            if most > val {
                most
            } else {
                val
            }
        },
    )(i)
    .unwrap();
    most
}

fn part2(i: &str) -> Elf {
    let mut heap: BinaryHeap<Elf, Max, 250> = BinaryHeap::new();
    let (_, _) = fold_many1(
        parse_elf,
        || 0,
        |_, val| {
            heap.push(val).unwrap();
            val
        },
    )(i)
    .unwrap();
    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}

#[entry]
fn main() -> ! {
    let input = include_str!("../../input/01.txt");

    let p1 = part1(input);
    hprintln!("Part 1: {:?}", p1).unwrap();

    let p2 = part2(input);
    hprintln!("Part 2: {:?}", p2).unwrap();

    // Exit QEMU.
    debug::exit(debug::EXIT_SUCCESS);
    unreachable!()
}
