use num_bigint::{BigInt, ToBigInt};

#[derive(Debug)]
pub struct Player {
    pub points: BigInt,
    pub points_multi: BigInt,
    pub xp: BigInt,
    pub xp_mult: BigInt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Points,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::Points => "Points"
        }
    }
}

impl Player {
    pub fn gain_points(&mut self) {
        self.points += &self.points_multi;
        self.xp += &self.xp_mult;
    }

    pub fn get_currency(&self, c: Currency) -> &BigInt {
        match c {
            Currency::Points => &self.points,
        }
    }

    pub fn spend(&mut self, c: Currency, amount: &BigInt) {
        match c {
            Currency::Points => self.points -= amount,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            points: 0.to_bigint().unwrap(),
            points_multi: 1.to_bigint().unwrap(),
            xp: 0.to_bigint().unwrap(),
            xp_mult: 1.to_bigint().unwrap(),
        }
    }
}
