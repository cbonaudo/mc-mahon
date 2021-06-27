// Fight
#[derive(Debug)]
pub struct Fighters {
    pub fighter1: String,
    pub fighter2: String,
}

impl From<Fighters> for GetUpperHand {
    fn from(f: Fighters) -> Self {
        Self {
            fighter1: f.fighter1,
            fighter2: f.fighter2,
        }
    }
}

#[derive(Debug)]
pub struct GetUpperHand {
    pub fighter1: String,
    pub fighter2: String,
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
