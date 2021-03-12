use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Speed {
    Slow,
    Normal,
    Fast,
}

impl FromStr for Speed {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "1" => Self::Slow,
                "2" => Self::Normal,
                _ => Self::Fast,
            }
        )
    }
}

#[derive(PartialEq, Debug)]
pub enum SkinSnake {
    Snake,
    Chiken,
    LadyBeelt,
}

impl FromStr for SkinSnake {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "snake" => Self::Snake,
                "chiken" => Self::Chiken,
                _ => Self::LadyBeelt,
            }
        )
    }
}

#[derive(PartialEq, Debug)]
pub enum SkinApple {
    Apple,
    Pinapple,
    Rice,
}

impl FromStr for SkinApple {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "apple" => Self::Apple,
                "pinapple" => Self::Pinapple,
                _ => Self::Rice,
            }
        )
    }
}