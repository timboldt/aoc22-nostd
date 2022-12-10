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
use panic_halt as _;

fn parse(input: &[u8], output: &mut [u8]) {
    for (i, ch) in input.iter().enumerate() {
        output[i] = *ch;
    }
}

fn part1(puzzle: &[u8]) -> i32 {
    let mut marker: [u8; 4] = puzzle[0..4].try_into().unwrap();
    for (idx, ch) in puzzle.iter().enumerate() {
        marker[idx % 4] = *ch;
        let mut dup = false;
        for i in 0..4 {
            for j in i + 1..4 {
                if marker[i] == marker[j] {
                    dup = true;
                }
            }
        }
        if !dup {
            return idx as i32 + 1;
        }
    }
    0
}

fn part2(puzzle: &[u8]) -> i32 {
    let mut marker: [u8; 14] = puzzle[0..14].try_into().unwrap();
    for (idx, ch) in puzzle.iter().enumerate() {
        marker[idx % 14] = *ch;
        let mut dup = false;
        for i in 0..14 {
            for j in i + 1..14 {
                if marker[i] == marker[j] {
                    dup = true;
                }
            }
        }
        if !dup {
            return idx as i32 + 1;
        }
    }
    0
}

#[entry]
fn main() -> ! {
    let input = include_bytes!("../input/06.txt");
    // TODO: make static.
    let mut parsed = [b' '; 8192];
    parse(input, &mut parsed);

    let p1 = part1(&parsed);
    hprintln!("Part 1: {:?}", p1).unwrap();

    let p2 = part2(&parsed);
    hprintln!("Part 2: {:?}", p2).unwrap();

    // Exit QEMU.
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
