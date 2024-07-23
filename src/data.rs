pub struct Data {
    pub proverb: String,
    pub speaker: String,
    pub date: String,
    pub rarity: Rarity,
}

impl Data {
    pub fn new(proverb: String, speaker: String, date: String, rarity: String) -> Self {
        Data {
            proverb,
            speaker,
            date,
            rarity: Rarity::from(rarity),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Rarity {
    Common,
    Rare,
    SuperRare,
}

impl From<String> for Rarity {
    fn from(from: String) -> Rarity {
        match from.as_str() {
            "☆☆" => Rarity::Rare,
            "☆☆☆" => Rarity::SuperRare,
            _ => Rarity::Common,
        }
    }
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Rarity::Common => write!(f, "☆"),
            Rarity::Rare => write!(f, "☆☆"),
            Rarity::SuperRare => write!(f, "☆☆☆"),
        }
    }
}
