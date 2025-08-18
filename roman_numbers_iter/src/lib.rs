use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Unsupported Roman digit value"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut digits = Vec::new();

        let thousands = num / 1000;
        digits.extend(vec![M; thousands as usize]);
        num %= 1000;

        let hundreds = num / 100;
        match hundreds {
            1 => digits.push(C),
            2 => digits.extend(vec![C, C]),
            3 => digits.extend(vec![C, C, C]),
            4 => digits.extend(vec![C, D]),
            5 => digits.push(D),
            6 => digits.extend(vec![D, C]),
            7 => digits.extend(vec![D, C, C]),
            8 => digits.extend(vec![D, C, C, C]),
            9 => digits.extend(vec![C, M]),
            _ => (),
        }
        num %= 100;

        let tens = num / 10;
        match tens {
            1 => digits.push(X),
            2 => digits.extend(vec![X, X]),
            3 => digits.extend(vec![X, X, X]),
            4 => digits.extend(vec![X, L]),
            5 => digits.push(L),
            6 => digits.extend(vec![L, X]),
            7 => digits.extend(vec![L, X, X]),
            8 => digits.extend(vec![L, X, X, X]),
            9 => digits.extend(vec![X, C]),
            _ => (),
        }
        num %= 10;

        let units = num;
        match units {
            1 => digits.push(I),
            2 => digits.extend(vec![I, I]),
            3 => digits.extend(vec![I, I, I]),
            4 => digits.extend(vec![I, V]),
            5 => digits.push(V),
            6 => digits.extend(vec![V, I]),
            7 => digits.extend(vec![V, I, I]),
            8 => digits.extend(vec![V, I, I, I]),
            9 => digits.extend(vec![I, X]),
            _ => (),
        }

        RomanNumber(digits)
    }
}

impl From<RomanNumber> for u32 {
    fn from(roman: RomanNumber) -> u32 {
        let mut total = 0;
        let mut prev_value = 0;

        for &digit in roman.0.iter().rev() {
            let value = match digit {
                Nulla => 0,
                I => 1,
                V => 5,
                X => 10,
                L => 50,
                C => 100,
                D => 500,
                M => 1000,
            };

            if value < prev_value {
                total -= value;
            } else {
                total += value;
            }

            prev_value = value;
        }

        total
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let current_value: u32 = self.clone().into();
        let next_value = current_value + 1;
        *self = RomanNumber::from(next_value);
        Some(self.clone())
    }
}