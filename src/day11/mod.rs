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

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Seat {
    EmptyFloor,
    Empty,
    Occupied,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct FloorPlan {
    seats: Vec<Vec<Seat>>,
}

impl FloorPlan {
    pub fn parse(text: &str) -> FloorPlan {
        FloorPlan {
            seats: text
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|char| match char {
                            '.' => Seat::EmptyFloor,
                            'L' => Seat::Empty,
                            '#' => Seat::Occupied,
                            _ => panic!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn get_seat(&self, x: usize, y: usize) -> Option<&Seat> {
        self.seats.get(x).and_then(|row| row.get(y))
    }

    fn get_adjacent_seat(&self, x: usize, y: usize) -> Vec<&Seat> {
        let mut coordinates = vec![];
        if x > 0 {
            if y > 0 {
                coordinates.push((x - 1, y - 1));
            }
            coordinates.push((x - 1, y));
            if y < self.seats[x - 1].len() {
                coordinates.push((x - 1, y + 1));
            }
        }
        if y > 0 {
            coordinates.push((x, y - 1));
        }
        if y < self.seats[x].len() {
            coordinates.push((x, y + 1));
        }
        if x < self.seats.len() - 1 {
            if y > 0 {
                coordinates.push((x + 1, y - 1));
            }
            coordinates.push((x + 1, y));
            if y < self.seats[x + 1].len() {
                coordinates.push((x + 1, y + 1));
            }
        }
        coordinates
            .iter()
            .filter_map(|(x, y)| self.get_seat(*x, *y))
            .collect()
    }

    fn add_direction(pos: (usize, usize), direction: (isize, isize)) -> (usize, usize) {
        if (direction.0 >= 0 || pos.0 > 0) && (direction.1 >= 0 || pos.1 > 0) {
            (
                (pos.0 as isize + direction.0) as usize,
                (pos.1 as isize + direction.1) as usize,
            )
        } else {
            (999999, 999999)
        }
    }

    fn get_seen_seat(&self, seat: (usize, usize), direction: (isize, isize)) -> Option<&Seat> {
        let mut seen_seat = Self::add_direction(seat, direction);
        loop {
            let option_seat = self.get_seat(seen_seat.0, seen_seat.1);
            if let Some(seat) = option_seat {
                if seat != &Seat::EmptyFloor {
                    return option_seat;
                }
            } else {
                return option_seat;
            }
            seen_seat = Self::add_direction(seen_seat, direction);
        }
    }

    fn get_seen_seats(&self, x: usize, y: usize) -> Vec<&Seat> {
        let directions = vec![
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (1, 0),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];
        directions
            .iter()
            .filter_map(|direction| self.get_seen_seat((x, y), *direction))
            .collect()
    }

    pub fn get_number_of_seen_occupied_seat(&self, x: usize, y: usize) -> usize {
        self.get_seen_seats(x, y)
            .iter()
            .filter(|&&&seat| seat == Seat::Occupied)
            .count()
    }

    pub fn get_number_of_adjacent_occupied_seat(&self, x: usize, y: usize) -> usize {
        self.get_adjacent_seat(x, y)
            .iter()
            .filter(|&&&seat| seat == Seat::Occupied)
            .count()
    }

    pub fn get_evolution(&self) -> Self {
        FloorPlan {
            seats: self
                .seats
                .iter()
                .enumerate()
                .map(|(x, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(y, seat)| match seat {
                            Seat::EmptyFloor => Seat::EmptyFloor,
                            Seat::Occupied => {
                                if self.get_number_of_adjacent_occupied_seat(x, y) >= 4 {
                                    Seat::Empty
                                } else {
                                    Seat::Occupied
                                }
                            }
                            Seat::Empty => {
                                if self.get_number_of_adjacent_occupied_seat(x, y) == 0 {
                                    Seat::Occupied
                                } else {
                                    Seat::Empty
                                }
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn get_evolution_part2(&self) -> Self {
        FloorPlan {
            seats: self
                .seats
                .iter()
                .enumerate()
                .map(|(x, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(y, seat)| match seat {
                            Seat::EmptyFloor => Seat::EmptyFloor,
                            Seat::Occupied => {
                                if self.get_number_of_seen_occupied_seat(x, y) >= 5 {
                                    Seat::Empty
                                } else {
                                    Seat::Occupied
                                }
                            }
                            Seat::Empty => {
                                if self.get_number_of_seen_occupied_seat(x, y) == 0 {
                                    Seat::Occupied
                                } else {
                                    Seat::Empty
                                }
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn get_stable_evolution(&self) -> Self {
        let mut prev = self.clone();
        loop {
            let next = prev.get_evolution();
            if prev == next {
                return next;
            }
            prev = next
        }
    }

    pub fn get_stable_evolution_part2(&self) -> Self {
        let mut prev = self.clone();
        loop {
            let next = prev.get_evolution_part2();
            if prev == next {
                return next;
            }
            prev = next
        }
    }

    pub fn get_number_of_occupied_seat(&self) -> usize {
        self.seats
            .iter()
            .map(|row| row.iter().filter(|&&seat| seat == Seat::Occupied).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let text = "\
L.#
LLL";

        let floor_plan = FloorPlan::parse(text);
        assert_eq!(floor_plan.seats.len(), 2);
        assert_eq!(floor_plan.seats[0].len(), 3);
        assert_eq!(floor_plan.seats[0][0], Seat::Empty);
        assert_eq!(floor_plan.seats[0][1], Seat::EmptyFloor);
        assert_eq!(floor_plan.seats[0][2], Seat::Occupied);
        assert_eq!(floor_plan.seats[1].len(), 3);
        assert_eq!(floor_plan.seats[1][0], Seat::Empty);
        assert_eq!(floor_plan.seats[1][1], Seat::Empty);
        assert_eq!(floor_plan.seats[1][2], Seat::Empty);
    }

    #[test]
    fn test_adjasent_seats() {
        let text = "\
###
.L#
LLL";

        let floor_plan = FloorPlan::parse(text);
        let adjacent_seats = floor_plan.get_adjacent_seat(1, 1);
        assert_eq!(adjacent_seats.len(), 8);
        assert_eq!(adjacent_seats[0], &Seat::Occupied);
        assert_eq!(adjacent_seats[1], &Seat::Occupied);
        assert_eq!(adjacent_seats[2], &Seat::Occupied);
        assert_eq!(adjacent_seats[3], &Seat::EmptyFloor);
        assert_eq!(adjacent_seats[4], &Seat::Occupied);
        assert_eq!(adjacent_seats[5], &Seat::Empty);
        assert_eq!(adjacent_seats[6], &Seat::Empty);
        assert_eq!(adjacent_seats[7], &Seat::Empty);
    }

    #[test]
    fn test_evolution() {
        let text = "\
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";

        let floor_plan = FloorPlan::parse(text);
        let evolution = floor_plan.get_evolution();

        let expected_text = "\
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";
        let expected_floor_plan = FloorPlan::parse(expected_text);
        assert_eq!(evolution, expected_floor_plan);
    }

    #[test]
    fn test_stable_evolution() {
        let text = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let floor_plan = FloorPlan::parse(text);
        let evolution = floor_plan.get_stable_evolution();

        let expected_text = "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";
        let expected_floor_plan = FloorPlan::parse(expected_text);
        assert_eq!(evolution, expected_floor_plan);

        let occupied_seats = evolution.get_number_of_occupied_seat();
        assert_eq!(occupied_seats, 37);
    }

    #[test]
    fn test_seen_seats() {
        let text = "\
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";

        let floor_plan = FloorPlan::parse(text);
        let adjacent_seats = floor_plan.get_seen_seats(4, 3);
        assert_eq!(adjacent_seats.len(), 8);
        assert_eq!(adjacent_seats[0], &Seat::Occupied);
        assert_eq!(adjacent_seats[1], &Seat::Occupied);
        assert_eq!(adjacent_seats[2], &Seat::Occupied);
        assert_eq!(adjacent_seats[3], &Seat::Occupied);
        assert_eq!(adjacent_seats[4], &Seat::Occupied);
        assert_eq!(adjacent_seats[5], &Seat::Occupied);
        assert_eq!(adjacent_seats[6], &Seat::Occupied);
        assert_eq!(adjacent_seats[7], &Seat::Occupied);
    }

    #[test]
    fn test_seen_seats2() {
        let text = "\
.............
.L.L.#.#.#.#.
.............";

        let floor_plan = FloorPlan::parse(text);
        let adjacent_seats = floor_plan.get_seen_seats(1, 1);
        assert_eq!(adjacent_seats.len(), 1);
        assert_eq!(adjacent_seats[0], &Seat::Empty);
    }

    #[test]
    fn test_stable_evolution_part2() {
        let text = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let floor_plan = FloorPlan::parse(text);
        let evolution = floor_plan.get_stable_evolution_part2();

        let expected_text = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";
        let expected_floor_plan = FloorPlan::parse(expected_text);
        assert_eq!(evolution, expected_floor_plan);

        let occupied_seats = evolution.get_number_of_occupied_seat();
        assert_eq!(occupied_seats, 26);
    }
}
