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

use advent_of_code_2020::day3::*;

fn main() {
    println!("Part 1");

    let text = std::fs::read_to_string("data/input-day-3.txt").unwrap();
    let map = Map::from_text(text.as_str());
    let map = map.auto_extend(Point { x: 1, y: 3 });
    let count = map.traverse(Point { x: 1, y: 3 });

    println!("Trees encountered: {}", count);

    println!("Part 2");

    let text = std::fs::read_to_string("data/input-day-3.txt").unwrap();
    let map = Map::from_text(text.as_str());
    let map = map.auto_extend(Point { x: 1, y: 7 });
    let count1 = map.traverse(Point { x: 1, y: 1 });
    let count2 = map.traverse(Point { x: 1, y: 3 });
    let count3 = map.traverse(Point { x: 1, y: 5 });
    let count4 = map.traverse(Point { x: 1, y: 7 });
    let count5 = map.traverse(Point { x: 2, y: 1 });

    println!(
        "Multiplied trees encountered: {}",
        count1 * count2 * count3 * count4 * count5
    );
}
