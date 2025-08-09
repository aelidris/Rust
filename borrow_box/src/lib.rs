use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u32),
    pub p2: (String, u32),
    pub nb_games: u32,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u32) -> GameSession {
        GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        }
    }

    #[inline]
    pub fn read_winner(&self) -> Option<&(String, u32)> {
        match self.p1.1.cmp(&self.p2.1) {
            Ordering::Greater => Some(&self.p1),
            Ordering::Less => Some(&self.p2),
            Ordering::Equal => None,
        }
    }

    #[inline]
    fn game_ended(&self) -> bool {
        self.p1.1 * 2 > self.nb_games ||
            self.p2.1 * 2 > self.nb_games ||
            self.p1.1 + self.p2.1 == self.nb_games
    }

    pub fn update_score(&mut self, user_name: &str) {
        if self.game_ended() {
            return;
        }

        if user_name == self.p1.0 {
            self.p1.1 += 1;
        } else if user_name == self.p2.0 {
            self.p2.1 += 1;
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
