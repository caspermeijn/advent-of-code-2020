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

use itertools::Itertools;
use nom::lib::std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct SpokenNumber {
    index: usize,
    pub spoken_number: u32,
}

pub struct MemoryGame {
    last_number: SpokenNumber,
    memory_map: HashMap<u32, SpokenNumber>,
}

impl MemoryGame {
    pub fn new(starting_numbers: &[u32]) -> Self {
        let spoken_numbers: Vec<SpokenNumber> = starting_numbers
            .iter()
            .enumerate()
            .map(|(i, &spoken_number)| {
                let index = i + 1;
                SpokenNumber {
                    index,
                    spoken_number,
                }
            })
            .collect();

        let mut memory_map = HashMap::new();
        for &number in spoken_numbers.iter().dropping_back(1) {
            memory_map.insert(number.spoken_number, number);
        }

        let &last_number = spoken_numbers.last().unwrap();

        MemoryGame {
            memory_map,
            last_number,
        }
    }

    fn speak_next_number(&mut self) -> SpokenNumber {
        let previous_occurrence_last_number = self.memory_map.get(&self.last_number.spoken_number);

        let new_number = SpokenNumber {
            index: self.last_number.index + 1,
            spoken_number: {
                if let Some(old_number) = previous_occurrence_last_number {
                    (self.last_number.index - old_number.index) as u32
                } else {
                    0
                }
            },
        };

        self.memory_map
            .insert(self.last_number.spoken_number, self.last_number);
        self.last_number = new_number;

        new_number
    }

    pub fn speak_nth_number(&mut self, n: usize) -> SpokenNumber {
        loop {
            let next_number = self.speak_next_number();
            if next_number.index >= n {
                return next_number;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut game = MemoryGame::new(&[0, 3, 6]);

        let next_number = game.speak_next_number();
        assert_eq!(next_number.index, 4);
        assert_eq!(next_number.spoken_number, 0);

        let next_number = game.speak_next_number();
        assert_eq!(next_number.index, 5);
        assert_eq!(next_number.spoken_number, 3);

        let next_number = game.speak_next_number();
        assert_eq!(next_number.index, 6);
        assert_eq!(next_number.spoken_number, 3);

        let next_number = game.speak_next_number();
        assert_eq!(next_number.index, 7);
        assert_eq!(next_number.spoken_number, 1);

        let next_number = game.speak_next_number();
        assert_eq!(next_number.index, 8);
        assert_eq!(next_number.spoken_number, 0);

        let next_number = game.speak_next_number();
        assert_eq!(next_number.index, 9);
        assert_eq!(next_number.spoken_number, 4);

        let next_number = game.speak_next_number();
        assert_eq!(next_number.index, 10);
        assert_eq!(next_number.spoken_number, 0);

        let number_2020 = game.speak_nth_number(2020);
        assert_eq!(number_2020.index, 2020);
        assert_eq!(number_2020.spoken_number, 436);

        let number_30million = game.speak_nth_number(30000000);
        assert_eq!(number_30million.index, 30000000);
        assert_eq!(number_30million.spoken_number, 175594);
    }
}
