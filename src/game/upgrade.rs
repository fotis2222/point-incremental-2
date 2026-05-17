use crate::game::player::*;
use num_bigint::{BigInt, ToBigInt};
use std::fmt;

pub struct NoDebug<T>(pub T);

impl<T> fmt::Debug for NoDebug<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<skipped>")
    }
}


#[derive(Debug)]
pub struct Upgrade {
    pub cost: BigInt,
    pub level: BigInt,
    pub max_level: BigInt,
    pub name: String,
    pub desc: String,
    pub currency: Currency,

    pub action: NoDebug<Box<dyn FnMut(&mut Player)>>
}

impl Upgrade {
    pub fn buy(&mut self, player: &mut Player) {
        if player.get_currency(self.currency) < &self.cost || self.level == self.max_level {
            return;
        }

        player.spend(self.currency, &self.cost);

        (self.action.0)(player);
        self.the_basics();
    }

    fn the_basics(&mut self) {
        self.level += 1;
        self.cost = &self.cost * 11.to_bigint().unwrap() / 10.to_bigint().unwrap();
    }
}
