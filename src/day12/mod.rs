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

pub struct OwnShip {
    pos_east: i32,
    pos_north: i32,
    heading: i32,
    waypoint_east: i32,
    waypoint_north: i32,
}

impl Default for OwnShip {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnShip {
    pub fn new() -> OwnShip {
        OwnShip {
            pos_east: 0,
            pos_north: 0,
            heading: 90,
            waypoint_east: 10,
            waypoint_north: 1,
        }
    }

    pub fn execute(&mut self, instruction: &str) {
        let direction = &instruction[0..1];
        let amount: i32 = instruction[1..].parse().unwrap();

        match direction {
            "N" => self.pos_north += amount,
            "S" => self.pos_north -= amount,
            "E" => self.pos_east += amount,
            "W" => self.pos_east -= amount,
            "L" => {
                self.heading -= amount;
                if self.heading < 0 {
                    self.heading += 360
                }
            }
            "R" => {
                self.heading += amount;
                if self.heading >= 360 {
                    self.heading -= 360
                }
            }
            "F" => match self.heading {
                0 => self.pos_north += amount,
                90 => self.pos_east += amount,
                180 => self.pos_north -= amount,
                270 => self.pos_east -= amount,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn execute_part2(&mut self, instruction: &str) {
        let direction = &instruction[0..1];
        let amount: i32 = instruction[1..].parse().unwrap();

        match direction {
            "N" => self.waypoint_north += amount,
            "S" => self.waypoint_north -= amount,
            "E" => self.waypoint_east += amount,
            "W" => self.waypoint_east -= amount,
            "L" => {
                self.rotate_waypoint(360 - amount);
            }
            "R" => {
                self.rotate_waypoint(amount);
            }
            "F" => {
                self.pos_east += self.waypoint_east * amount;
                self.pos_north += self.waypoint_north * amount;
            }
            _ => panic!(),
        }
    }

    fn rotate_waypoint(&mut self, rotation: i32) {
        let old_east = self.waypoint_east;
        let old_north = self.waypoint_north;
        match rotation {
            90 => {
                self.waypoint_east = old_north;
                self.waypoint_north = -old_east
            }
            180 => {
                self.waypoint_east = -old_east;
                self.waypoint_north = -old_north;
            }
            270 => {
                self.waypoint_east = -old_north;
                self.waypoint_north = old_east
            }
            _ => panic!(),
        }
    }

    pub fn get_manhattan_distance(&self) -> i32 {
        self.pos_north.abs() + self.pos_east.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instructions() {
        let mut own_ship = OwnShip::new();

        own_ship.execute("F10");
        assert_eq!(own_ship.pos_east, 10);
        assert_eq!(own_ship.pos_north, 0);
        assert_eq!(own_ship.heading, 90);

        own_ship.execute("N3");
        assert_eq!(own_ship.pos_east, 10);
        assert_eq!(own_ship.pos_north, 3);
        assert_eq!(own_ship.heading, 90);

        own_ship.execute("F7");
        assert_eq!(own_ship.pos_east, 17);
        assert_eq!(own_ship.pos_north, 3);
        assert_eq!(own_ship.heading, 90);

        own_ship.execute("R90");
        assert_eq!(own_ship.pos_east, 17);
        assert_eq!(own_ship.pos_north, 3);
        assert_eq!(own_ship.heading, 180);

        own_ship.execute("F11");
        assert_eq!(own_ship.pos_east, 17);
        assert_eq!(own_ship.pos_north, -8);
        assert_eq!(own_ship.heading, 180);

        let distance = own_ship.get_manhattan_distance();
        assert_eq!(distance, 25);
    }

    #[test]
    fn test_instructions_part2() {
        let mut own_ship = OwnShip::new();

        own_ship.execute_part2("F10");
        assert_eq!(own_ship.pos_east, 100);
        assert_eq!(own_ship.pos_north, 10);
        assert_eq!(own_ship.waypoint_east, 10);
        assert_eq!(own_ship.waypoint_north, 1);

        own_ship.execute_part2("N3");
        assert_eq!(own_ship.pos_east, 100);
        assert_eq!(own_ship.pos_north, 10);
        assert_eq!(own_ship.waypoint_east, 10);
        assert_eq!(own_ship.waypoint_north, 4);

        own_ship.execute_part2("F7");
        assert_eq!(own_ship.pos_east, 170);
        assert_eq!(own_ship.pos_north, 38);
        assert_eq!(own_ship.waypoint_east, 10);
        assert_eq!(own_ship.waypoint_north, 4);

        own_ship.execute_part2("R90");
        assert_eq!(own_ship.pos_east, 170);
        assert_eq!(own_ship.pos_north, 38);
        assert_eq!(own_ship.waypoint_east, 4);
        assert_eq!(own_ship.waypoint_north, -10);

        own_ship.execute_part2("F11");
        assert_eq!(own_ship.pos_east, 214);
        assert_eq!(own_ship.pos_north, -72);
        assert_eq!(own_ship.waypoint_east, 4);
        assert_eq!(own_ship.waypoint_north, -10);

        let distance = own_ship.get_manhattan_distance();
        assert_eq!(distance, 286);
    }
}
