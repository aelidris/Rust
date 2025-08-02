
#[derive(Debug, PartialEq)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}
#[derive(Debug, PartialEq)]

pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}
impl Role {
    pub fn power(&self) -> u32 {
        match self {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }
    }
}

impl Member {
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Underboss => panic!(""),
            Role::Associate => self.role = Role::Soldier,
            Role::Caporegime => self.role = Role::Underboss,
            Role::Soldier => self.role = Role::Caporegime,
        }
    }
}