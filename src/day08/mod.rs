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

use std::collections::BTreeMap;
use std::ops::Index;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Instruction {
    Accumulate(i32),
    Jump(i32),
    NoOperation(i32),
}

impl Instruction {
    pub fn parse(text: &str) -> Vec<Instruction> {
        text.lines()
            .map(|line| {
                let mut split = line.split(' ');
                let op = split.next().unwrap();
                let arg: i32 = split.next().unwrap().parse().unwrap();
                match op {
                    "acc" => Instruction::Accumulate(arg),
                    "jmp" => Instruction::Jump(arg),
                    "nop" => Instruction::NoOperation(arg),
                    _ => panic!(),
                }
            })
            .collect()
    }
}

pub fn execute_once(program: &[Instruction]) -> (i32, bool) {
    let mut program_counter = 0;
    let mut accumulator = 0;
    let mut instruction_executed = BTreeMap::new();
    loop {
        if *instruction_executed.get(&program_counter).unwrap_or(&false) {
            return (accumulator, false);
        } else {
            instruction_executed.insert(program_counter, true);
        }
        let instruction = program.index(program_counter);
        match instruction {
            Instruction::Accumulate(arg) => {
                accumulator += arg;
                program_counter += 1;
            }
            Instruction::Jump(arg) => {
                program_counter = ((program_counter as isize) + (*arg as isize)) as usize
            }
            Instruction::NoOperation(_) => {
                program_counter += 1;
            }
        }
        if program_counter >= program.len() {
            return (accumulator, true);
        }
    }
}

pub struct MutatedPrograms {
    count: usize,
    program: Vec<Instruction>,
}

impl MutatedPrograms {
    pub fn new(program: Vec<Instruction>) -> MutatedPrograms {
        MutatedPrograms { count: 0, program }
    }
}

impl Iterator for MutatedPrograms {
    type Item = Vec<Instruction>;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.program.len() {
            return None;
        }

        while let Instruction::Accumulate(_) = self.program[self.count] {
            self.count += 1;
            if self.count >= self.program.len() {
                return None;
            }
        }

        let mut mutated_program = self.program.clone();
        match mutated_program[self.count] {
            Instruction::NoOperation(arg) => mutated_program[self.count] = Instruction::Jump(arg),
            Instruction::Jump(arg) => mutated_program[self.count] = Instruction::NoOperation(arg),
            Instruction::Accumulate(_) => panic!(),
        }

        self.count += 1;

        Some(mutated_program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Index;

    #[test]
    fn test_parse() {
        let text = "\
nop +0
acc +1
jmp +4";

        let instruction_list = Instruction::parse(text);

        assert_eq!(instruction_list.len(), 3);
        assert_eq!(*instruction_list.index(0), Instruction::NoOperation(0));
        assert_eq!(*instruction_list.index(1), Instruction::Accumulate(1));
        assert_eq!(*instruction_list.index(2), Instruction::Jump(4));
    }

    #[test]
    fn test_execute_once() {
        let text = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let program = Instruction::parse(text);

        let (acc, completed) = execute_once(&program);
        assert_eq!(acc, 5);
        assert_eq!(completed, false);
    }

    #[test]
    fn test_mutated_program() {
        let text = "\
nop +0
acc +1
jmp +4";

        let program = Instruction::parse(text);

        let mut mutated_programs = MutatedPrograms::new(program);

        let mutated_program = mutated_programs.next().unwrap();

        assert_eq!(mutated_program.len(), 3);
        assert_eq!(*mutated_program.index(0), Instruction::Jump(0));
        assert_eq!(*mutated_program.index(1), Instruction::Accumulate(1));
        assert_eq!(*mutated_program.index(2), Instruction::Jump(4));

        let mutated_program = mutated_programs.next().unwrap();

        assert_eq!(mutated_program.len(), 3);
        assert_eq!(*mutated_program.index(0), Instruction::NoOperation(0));
        assert_eq!(*mutated_program.index(1), Instruction::Accumulate(1));
        assert_eq!(*mutated_program.index(2), Instruction::NoOperation(4));
    }
}
