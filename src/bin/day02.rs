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

use nom::character::streaming::alpha1;
use nom::character::streaming::char;
use nom::character::streaming::digit1;
use nom::IResult;

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

fn parser(i: &str) -> IResult<&str, Rule> {
    let (i, min) = digit1(i)?;
    let (i, _) = char('-')(i)?;
    let (i, max) = digit1(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, letter) = alpha1(i)?;
    let (i, _) = char(':')(i)?;
    let (i, _) = char(' ')(i)?;
    let password = i;

    let min = min.parse().unwrap();
    let max = max.parse().unwrap();
    let letter = letter.parse().unwrap();

    Ok((
        &"",
        Rule {
            min,
            max,
            letter,
            password,
        },
    ))
}

fn main() {
    println!("Part 1");
    let text = include_str!("../../data/input-day-02.txt");

    let lines: Vec<&str> = text.lines().collect();

    let mut match_count = 0;
    for line in &lines {
        let (_, rule) = parser(line).unwrap();
        // println!("{}-{} {}: {}", rule.min, rule.max, rule.letter, rule.password);

        let mut count = 0;
        for c in rule.password.chars() {
            if c == rule.letter {
                count += 1;
            }
        }
        if count >= rule.min && count <= rule.max {
            // println!("Match");
            match_count += 1;
        }
    }

    println!("Match count: {}", match_count);

    println!("Part 2");

    let mut match_count = 0;
    for line in &lines {
        let (_, rule) = parser(line).unwrap();
        // println!("{}-{} {}: {}", rule.min, rule.max, rule.letter, rule.password);

        let match1 = rule.password.chars().nth(rule.min - 1).unwrap() == rule.letter;
        let match2 = rule.password.chars().nth(rule.max - 1).unwrap() == rule.letter;
        if match1 ^ match2 {
            // println!("Match");
            match_count += 1;
        }
    }

    println!("Match count: {}", match_count);
}
