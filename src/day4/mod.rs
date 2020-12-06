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
pub struct Passport {
    pub birth_year: Option<String>,
    pub issue_year: Option<String>,
    pub expiration_year: Option<String>,
    pub height: Option<String>,
    pub hair_color: Option<String>,
    pub eye_color: Option<String>,
    pub passport_id: Option<String>,
    pub country_id: Option<String>,
}

impl Passport {
    pub fn parse_text(text: &str) -> Vec<Passport> {
        let mut passport_list = vec![];
        let mut passport = Passport::default();
        for line in text.lines() {
            if line.is_empty() {
                passport_list.push(passport.clone());
                passport = Passport::default();
            } else {
                for field in line.split(' ') {
                    let mut split_field = field.split(':');
                    let key = split_field.next().unwrap();
                    let value = split_field.next().unwrap();
                    match key {
                        "byr" => {
                            passport.birth_year = Some(String::from(value));
                        }
                        "iyr" => {
                            passport.issue_year = Some(String::from(value));
                        }
                        "eyr" => {
                            passport.expiration_year = Some(String::from(value));
                        }
                        "hgt" => {
                            passport.height = Some(String::from(value));
                        }
                        "hcl" => {
                            passport.hair_color = Some(String::from(value));
                        }
                        "ecl" => {
                            passport.eye_color = Some(String::from(value));
                        }
                        "pid" => {
                            passport.passport_id = Some(String::from(value));
                        }
                        "cid" => {
                            passport.country_id = Some(String::from(value));
                        }
                        _ => panic!("Invalid field key"),
                    }
                }
            }
        }
        passport_list.push(passport.clone());
        passport_list
    }

    pub fn fields_valid(&self) -> bool {
        self.passport_id.is_some()
            && self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
    }

    pub fn data_valid(&self) -> bool {
        self.fields_valid()
            && self.passport_id_valid()
            && self.birth_year_valid()
            && self.issue_year_valid()
            && self.expiration_year_valid()
            && self.height_valid()
            && self.hair_color_valid()
            && self.eye_color_valid()
    }

    pub fn birth_year_valid(&self) -> bool {
        let birth_year: u32 = self.birth_year.as_ref().unwrap().parse().unwrap();
        birth_year >= 1920 && birth_year <= 2002
    }

    pub fn issue_year_valid(&self) -> bool {
        let issue_year: u32 = self.issue_year.as_ref().unwrap().parse().unwrap();
        issue_year >= 2010 && issue_year <= 2020
    }
    pub fn expiration_year_valid(&self) -> bool {
        let expiration_year: u32 = self.expiration_year.as_ref().unwrap().parse().unwrap();
        expiration_year >= 2020 && expiration_year <= 2030
    }
    pub fn height_valid(&self) -> bool {
        let height = self.height.as_ref().unwrap();
        if height.ends_with("in") {
            let height: u32 = height[..2].parse().unwrap_or_default();
            height >= 59 && height <= 76
        } else if height.ends_with("cm") {
            let height: u32 = height[..3].parse().unwrap_or_default();
            height >= 150 && height <= 193
        } else {
            false
        }
    }
    pub fn hair_color_valid(&self) -> bool {
        let hair_color = self.hair_color.as_ref().unwrap();
        hair_color.starts_with("#") && i64::from_str_radix(&hair_color[1..], 16).is_ok()
    }
    pub fn eye_color_valid(&self) -> bool {
        match self.eye_color.as_ref().unwrap().as_str() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        }
    }
    pub fn passport_id_valid(&self) -> bool {
        let passport_id = self.passport_id.as_ref().unwrap();
        passport_id.chars().all(|c| c.is_numeric()) && passport_id.len() == 9
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let text = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm
";

        let passport_list = Passport::parse_text(text);
        let passport = passport_list.first().unwrap().clone();

        assert_eq!(passport.country_id.unwrap(), "147");
        assert_eq!(passport.passport_id.unwrap(), "860033327");
        assert_eq!(passport.eye_color.unwrap(), "gry");
        assert_eq!(passport.hair_color.unwrap(), "#fffffd");
        assert_eq!(passport.height.unwrap(), "183cm");
        assert_eq!(passport.expiration_year.unwrap(), "2020");
        assert_eq!(passport.issue_year.unwrap(), "2017");
        assert_eq!(passport.birth_year.unwrap(), "1937");
    }

    #[test]
    fn test_fields_valid() {
        let text = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let passport_list = Passport::parse_text(text);
        let mut passport_list = passport_list.iter();

        assert_eq!(passport_list.next().unwrap().fields_valid(), true);
        assert_eq!(passport_list.next().unwrap().fields_valid(), false);
        assert_eq!(passport_list.next().unwrap().fields_valid(), true);
        assert_eq!(passport_list.next().unwrap().fields_valid(), false);
    }

    #[test]
    fn test_birth_year_valid() {
        let mut passport = Passport::default();

        passport.birth_year = Some(String::from("2002"));
        assert_eq!(passport.birth_year_valid(), true);

        passport.birth_year = Some(String::from("2003"));
        assert_eq!(passport.birth_year_valid(), false);
    }

    #[test]
    fn test_height_valid() {
        let mut passport = Passport::default();

        passport.height = Some(String::from("60in"));
        assert_eq!(passport.height_valid(), true);

        passport.height = Some(String::from("190cm"));
        assert_eq!(passport.height_valid(), true);

        passport.height = Some(String::from("190in"));
        assert_eq!(passport.height_valid(), false);

        passport.height = Some(String::from("190"));
        assert_eq!(passport.height_valid(), false);
    }

    #[test]
    fn test_hair_color_valid() {
        let mut passport = Passport::default();

        passport.hair_color = Some(String::from("#123abc"));
        assert_eq!(passport.hair_color_valid(), true);

        passport.hair_color = Some(String::from("#123abz"));
        assert_eq!(passport.hair_color_valid(), false);

        passport.hair_color = Some(String::from("123abc"));
        assert_eq!(passport.hair_color_valid(), false);
    }

    #[test]
    fn test_eye_color_valid() {
        let mut passport = Passport::default();

        passport.eye_color = Some(String::from("brn"));
        assert_eq!(passport.eye_color_valid(), true);

        passport.eye_color = Some(String::from("wat"));
        assert_eq!(passport.eye_color_valid(), false);
    }

    #[test]
    fn test_passport_id_valid() {
        let mut passport = Passport::default();

        passport.passport_id = Some(String::from("000000001"));
        assert_eq!(passport.passport_id_valid(), true);

        passport.passport_id = Some(String::from("0123456789"));
        assert_eq!(passport.passport_id_valid(), false);
    }

    #[test]
    fn test_data_invalid() {
        let text = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let passport_list = Passport::parse_text(text);
        let mut passport_list = passport_list.iter();

        assert_eq!(passport_list.next().unwrap().data_valid(), false);
        assert_eq!(passport_list.next().unwrap().data_valid(), false);
        assert_eq!(passport_list.next().unwrap().data_valid(), false);
        assert_eq!(passport_list.next().unwrap().data_valid(), false);
    }

    #[test]
    fn test_data_valid() {
        let text = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passport_list = Passport::parse_text(text);
        let mut passport_list = passport_list.iter();

        assert_eq!(passport_list.next().unwrap().data_valid(), true);
        assert_eq!(passport_list.next().unwrap().data_valid(), true);
        assert_eq!(passport_list.next().unwrap().data_valid(), true);
        assert_eq!(passport_list.next().unwrap().data_valid(), true);
    }
}
