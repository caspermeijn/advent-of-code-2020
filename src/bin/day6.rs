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

use advent_of_code_2020::day6::DeclarationForm;

fn main() {
    println!("Part 1");

    let text = std::fs::read_to_string("data/input-day-6.txt").unwrap();
    let form_list = DeclarationForm::parse_part1(text.as_str());

    let sum = form_list.iter().fold(0, |sum, form| {
        form.answers.len() + sum
    });

    println!("Sum of counts: {}", sum);

    println!("Part 2");

    let form_list = DeclarationForm::parse_part2(text.as_str());

    let sum: usize = form_list.iter().map(|form| form.answers.len()).sum();

    println!("Sum of counts: {}", sum);
}
