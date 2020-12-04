use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::iter::Iterator;
use std::str::FromStr;

lazy_static! {
    static ref HEX_REGEX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
}

pub fn n_valid_keys_and_values(passports: &Vec<HashMap<PasswordField, String>>) -> usize {
    passports
        .iter()
        .filter(|passport| {
            NON_OPTIONAL_KEYS.iter().all(|key| {
                passport
                    .get(key)
                    .map_or(false, |value| has_valid_value(key, value))
            })
        })
        .count()
}

pub fn n_valid_keys(passports: &Vec<HashMap<PasswordField, String>>) -> usize {
    passports
        .iter()
        .filter(|passport| {
            NON_OPTIONAL_KEYS
                .iter()
                .all(|key| passport.contains_key(key))
        })
        .count()
}

fn has_valid_value(field: &PasswordField, value: &String) -> bool {
    match field {
        PasswordField::BYR => parse_number_and(value, |x| x >= 1920 && x <= 2002),
        PasswordField::IYR => parse_number_and(value, |x| x >= 2010 && x <= 2020),
        PasswordField::EYR => parse_number_and(value, |x| x >= 2020 && x <= 2030),
        PasswordField::HGT => is_valid_height(value),
        PasswordField::HCL => HEX_REGEX.is_match(value),
        PasswordField::ECL => EYE_COLORS.contains(&value.as_str()),
        PasswordField::PID => value.len() == 9 && value.chars().all(|c| c.is_numeric()),
        PasswordField::CID => true,
    }
}

fn is_valid_height(height: &String) -> bool {
    match (height.split("cm").next(), height.split("in").next()) {
        (Some(cm), _) => parse_number_and(&cm.to_string(), |x| x >= 150 && x <= 193),
        (_, Some(inch)) => parse_number_and(&inch.to_string(), |x| x >= 59 && x <= 76),
        _ => false,
    }
}

fn parse_number_and<F: FnOnce(u32) -> bool>(value: &String, f: F) -> bool {
    value.parse::<u32>().map_or(false, f)
}

pub fn to_passports<I>(passports: I) -> Vec<HashMap<PasswordField, String>>
where
    I: Iterator<Item = String>,
{
    let (mut passports, last) = passports.fold(
        (Vec::new(), Vec::new()),
        |(mut passports, mut current_passport), line| {
            let mut data: Vec<String> = line
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|s| s.to_string())
                .collect();

            match data.len() {
                0 => {
                    passports.push(current_passport);

                    (passports, Vec::new())
                }
                _ => {
                    current_passport.append(&mut data);

                    (passports, current_passport)
                }
            }
        },
    );

    passports.push(last);

    passports.iter().map(to_passport).collect()
}

fn to_passport(passport: &Vec<String>) -> HashMap<PasswordField, String> {
    passport.iter().fold(HashMap::new(), |mut map, field| {
        let mut data = field.split(':');

        match (data.next(), data.next()) {
            (Some(key), Some(value)) => {
                map.insert(PasswordField::from_str(key).unwrap(), value.to_string());
            }
            _ => {}
        };

        map
    })
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

const NON_OPTIONAL_KEYS: [PasswordField; 7] = [
    PasswordField::BYR,
    PasswordField::IYR,
    PasswordField::EYR,
    PasswordField::HGT,
    PasswordField::HCL,
    PasswordField::ECL,
    PasswordField::PID,
];

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PasswordField {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}

impl FromStr for PasswordField {
    type Err = ();

    fn from_str(input: &str) -> Result<PasswordField, Self::Err> {
        match input {
            "byr" => Ok(PasswordField::BYR),
            "iyr" => Ok(PasswordField::IYR),
            "eyr" => Ok(PasswordField::EYR),
            "hgt" => Ok(PasswordField::HGT),
            "hcl" => Ok(PasswordField::HCL),
            "ecl" => Ok(PasswordField::ECL),
            "pid" => Ok(PasswordField::PID),
            "cid" => Ok(PasswordField::CID),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    fn create_factory() -> Vec<String> {
        vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
            "",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_n_valid() {
        let passports = super::to_passports(create_factory().into_iter());

        assert_eq!(super::n_valid_keys(&passports), 2);
    }
}
