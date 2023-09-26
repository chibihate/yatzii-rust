pub mod dices {
    use rand::{self, Rng};
    #[derive(Copy, Clone, Debug)]
    pub struct Dice {
        is_lock: bool,
        side: u8,
    }

    impl Dice {
        pub fn origin() -> Dice {
            Dice {
                is_lock: false,
                side: 0,
            }
        }

        #[allow(dead_code)]
        pub fn get_is_lock(&self) -> bool {
            self.is_lock
        }

        #[allow(dead_code)]
        pub fn get_side(&self) -> u8 {
            self.side
        }

        #[allow(dead_code)]
        pub fn set_is_lock(&mut self, _is_lock: bool) {
            self.is_lock = _is_lock;
        }

        #[allow(dead_code)]
        pub fn set_side(&mut self, _side: u8) {
            self.side = _side;
        }

        #[allow(dead_code)]
        pub fn roll(&mut self) {
            let random: u8 = rand::thread_rng().gen_range(1..=6);
            if self.is_lock == false {
                self.side = random
            }
        }
    }
    #[derive(Copy, Clone, Debug)]
    #[allow(dead_code)]
    pub struct Dices {
        value: [Dice; 5],
    }

    impl Dices {
        pub fn origin() -> Dices {
            Dices {
                value: [Dice {
                    is_lock: false,
                    side: 0,
                }; 5],
            }
        }

        #[allow(dead_code)]
        pub fn get_value(&self) -> [Dice; 5] {
            self.value
        }

        #[allow(dead_code)]
        pub fn roll(&mut self) {
            for dice in self.value.iter_mut() {
                dice.roll();
            }
        }
    }
}
