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
use std::collections::HashMap;

pub fn parse(text: &str) -> Vec<i32> {
    let mut list: Vec<i32> = text.lines().map(|line| line.parse().unwrap()).collect();
    list.push(0);
    list.push(*list.iter().max().unwrap() + 3);
    list.sort_unstable();
    list
}

pub fn find_diffs(input: &[i32]) -> HashMap<i32, usize> {
    let mut diff_map = HashMap::new();
    for i in 1..input.len() {
        let prev = input[i - 1];
        let current = input[i];
        let diff: i32 = current - prev;
        diff_map.insert(diff, diff_map.get(&diff).unwrap_or(&0) + 1);
    }
    diff_map
}

pub fn find_distinct_arrangements_orig(input: &[i32], start_index: Option<usize>) -> usize {
    let start_index = start_index.unwrap_or(1);
    let mut counter = 1;
    for i in start_index..input.len() - 2 {
        let prev = input[i - 1];
        let next = input[i + 1];
        if next - prev <= 3 {
            let mut next_list = Vec::from(input);
            next_list.remove(i);
            counter += find_distinct_arrangements_orig(&next_list, Some(i));
        }
    }
    counter
}

pub fn find_distinct_arrangements(input: &[i32], _start_index: Option<usize>) -> usize {
    let mut paths_per_adapter: HashMap<i32, usize> = HashMap::new();
    paths_per_adapter.insert(0, 1);
    input.iter().dropping(1).for_each(|i| {
        paths_per_adapter.insert(
            *i,
            (1..=3)
                .map(|look_back| paths_per_adapter.get(&(i - look_back)).unwrap_or(&0))
                .sum(),
        );
    });
    *paths_per_adapter.get(input.last().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let text = "\
47
61
131
15";

        let numbers = parse(text);
        assert_eq!(numbers.len(), 6);
        assert_eq!(numbers[0], 0);
        assert_eq!(numbers[1], 15);
        assert_eq!(numbers[2], 47);
        assert_eq!(numbers[3], 61);
        assert_eq!(numbers[4], 131);
        assert_eq!(numbers[5], 134);
    }

    #[test]
    fn test_find_diffs() {
        let text = "\
16
10
15
5
1
11
7
19
6
12
4";

        let numbers = parse(text);
        let diff_map = find_diffs(&numbers);
        assert_eq!(diff_map.get(&1), Some(&7));
        assert_eq!(diff_map.get(&3), Some(&5));

        let distinct_arrangements = find_distinct_arrangements_orig(&numbers, None);
        assert_eq!(distinct_arrangements, 8);

        let distinct_arrangements = find_distinct_arrangements(&numbers, None);
        assert_eq!(distinct_arrangements, 8);
    }

    #[test]
    fn test_example1() {
        let text = "\
10
6
4
7
1
5";

        let numbers = parse(text);

        let distinct_arrangements = find_distinct_arrangements_orig(&numbers, None);
        assert_eq!(distinct_arrangements, 4);

        let distinct_arrangements = find_distinct_arrangements(&numbers, None);
        assert_eq!(distinct_arrangements, 4);
    }

    #[test]
    fn test_example2() {
        let text = "\
4
11
7
8
1
6
5";

        let numbers = parse(text);

        let distinct_arrangements = find_distinct_arrangements_orig(&numbers, None);
        assert_eq!(distinct_arrangements, 7);

        let distinct_arrangements = find_distinct_arrangements(&numbers, None);
        assert_eq!(distinct_arrangements, 7);
    }

    #[test]
    fn test_example3() {
        let text = "\
3
1
6
2";

        let numbers = parse(text);

        let distinct_arrangements = find_distinct_arrangements_orig(&numbers, None);
        assert_eq!(distinct_arrangements, 4);

        let distinct_arrangements = find_distinct_arrangements(&numbers, None);
        assert_eq!(distinct_arrangements, 4);
    }

    #[test]
    fn test_example4() {
        let text = "\
17
6
10
5
13
7
1
4
12
11
14";

        let numbers = parse(text);

        let distinct_arrangements = find_distinct_arrangements_orig(&numbers, None);
        assert_eq!(distinct_arrangements, 28);

        let distinct_arrangements = find_distinct_arrangements(&numbers, None);
        assert_eq!(distinct_arrangements, 28);
    }

    #[test]
    fn test_find_diffs_larger() {
        let text = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        let numbers = parse(text);
        let diff_map = find_diffs(&numbers);
        assert_eq!(diff_map.get(&1), Some(&22));
        assert_eq!(diff_map.get(&2), None);
        assert_eq!(diff_map.get(&3), Some(&10));

        let distinct_arrangements = find_distinct_arrangements_orig(&numbers, None);
        assert_eq!(distinct_arrangements, 19208);

        let distinct_arrangements = find_distinct_arrangements(&numbers, None);
        assert_eq!(distinct_arrangements, 19208);
    }
}
