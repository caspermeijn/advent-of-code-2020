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

use advent_of_code_2020::day14::*;

fn main() {
    let text = include_str!("../../data/input-day-14.txt");

    println!("Part 1");

    let memory = Memory::parse(text);
    let sum = memory.get_sum();

    println!("Memory sum: {}", sum);

    println!("Part 2");

    let memory = Memory::parse_part2(text);
    let sum = memory.get_sum();

    println!("Memory sum: {}", sum);
}
