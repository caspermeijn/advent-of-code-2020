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

use advent_of_code_2020::day10::*;

fn main() {
    let text = include_str!("../../data/input-day-10.txt");

    println!("Part 1");

    let numbers = parse(text);
    let diff_map = find_diffs(&numbers);
    let multiplied = *diff_map.get(&1).unwrap() * *diff_map.get(&3).unwrap();
    assert_eq!(diff_map.get(&2), None);

    println!("Multiplied differences: {}", multiplied);

    println!("Part 2");

    let distinct_arrangements = find_distinct_arrangements(&numbers, None);

    println!("distinct_arrangements: {}", distinct_arrangements);
}
