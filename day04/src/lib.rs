use std::collections::HashMap;
use std::iter::Iterator;

pub fn n_valid(passports: &Vec<HashMap<String, String>>) -> usize {
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    passports
        .iter()
        .filter(|passport| keys.iter().all(|key| passport.contains_key(*key)))
        .count()
}

pub fn to_passports<I>(passports: I) -> Vec<HashMap<String, String>>
where
    I: Iterator<Item = String>,
{
    let (passports, _) = passports.fold(
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

    passports.iter().map(to_passport).collect()
}

fn to_passport(passport: &Vec<String>) -> HashMap<String, String> {
    passport.iter().fold(HashMap::new(), |mut map, field| {
        let mut data = field.split(':');

        match (data.next(), data.next()) {
            (Some(key), Some(value)) => {
                map.insert(key.to_string(), value.to_string());
            }
            _ => {}
        };

        map
    })
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

        assert_eq!(super::n_valid(&passports), 2);
    }
}
