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

use std::cmp::Ordering;
use std::collections::VecDeque;

pub fn parse(text: &str) -> Vec<i64> {
    text.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn contains_match(preable: &VecDeque<i64>, needle: i64) -> bool {
    for x in preable {
        for y in preable {
            if x != y && x + y == needle {
                return true;
            }
        }
    }
    false
}

pub fn find_weakness(preamble_size: usize, input: &[i64]) -> Option<i64> {
    let mut preamble = VecDeque::new();
    for &number in input {
        if preamble.len() == preamble_size && !contains_match(&preamble, number) {
            return Some(number);
        }
        preamble.push_back(number);
        if preamble.len() > preamble_size {
            preamble.pop_front();
        }
    }
    None
}

pub fn exploit_weakness(weakness: i64, input: &[i64]) -> Option<i64> {
    let mut contiguous_set = VecDeque::new();
    for &number in input {
        contiguous_set.push_back(number);
        loop {
            let sum: i64 = contiguous_set.iter().sum();
            match sum.cmp(&weakness) {
                Ordering::Equal => {
                    return Some(
                        *contiguous_set.iter().min().unwrap()
                            + *contiguous_set.iter().max().unwrap(),
                    )
                }
                Ordering::Greater => {
                    contiguous_set.pop_front();
                }
                Ordering::Less => break,
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let text = "\
35
20
15";

        let numbers = parse(text);
        assert_eq!(numbers.len(), 3);
        assert_eq!(numbers[0], 35);
        assert_eq!(numbers[1], 20);
        assert_eq!(numbers[2], 15);
    }

    #[test]
    fn test_find_weakness() {
        let text = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let numbers = parse(text);

        let weakness = find_weakness(5, &numbers);

        assert_eq!(weakness, Some(127));
    }

    #[test]
    fn test_exploit_weakness() {
        let text = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let numbers = parse(text);

        let weakness = find_weakness(5, &numbers).unwrap();
        let exploit = exploit_weakness(weakness, &numbers);

        assert_eq!(exploit, Some(62));
    }
}
