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

fn main() {
    println!("Part 1");
    let text = include_str!("../../data/input-day-01.txt");

    let numbers: Vec<i32> = text.lines().map(|line| line.parse().unwrap()).collect();

    for num1 in numbers.clone() {
        for num2 in numbers.clone() {
            if num1 + num2 == 2020 {
                println!("{} + {} = 2020", num1, num2);
                println!("{} * {} = {}", num1, num2, num1 * num2);
            }
        }
    }

    println!("Part 2");
    for num1 in numbers.clone() {
        for num2 in numbers.clone() {
            for num3 in numbers.clone() {
                if num1 + num2 + num3 == 2020 {
                    println!("{} + {} + {} = 2020", num1, num2, num3);
                    println!("{} * {} * {} = {}", num1, num2, num3, num1 * num2 * num3);
                }
            }
        }
    }
}
