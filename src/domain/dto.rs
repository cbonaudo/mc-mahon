// Fight
#[derive(Debug)]
pub struct Fighters {
    pub fighter1: String,
    pub fighter2: String,
}
#[derive(Debug)]
pub struct GetUpperHand {
    pub fighter1: String,
    pub fighter2: String,
    pub champion_name: String,
}

impl GetUpperHand {
    pub fn for_fight(fighters: Fighters, champion_name: String) -> GetUpperHand {
        GetUpperHand {
            fighter1: fighters.fighter1,
            fighter2: fighters.fighter2,
            champion_name,
        }
    }
    pub fn fighters(&self) -> GetUpperHandFighters {
        GetUpperHandFighters::Two(self.fighter1.as_str(), self.fighter2.as_str())
    }
}

pub enum GetUpperHandFighters<'a> {
    Two(&'a str, &'a str),
    One(&'a str),
    Zero,
}

impl<'a> Iterator for GetUpperHandFighters<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let (this, output) = match self {
            GetUpperHandFighters::Two(a, b) => (GetUpperHandFighters::One(b), Some(*a)),
            GetUpperHandFighters::One(a) => (GetUpperHandFighters::Zero, Some(*a)),
            GetUpperHandFighters::Zero => (GetUpperHandFighters::Zero, None),
        };

        *self = this;
        output
    }
}

#[derive(Debug)]
pub struct UpperHand {
    pub victor: String,
    pub vanquished: String,
}

impl From<UpperHand> for GetFinisher {
    fn from(uh: UpperHand) -> Self {
        Self {
            victor: uh.victor,
            vanquished: uh.vanquished,
        }
    }
}

#[derive(Debug)]
pub struct GetFinisher {
    pub victor: String,
    pub vanquished: String,
}

// Chat
#[derive(Debug)]
pub struct SendMessage {
    pub message: String,
}
