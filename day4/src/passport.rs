use std::i64;

lazy_static! {
    static ref VALID_EYE_COLORS: Vec<String> = {
        let mut vec: Vec<String> = Vec::new();
        vec.push("amb".into());
        vec.push("blu".into());
        vec.push("brn".into());
        vec.push("gry".into());
        vec.push("grn".into());
        vec.push("hzl".into());
        vec.push("oth".into());
        vec
    };
}

pub struct Passport {
    pub byr: String,
    pub iyr: String,
    pub eyr: String,
    pub hgt: String,
    pub hcl: String,
    pub ecl: String,
    pub pid: String,
    pub cid: String,
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            byr: "".into(),
            iyr: "".into(),
            eyr: "".into(),
            hgt: "".into(),
            hcl: "".into(),
            ecl: "".into(),
            pid: "".into(),
            cid: "".into(),
        }
    }

    /// Checks that all required fields have a non-empty value.
    pub fn has_required_fields(&self) -> bool {
        self.byr.len() > 0
            && self.iyr.len() > 0
            && self.eyr.len() > 0
            && self.hgt.len() > 0
            && self.hcl.len() > 0
            && self.ecl.len() > 0
            && self.pid.len() > 0
    }

    /// Checks that all required fields have valid values.
    pub fn has_valid_values(&self) -> bool {
        if !self.has_required_fields() {
            return false;
        }
        // Birth year
        match self.byr.parse::<u16>() {
            Ok(n) => {
                if n < 1920 || n > 2002 {
                    return false;
                }
            }
            Err(_) => return false,
        }

        // Issue year
        match self.iyr.parse::<u16>() {
            Ok(n) => {
                if n < 2010 || n > 2020 {
                    return false;
                }
            }
            Err(_) => return false,
        }

        // Expiration year
        match self.eyr.parse::<u16>() {
            Ok(n) => {
                if n < 2020 || n > 2030 {
                    return false;
                }
            }
            Err(_) => return false,
        }

        // Height
        if self.hgt.len() < 3 {
            return false;
        }
        let hgt_len = self.hgt.len();
        let hgt_val = match self.hgt[0..hgt_len - 2].parse::<u16>() {
            Ok(n) => n,
            Err(_) => {
                println!("parse fail"); // DEBUG
                return false;
            }
        };
        let hgt_unit = &self.hgt[hgt_len - 2..hgt_len];
        match hgt_unit {
            "in" => {
                if hgt_val < 59 || hgt_val > 76 {
                    return false;
                }
            }
            "cm" => {
                if hgt_val < 150 || hgt_val > 193 {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }

        // Hair color
        if self.hcl.len() != 7 {
            return false;
        }
        if self.hcl.chars().collect::<Vec<char>>()[0] != '#' {
            return false;
        }
        if i64::from_str_radix(&self.hcl[1..], 16).is_err() {
            return false;
        }

        // Eye color
        if !VALID_EYE_COLORS.contains(&self.ecl) {
            return false;
        }

        // Passport ID
        if self.pid.len() != 9 {
            return false;
        }
        if !self.pid.chars().all(char::is_numeric) {
            return false;
        }

        true
    }
}
