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

use std::cmp::max;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Map {
    trees: Vec<Point>,
    size: Point,
}

impl Map {
    pub fn from_text<'a, T: Into<&'a str>>(input: T) -> Map {
        let input = input.into().lines();

        let mut map = Map {
            trees: vec!(),
            size: Point{ x: 0, y: 0 }
        };


        for (x, line) in input.enumerate() {
            map.size.x = max(x + 1, map.size.x);
            for (y, char) in line.chars().enumerate() {
                map.size.y = max(y + 1, map.size.y);
                if char == '#' {
                    map.trees.push(Point{x, y});
                }
            }
        }


        map
    }

    pub fn extend_point(&self, tree: &Point) -> Point {
        Point { x: tree.x, y: tree.y + self.size.y }
    }

    pub fn extend(mut self) -> Map {
        let mut new_trees = vec!();

        for tree in self.trees.iter() {
            new_trees.push(self.extend_point(tree));
            new_trees.push(*tree);
        }

        self.trees = new_trees;
        self.size = self.extend_point(&self.size);
        self
    }

    pub fn auto_extend(self, direction: Point) -> Map {
        let mut map = self;
        while map.size.y < map.size.x * direction.y {
            map = map.extend();
        }
        map
    }

    pub fn has_tree_at(&self, p: Point) -> bool {
        self.trees.iter().any(|tree| {*tree == p})
    }

    pub fn traverse(&self, direction: Point) -> usize {
        let mut count = 0;
        for x in (0..self.size.x).step_by(direction.x) {
            let y = x * direction.y / direction.x;
            assert!(y < self.size.y);
            if self.has_tree_at(Point{x, y}) {
                count = count + 1
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let text = "\
        ..#\
        #..\
        .#.";

        let map = Map::from_text(text);

        assert_eq!(map.trees, vec![Point{x: 0, y: 2},Point{x: 1, y: 0},Point{x: 2, y: 1}]);
        assert_eq!(map.size, Point{x:3, y:3});
    }

    #[test]
    fn extend() {
        let text = "\
        ..#\
        #..\
        .#.";

        let map = Map::from_text(text);
        let map = map.extend();

        let expected_text = "\
            ..#..#\
            #..#..\
            .#..#.";
        let expected_map = Map::from_text(expected_text);

        assert_eq!({let mut list = map.trees.clone(); list.sort(); list},
                   {let mut list = expected_map.trees.clone(); list.sort(); list});
        assert_eq!(map.size, expected_map.size);
    }

    #[test]
    fn traverse() {
        let text = include_str!("../../data/test-day-3.txt");
        let map = Map::from_text(text);
        let map = map.extend().extend().extend();

        assert_eq!(map.traverse(Point{x:1, y:1}), 2);
        assert_eq!(map.traverse(Point{x:1, y:3}), 7);
        assert_eq!(map.traverse(Point{x:1, y:5}), 3);
        assert_eq!(map.traverse(Point{x:1, y:7}), 4);
        assert_eq!(map.traverse(Point{x:2, y:1}), 2);
    }
}