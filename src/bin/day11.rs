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

use atoi::atoi;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

const NUM_MONKEYS: usize = 8;
const MAX_ITEMS: usize = 64;
const PARSE_SIZE: usize = 256;

#[derive(Debug, Clone, Copy)]
struct Monkey {
    num_inspections: usize,
    items: [u64; MAX_ITEMS],
    op: MonkeyOp,
    modulus: u64,
    if_true: usize,
    if_false: usize,
}
const DEFAULT_MONKEY: Monkey = Monkey {
    num_inspections: 0,
    items: [0; MAX_ITEMS],
    op: MonkeyOp::Square,
    modulus: 0,
    if_true: 0,
    if_false: 0,
};

#[derive(Debug, Clone, Copy)]
enum MonkeyOp {
    Plus(u64),
    Times(u64),
    Square,
}

fn parse_monkey(input: &[u8], monkey: &mut Monkey) {
    // Flatten out the line feeds.
    let mut flattened = [b' '; PARSE_SIZE];
    for (i, b) in input.iter().enumerate() {
        flattened[i] = match b {
            b'\n' => b' ',
            v => *v,
        };
    }

    #[allow(clippy::assign_op_pattern)]
    let re = safe_regex::regex!(br"Monkey[ ]*([0-9]+):[ ]*Starting items:[ ]*(.*)[ ]*Operation: new = old (.) ([old0-9]+)[ ]*Test: divisible by ([0-9]+)[ ]*If true: throw to monkey ([0-9]+)[ ]*If false: throw to monkey ([0-9]+).*");
    let (_, items, operator, operand, modulus, if_true, if_false) =
        re.match_slices(&flattened).unwrap();
    *monkey = Monkey {
        num_inspections: 0,
        items: [0; MAX_ITEMS],
        op: match (operator, operand) {
            (b"+", v) => MonkeyOp::Plus(atoi::<u64>(v).unwrap()),
            (b"*", b"old") => MonkeyOp::Square,
            (b"*", v) => MonkeyOp::Times(atoi::<u64>(v).unwrap()),
            _ => unreachable!(),
        },
        modulus: atoi::<u64>(modulus).unwrap(),
        if_true: atoi::<usize>(if_true).unwrap(),
        if_false: atoi::<usize>(if_false).unwrap(),
    };
    let mut i = 0;
    for b in items {
        match b {
            b'0'..=b'9' => {
                // HACK: This exploits the fact that all numbers are exactly two digits.
                // TODO: Do the parsing correctly using atoi.
                monkey.items[i / 2] += (b - b'0') as u64;
                if i % 2 == 0 {
                    monkey.items[i / 2] *= 10;
                }
                i += 1;
            }
            b' ' | b',' => {}
            x => hprintln!("Unexpected char: {}", x).unwrap(),
        }
    }
}

fn parse(input: &[u8], monkeys: &mut [Monkey; NUM_MONKEYS]) {
    let mut start = 0;
    let mut end = 1;

    for monkey in monkeys.iter_mut().take(NUM_MONKEYS) {
        // HACK: Manually split on double linefeed.
        for i in start + 1..start + PARSE_SIZE {
            end = i;
            if i == input.len() {
                break;
            }
            if input[i - 1] == b'\n' && input[i] == b'\n' {
                break;
            }
        }
        parse_monkey(&input[start..end], monkey);
        start = end + 1;
    }
}

fn part1(parsed: &[Monkey]) -> u64 {
    let mut monkeys: [Monkey; NUM_MONKEYS] = [DEFAULT_MONKEY; NUM_MONKEYS];
    monkeys[..NUM_MONKEYS].copy_from_slice(&parsed[..NUM_MONKEYS]);

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            for i in 0..monkeys[m].items.len() {
                let item = monkeys[m].items.get_mut(i).unwrap();
                if *item != 0 {
                    let w = *item;
                    *item = 0;
                    let worry = match monkeys[m].op {
                        MonkeyOp::Plus(x) => w + x,
                        MonkeyOp::Times(x) => w * x,
                        MonkeyOp::Square => w * w,
                    } / 3;
                    let target = if worry % monkeys[m].modulus == 0 {
                        monkeys[m].if_true
                    } else {
                        monkeys[m].if_false
                    };
                    let mut ok = false;
                    for t in 0..monkeys[target].items.len() {
                        if monkeys[target].items[t] == 0 {
                            monkeys[target].items[t] = worry;
                            ok = true;
                            break;
                        }
                    }
                    if !ok {
                        hprintln!("Monkey items overflow!").unwrap();
                    }
                    monkeys[m].num_inspections += 1;
                }
            }
        }
    }

    // Find the top two and multiply them together.
    let mut top_two: [u64; 2] = [0; 2];
    for m in monkeys {
        if m.num_inspections as u64 > top_two[0] {
            if top_two[0] > top_two[1] {
                top_two[1] = top_two[0];
            }
            top_two[0] = m.num_inspections as u64;
            continue;
        }
        if m.num_inspections as u64 > top_two[1] {
            top_two[1] = m.num_inspections as u64;
        }
    }
    top_two.iter().product()
}

fn part2(parsed: &[Monkey]) -> u64 {
    let mut monkeys: [Monkey; NUM_MONKEYS] = [DEFAULT_MONKEY; NUM_MONKEYS];
    monkeys[..NUM_MONKEYS].copy_from_slice(&parsed[..NUM_MONKEYS]);
    let mod_product: u64 = monkeys.iter().map(|m| m.modulus).product();

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            for i in 0..monkeys[m].items.len() {
                let item = monkeys[m].items.get_mut(i).unwrap();
                if *item != 0 {
                    let w = *item;
                    *item = 0;
                    let worry = match monkeys[m].op {
                        MonkeyOp::Plus(x) => w + x,
                        MonkeyOp::Times(x) => w * x,
                        MonkeyOp::Square => w * w,
                    };
                    let target = if worry % monkeys[m].modulus == 0 {
                        monkeys[m].if_true
                    } else {
                        monkeys[m].if_false
                    };
                    let mut ok = false;
                    for t in 0..monkeys[target].items.len() {
                        if monkeys[target].items[t] == 0 {
                            monkeys[target].items[t] = worry % mod_product;
                            ok = true;
                            break;
                        }
                    }
                    if !ok {
                        hprintln!("Monkey items overflow!").unwrap();
                    }
                    monkeys[m].num_inspections += 1;
                }
            }
        }
    }

    // Find the top two and multiply them together.
    let mut top_two: [u64; 2] = [0; 2];
    for m in monkeys {
        if m.num_inspections as u64 > top_two[0] {
            if top_two[0] > top_two[1] {
                top_two[1] = top_two[0];
            }
            top_two[0] = m.num_inspections as u64;
            continue;
        }
        if m.num_inspections as u64 > top_two[1] {
            top_two[1] = m.num_inspections as u64;
        }
    }
    top_two.iter().product()
}

#[entry]
fn main() -> ! {
    let input = include_bytes!("../../input/11.txt");

    let monkeys: &mut [Monkey; NUM_MONKEYS] = &mut [DEFAULT_MONKEY; NUM_MONKEYS];
    parse(input, monkeys);
    let p1 = part1(monkeys);
    hprintln!("Part 1: {:?}", p1).unwrap();

    let p2 = part2(monkeys);
    hprintln!("Part 2: {:?}", p2).unwrap();

    // Exit QEMU.
    debug::exit(debug::EXIT_SUCCESS);
    unreachable!()
}
