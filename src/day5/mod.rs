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

use std::string::String;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct SeatNumber {
    seat_number: String,
}

impl SeatNumber {
    pub fn from(text: &str) -> SeatNumber {
        SeatNumber {
            seat_number: String::from(text)
        }
    }

    fn accumulator(acc: (u32, u32), c: char) -> (u32, u32) {
        let average = (acc.0 + acc.1) / 2;
        if c == 'F' || c == 'L' {
            (acc.0, average)
        } else if c == 'B' || c == 'R' {
            (average + 1, acc.1)
        } else {
            panic!()
        }
    }

    pub fn get_row_column(&self) -> (u32, u32) {
        let row = self.seat_number.as_str()[..7].chars().fold((0, 127), Self::accumulator);
        assert_eq!(row.0, row.1);
        let column = self.seat_number.as_str()[7..].chars().fold((0, 7), Self::accumulator);
        assert_eq!(column.0, column.1);
        (row.0, column.0)
    }

    pub fn get_seat_id(&self) -> u32 {
        let (row, column) = self.get_row_column();
        row * 8 + column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_number() {
        let seat = SeatNumber::from("BFFFBBFRRR");
        assert_eq!(seat.get_row_column(), (70, 7));
        assert_eq!(seat.get_seat_id(), 567);

        let seat = SeatNumber::from("FFFBBBFRRR");
        assert_eq!(seat.get_row_column(), (14, 7));
        assert_eq!(seat.get_seat_id(), 119);

        let seat = SeatNumber::from("BBFFBBFRLL");
        assert_eq!(seat.get_row_column(), (102, 4));
        assert_eq!(seat.get_seat_id(), 820);
    }
}
