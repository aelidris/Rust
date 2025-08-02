use std::collections::{HashMap, HashSet};
mod boss;
mod member;

pub use boss::Boss;
pub use member::{Member, Role};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn new(name: String, boss: Boss) -> Self {
        Mob {
            name,
            boss,
            members: HashMap::new(),
            cities: HashSet::new(),
            wealth: 0,
        }
    }

    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(name.to_string(), Member::new(age));
    }

    pub fn attack(&mut self, target: &mut Mob) {
        let self_power = self.combat_score();
        let target_power = target.combat_score();

        if self_power > target_power {
            target.remove_youngest();
            if target.members.is_empty() {
                self.cities.extend(target.cities.drain());
                self.wealth += target.wealth;
                target.wealth = 0;
            }
        } else {
            self.remove_youngest();
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let steal_amount = amount.min(target.wealth);
        target.wealth -= steal_amount;
        self.wealth += steal_amount;
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], city: String) {
        if !mobs.iter().any(|m| m.cities.contains(&city)) {
            self.cities.insert(city);
        }
    }

    fn combat_score(&self) -> u32 {
        self.members.values().map(|m| match m.role {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }).sum()
    }

    fn remove_youngest(&mut self) {
    let youngest_name = self.members.iter()
        .min_by_key(|(_, member)| member.age)
        .map(|(name, _)| name.clone());
    
    if let Some(name) = youngest_name {
        self.members.remove(&name);
    }
}
}