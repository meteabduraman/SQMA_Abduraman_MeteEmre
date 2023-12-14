struct Cat {
    mood: i32,
    hungry: bool,
    energy: i32,
}

impl Cat {
    fn meow(&self) -> &'static str {
        "Meow!"
    }

    fn play(&mut self) -> Result<&str, &str> {
        if self.hungry {
            return Err("Feed me!");
        }

        if self.energy < 3 {
            return Err("I need to sleep!");
        }

        if self.mood > 10 {
            return Err("Zoomies everywhere!");
        }

        self.mood += 1;
        self.energy -= 1;
        if self.energy < 5 {
            self.hungry = true;
        }
        return Ok("Playing...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_meows_regardless_of_mood() {
        let cat = Cat {
            mood: 0,
            hungry: false,
            energy: 0,
        };

        assert_eq!(cat.meow(), String::from("Meow!"));
    }

    #[test]
    fn it_wants_food_if_its_hungry() -> Result<(), String> {
        let mut cat = Cat {
            mood: 5,
            hungry: true,
            energy: 5,
        };

        return match cat.play() {
            Ok(_) => {
                Err(String::from("Should have wanted food"))
            }
            Err(_) => {
                Ok(())
            }
        };
    }

    #[test]
    fn it_wants_to_sleep_if_energy_is_low() -> Result<(), String> {
        let mut cat = Cat {
            mood: 5,
            hungry: false,
            energy: 2,
        };

        return match cat.play() {
            Ok(_) => {
                Err(String::from("Should have wanted to sleep"))
            }
            Err(_) => {
                Ok(())
            }
        };
    }

    #[test]
    fn it_does_zoomies_if_mood_is_too_high() -> Result<(), String> {
        let mut cat = Cat {
            mood: 20,
            hungry: false,
            energy: 10,
        };

        return match cat.play() {
            Ok(_) => {
                Err(String::from("Should have done zoomies everywhere"))
            }
            Err(_) => {
                Ok(())
            }
        };
    }

    #[test]
    fn playing_should_increase_its_mood_and_lower_its_energy() -> Result<(), String> {
        let mut cat = Cat {
            mood: 5,
            hungry: false,
            energy: 5,
        };

        return match cat.play() {
            Ok(_) => {
                assert_eq!(cat.energy, 4);
                assert_eq!(cat.hungry, true);
                Ok(())
            }
            Err(_) => {
                Err(String::from("Should have played"))
            }
        };
    }

    #[test]
    fn playing_should_make_it_hungry_if_energy_gets_low() -> Result<(), String> {
        let mut cat = Cat {
            mood: 5,
            hungry: false,
            energy: 5,
        };

        return match cat.play() {
            Ok(_) => {
                assert_eq!(cat.energy, 4);
                assert_eq!(cat.mood, 6);
                Ok(())
            }
            Err(_) => {
                Err(String::from("Should have played"))
            }
        };
    }
}