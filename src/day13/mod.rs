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
use std::cmp::max;

pub struct Schedule {
    depart_time: u64,
    available_busses: BTreeMap<u64, u64>,
}

impl Schedule {
    pub fn parse(text: &str) -> Schedule {
        let mut lines = text.lines();
        let depart_time = lines.next().unwrap().parse().unwrap();
        let available_busses = lines
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(i, text)| {
                let optional_number = text.parse().ok();
                if let Some(number) = optional_number {
                    Some((i as u64, number))
                } else {
                    None
                }
            })
            .collect();
        Schedule {
            depart_time,
            available_busses,
        }
    }

    pub fn get_earliest_bus(&self) -> (u64, u64) {
        self.available_busses
            .iter()
            .map(|(_number, &available_bus)| {
                let time_till_next_bus = available_bus - (self.depart_time % available_bus);
                (time_till_next_bus, available_bus)
            })
            .min_by_key(|(time_till_next_bus, _available_bus)| *time_till_next_bus)
            .unwrap()
    }

    pub fn get_contest_answer(&self) -> u64 {
        let mut counter = 0;
        let mut step = 1;
        loop {
            step = max(
                step,
                self.available_busses.iter().fold(1, |acc, (number, bus)| {
                    let modulo = (counter + number) % bus;
                    if modulo == 0 {
                        acc * bus
                    } else {
                        acc
                    }
                }),
            );

            let is_correct_answer = self.available_busses.iter().all(|(number, bus)| {
                let modulo = (counter + number) % bus;
                modulo == 0
            });

            if is_correct_answer {
                return counter;
            } else {
                counter += step;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earliest_bus() {
        let text = "\
939
7,13,x,x,59,x,31,19";

        let schedule = Schedule::parse(text);

        assert_eq!(schedule.depart_time, 939);
        let available_busses: Vec<&u64> = schedule.available_busses.values().collect();
        assert_eq!(available_busses, [&7, &13, &59, &31, &19]);

        let (time_till_next_bus, available_bus) = schedule.get_earliest_bus();
        assert_eq!(time_till_next_bus, 5);
        assert_eq!(available_bus, 59);
    }

    #[test]
    fn test_contest_example2() {
        let text = "\
939
17,x,13,19";

        let schedule = Schedule::parse(text);
        assert_eq!(schedule.get_contest_answer(), 3417);
    }

    #[test]
    fn test_contest() {
        let text = "\
939
7,13,x,x,59,x,31,19";

        let schedule = Schedule::parse(text);
        assert_eq!(schedule.get_contest_answer(), 1068781);
    }
}
