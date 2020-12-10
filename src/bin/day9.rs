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

use advent_of_code_2020::day9::*;

fn main() {
    println!("Part 1");

    let text = std::fs::read_to_string("data/input-day-9.txt").unwrap();
    let numbers = parse(text.as_str());

    let weakness = find_weakness(25, &numbers).unwrap();

    println!("Weakness: {}", weakness);

    println!("Part 2");

    let exploit = exploit_weakness(weakness, &numbers).unwrap();

    println!("Exploit: {}", exploit);
}
