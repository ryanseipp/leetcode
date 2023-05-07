#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum RomanSymbol {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl TryFrom<char> for RomanSymbol {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'I' => Ok(RomanSymbol::I),
            'V' => Ok(RomanSymbol::V),
            'X' => Ok(RomanSymbol::X),
            'L' => Ok(RomanSymbol::L),
            'C' => Ok(RomanSymbol::C),
            'D' => Ok(RomanSymbol::D),
            'M' => Ok(RomanSymbol::M),
            _ => Err(()),
        }
    }
}

struct RomanNumber {
    inner: Vec<RomanSymbol>,
}

impl TryFrom<&str> for RomanNumber {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut symbols = Vec::new();
        for c in value.chars() {
            match RomanSymbol::try_from(c) {
                Ok(symbol) => symbols.push(symbol),
                Err(_) => return Err(()),
            }
        }

        Ok(Self { inner: symbols })
    }
}

impl RomanNumber {
    pub fn as_integer(&self) -> i32 {
        let mut sum = 0;
        for (current, next) in self.inner.iter().zip(
            self.inner
                .iter()
                .skip(1)
                .map(Some)
                .chain(std::iter::once(None)),
        ) {
            if let Some(next) = next {
                if current < next {
                    sum -= *current as i32;
                    continue;
                }
            }
            sum += *current as i32;
        }

        sum
    }
}

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman = RomanNumber::try_from(s.as_str()).unwrap();
        roman.as_integer()
    }
}

fn main() {}
