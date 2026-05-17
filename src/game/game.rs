use crate::game::player::Player;
use crate::game::upgrade::{Upgrade, NoDebug};
use crate::game::player::Currency;

pub enum Upgrades {
    PointsI = 0,
    XPI = 1
}

#[derive(Debug, Default)]
pub struct Game {
    pub player: Player,
    pub upgrades: Vec<Upgrade>
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::default(),
            upgrades: vec![
                Upgrade {
                    cost: 10.into(),
                    level: 0.into(),
                    max_level: 500.into(),
                    name: "Points I".to_string(),
                    desc: "Earn +100% points per level.".to_string(),
                    currency: Currency::Points,
                    action: NoDebug(Box::new(|player: &mut Player| {
                        player.points_multi += 1;
                    }))
                },
                Upgrade {
                    cost: 10000.into(),
                    level: 0.into(),
                    max_level: 30.into(),
                    name: "XP I".to_string(),
                    desc: "Earn 2x XP per level.".to_string(),
                    currency: Currency::Points,
                    action: NoDebug(Box::new(|player: &mut Player| {
                        player.xp_mult *= 2;
                    }))
                }
            ]
        }
    }
}
