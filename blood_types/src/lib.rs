use std::cmp::{Ord, Ordering};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len < 2 {
            return Err(());
        }
        let antigen_str = &s[..len - 1];
        let rh_str = &s[len - 1..];
        let antigen = Antigen::from_str(antigen_str)?;
        let rh_factor = RhFactor::from_str(rh_str)?;
        Ok(BloodType { antigen, rh_factor })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            ord => ord,
        }
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        (other.antigen == Antigen::O || other.antigen == self.antigen || self.antigen == Antigen::AB)
            && (other.rh_factor == RhFactor::Negative || self.rh_factor == RhFactor::Positive)
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_antigens = vec![Antigen::A, Antigen::AB, Antigen::B, Antigen::O];
        let all_rh_factors = vec![RhFactor::Positive, RhFactor::Negative];
        let mut donors = Vec::new();
        for antigen in all_antigens {
            for rh_factor in &all_rh_factors {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if self.can_receive_from(&donor) {
                    donors.push(donor);
                }
            }
        }
        donors
    }

    pub fn recipients(&self) -> Vec<Self> {
        let all_antigens = vec![Antigen::A, Antigen::AB, Antigen::B, Antigen::O];
        let all_rh_factors = vec![RhFactor::Positive, RhFactor::Negative];
        let mut recipients = Vec::new();
        for antigen in all_antigens {
            for rh_factor in &all_rh_factors {
                let recipient = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if recipient.can_receive_from(self) {
                    recipients.push(recipient);
                }
            }
        }
        recipients
    }
}