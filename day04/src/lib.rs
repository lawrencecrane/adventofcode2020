use std::iter::Iterator;

pub fn to_passports<I>(passports: I) -> Vec<Vec<String>>
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

    passports
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_() {}
}
