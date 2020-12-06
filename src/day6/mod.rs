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
use nom::AsChar;
use std::string::String;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeclarationForm {
    pub answers: String,
}

impl DeclarationForm {
    pub fn parse_part1(text: &str) -> Vec<DeclarationForm> {
        text.split("\n\n")
            .map(|form_text| DeclarationForm {
                answers: form_text
                    .chars()
                    .filter(|c| c.is_alpha())
                    .unique()
                    .collect(),
            })
            .collect()
    }

    pub fn parse_part2(text: &str) -> Vec<DeclarationForm> {
        let possible_answers: String = ('a'..='z').collect();
        text.split("\n\n")
            .map(|form_text| {
                let answers = form_text
                    .lines()
                    .fold(possible_answers.clone(), |answers, line| {
                        answers.chars().filter(|c| line.contains(*c)).collect()
                    });
                DeclarationForm { answers }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Index;

    #[test]
    fn test_parse_part1() {
        let text = "\
abc

a
b
c

ab
ac

a
a
a
a

b";
        let forms = DeclarationForm::parse_part1(text);

        assert_eq!(forms.len(), 5);
        assert_eq!(forms.index(0).answers, "abc");
        assert_eq!(forms.index(1).answers, "abc");
        assert_eq!(forms.index(2).answers, "abc");
        assert_eq!(forms.index(3).answers, "a");
        assert_eq!(forms.index(4).answers, "b");
    }

    #[test]
    fn test_parse_part2() {
        let text = "\
abc

a
b
c

ab
ac

a
a
a
a

b";
        let forms = DeclarationForm::parse_part2(text);

        assert_eq!(forms.len(), 5);
        assert_eq!(forms.index(0).answers, "abc");
        assert_eq!(forms.index(1).answers, "");
        assert_eq!(forms.index(2).answers, "a");
        assert_eq!(forms.index(3).answers, "a");
        assert_eq!(forms.index(4).answers, "b");
    }
}
