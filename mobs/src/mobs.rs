pub mod boss;
pub mod member;
pub use boss::*;
pub use member::*;
pub use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, info: (&str, u32)) {
        let member = Member {
            role: Role::Associate,
            age: info.1,
        };
        self.members.insert((info.0).to_string(), member);
    }
    pub fn attack(&mut self, another: &mut Mob) {
        let self_score: u32 = self.members.values().map(|m| m.role.power()).sum();
        let other_score: u32 = another.members.values().map(|m| m.role.power()).sum();

        let (loser, winner) = if self_score > other_score {
            (another, self)
        } else {
            (self, another)
        };

        if let Some(min_age) = loser.members.values().map(|m| m.age).min() {
            loser.members.retain(|_, m| m.age != min_age);
        }

        if loser.members.is_empty() {
            winner.cities.extend(loser.cities.drain());
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }
    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let steal_amount = amount.min(target.wealth);
        target.wealth -= steal_amount;
        self.wealth += steal_amount;
    }

    pub fn conquer_city(&mut self, all: &[&Mob], name: String) {
        let already_taken = all.iter().any(|mob| mob.cities.contains(&name));
        if !already_taken {
            self.cities.insert(name);
        }
    }
}
