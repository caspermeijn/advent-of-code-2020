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

use advent_of_code_2020::day5::*;

fn main() {
    println!("Part 1");

    let text = std::fs::read_to_string("data/input-day-5.txt").unwrap();
    let seat_numbers: Vec<SeatNumber> = text.lines().map(|line| SeatNumber::from(line)).collect();

    let highest_seat_id = seat_numbers.iter().map(|sn| sn.get_seat_id()).max().unwrap();

    println!("Highest seat id: {}", highest_seat_id);

    println!("Part 2");

    for seat_left in seat_numbers.iter() {
        let your_seat = seat_numbers.iter().find(|seat| seat.get_seat_id() == seat_left.get_seat_id() + 1);
        if your_seat.is_none() {
            let seat_right = seat_numbers.iter().find(|seat| seat.get_seat_id() == seat_left.get_seat_id() + 2);
            if let Some(seat_right) = seat_right {
                println!("Seat {} and {} are taken, so your seat is: {}", seat_left.get_seat_id(), seat_right.get_seat_id(), seat_left.get_seat_id() + 1)
            }
        }
    }
}
