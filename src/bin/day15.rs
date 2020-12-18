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

use advent_of_code_2020::day15::*;

fn main() {
    let mut game = MemoryGame::new(&[11, 18, 0, 20, 1, 7, 16]);

    let number_2020 = game.speak_nth_number(2020);
    println!("Number at 2020: {}", number_2020.spoken_number);

    let number_30million = game.speak_nth_number(30000000);
    println!("Number at 30000000: {}", number_30million.spoken_number);
}
