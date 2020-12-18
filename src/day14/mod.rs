/* Copyright (C) 2020 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use nom::lib::std::collections::BTreeMap;

pub struct Memory {
    memory_map: BTreeMap<usize, u64>,
}

impl Memory {
    pub fn parse(text: &str) -> Memory {
        let mut memory = Memory {
            memory_map: BTreeMap::new(),
        };
        let mut or_mask: u64 = 0;
        let mut and_mask: u64 = 0;

        for line in text.lines() {
            let split: Vec<&str> = line.split(" = ").collect();
            if split[0] == "mask" {
                and_mask = u64::from_str_radix(split[1].replace('X', "1").as_str(), 2).unwrap();
                or_mask = u64::from_str_radix(split[1].replace('X', "0").as_str(), 2).unwrap();
            } else {
                use regex::Regex;
                let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
                let captures = re.captures(line).unwrap();
                let address = captures[1].parse().unwrap();
                let value: u64 = captures[2].parse().unwrap();

                let value = value & and_mask | or_mask;
                memory.memory_map.insert(address, value);
            }
        }
        memory
    }

    fn get_mask_variations(mask: String) -> Vec<String> {
        if mask.find('F').is_some() {
            [
                Self::get_mask_variations(mask.replacen('F', "0", 1)),
                Self::get_mask_variations(mask.replacen('F', "1", 1)),
            ]
            .concat()
        } else {
            vec![mask]
        }
    }

    fn get_floating_address_variations(base_address: usize, mask: String) -> Vec<usize> {
        Self::get_mask_variations(mask)
            .iter()
            .map(|mask| {
                let and_mask = usize::from_str_radix(mask.replace('X', "1").as_str(), 2).unwrap();
                let or_mask = usize::from_str_radix(mask.replace('X', "0").as_str(), 2).unwrap();
                base_address & and_mask | or_mask
            })
            .collect()
    }

    fn get_address_variations(base_address: usize, mask: String) -> Vec<usize> {
        let or_mask = usize::from_str_radix(mask.replace('X', "0").as_str(), 2).unwrap();
        let address = base_address | or_mask;

        let remaining_mask = mask.replace('X', "F").replace('1', "X").replace('0', "X");
        Self::get_floating_address_variations(address, remaining_mask)
    }

    pub fn parse_part2(text: &str) -> Memory {
        let mut memory = Memory {
            memory_map: BTreeMap::new(),
        };
        let mut mask: String = String::new();

        for line in text.lines() {
            let split: Vec<&str> = line.split(" = ").collect();
            if split[0] == "mask" {
                mask = String::from(split[1]);
            } else {
                use regex::Regex;
                let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
                let captures = re.captures(line).unwrap();
                let base_address = captures[1].parse().unwrap();
                let value: u64 = captures[2].parse().unwrap();

                for address in Self::get_address_variations(base_address, mask.clone()) {
                    memory.memory_map.insert(address, value);
                }
            }
        }
        memory
    }

    pub fn get_sum(&self) -> u64 {
        self.memory_map.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let text = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let memory = Memory::parse(text);
        assert_eq!(memory.get_sum(), 165);
    }

    #[test]
    fn test_example_part2() {
        let text = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let memory = Memory::parse_part2(text);
        assert_eq!(memory.get_sum(), 208);
    }

    #[test]
    fn test_example_reddit1() {
        let text = "\
mask = 0XX000X1111001010X10XX1101XX00X00100
mem[50596] = 1000
mask = 0X000001111001010X1011100100001X0X0X
mem[45713] = 1";
        let memory = Memory::parse_part2(text);
        assert_eq!(memory.get_sum(), 508032);
    }

    #[test]
    fn test_example_reddit2() {
        let text = "\
mask = 000000000000000000000000000000000XXX
mem[8] = 4
mask = XX0000000000000000000000000000000000
mem[0] = 5";
        let memory = Memory::parse_part2(text);
        assert_eq!(memory.get_sum(), 52);
    }
}
