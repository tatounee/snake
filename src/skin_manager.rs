use crate::parameter::{SkinApple, SkinSnake};

pub struct SkinManager {
    skin_apple: [String; 3],
    skin_snake: [[String; 16]; 3],
    current_skin_apple: usize,
    current_skin_snake: usize,
}

impl SkinManager {
    pub fn new() -> Self {
        Self{
            skin_apple: ["tile-apple".to_owned(), "tile-pinapple".to_owned(), "tile-rice".to_owned()],
            skin_snake: [
                ["head-snake-up".to_owned(), "head-snake-down".to_owned(), "head-snake-left".to_owned(), "head-snake-right".to_owned(), 
                    "body-snake-down-up".to_owned(), "body-snake-up-down".to_owned(), "body-snake-left-right".to_owned(),"body-snake-right-left ".to_owned(), 
                    "body-snake-up-left".to_owned(), "body-snake-up-right".to_owned(), "body-snake-down-left".to_owned(), "body-snake-down-right".to_owned(),  
                    "queue-snake-up".to_owned(), "queue-snake-down".to_owned(), "queue-snake-left".to_owned(), "queue-snake-right".to_owned()],
                ["head-ladybeetle-up".to_owned(), "head-ladybeetle-down".to_owned(), "head-ladybeetle-left".to_owned(), "head-ladybeetle-right".to_owned(), 
                    "body-ladybeetle-down-up".to_owned(), "body-ladybeetle-up-down".to_owned(), "body-ladybeetle-left-right".to_owned(), "body-ladybeetle-right-left ".to_owned(),
                    "body-ladybeetle-up-left".to_owned(), "body-ladybeetle-up-right".to_owned(), "body-ladybeetle-down-left".to_owned(), "body-ladybeetle-down-right".to_owned(), 
                    "queue-ladybeetle-up".to_owned(), "queue-ladybeetle-down".to_owned(), "queue-ladybeetle-left".to_owned(), "queue-ladybeetle-right".to_owned()],
                ["head-duck-up".to_owned(), "head-duck-down".to_owned(), "head-duck-left".to_owned(), "head-duck-right".to_owned(), 
                    "body-duck-down-up".to_owned(), "body-duck-up-down".to_owned(), "body-duck-left-right".to_owned(), "body-duck-right-left ".to_owned(), 
                    "body-duck-up-left".to_owned(), "body-duck-up-right".to_owned(), "body-duck-down-left".to_owned(), "body-duck-down-right".to_owned(), 
                    "queue-duck-up".to_owned(), "queue-duck-down".to_owned(), "queue-duck-left".to_owned(), "queue-duck-right".to_owned()]

            ],
            current_skin_apple: 0,
            current_skin_snake: 0,
        }
    }

    pub fn get_skin_apple(&self) -> &String {
        self.skin_apple.get(self.current_skin_apple).unwrap()
    }

    pub fn get_skin_snake(&self, index: usize) -> &String {
        self.skin_snake[self.current_skin_snake].get(index).unwrap()
    }

    pub fn set_current_skin_apple(&mut self, skin_apple: &SkinApple) {
        self.current_skin_apple = match skin_apple {
            SkinApple::Apple => 0,
            SkinApple::Pinapple => 1,
            SkinApple::Rice => 2
        }
    }

    pub fn set_current_skin_snake(&mut self, skin_snake: &SkinSnake) {
        self.current_skin_snake = match skin_snake {
            SkinSnake::Snake => 0,
            SkinSnake::LadyBeelt => 1,
            SkinSnake::Chiken => 2,
        }
    }
}