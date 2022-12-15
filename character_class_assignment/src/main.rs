trait CharacterTraits {
    fn new(name: &str, phrase1: &str, phrase2: &str) -> Self;
    fn speak(&self, phrase_num: u8);
    fn set_level(&mut self, new_level: u32);
}

struct Character {
    // The name of the character.  Initialize to constructor argument.
    name: String,
    // Character’s primary catchphrase.  Initialize to constructor argument.
    phrase1: String,
    // Character’s secondary catchphrase.  Initialize to constructor argument.
    phrase2: String,
    // Character’s current level.  Initialize to 0.
    level: u32,
}

impl CharacterTraits for Character {
    fn new(name: &str, phrase1: &str, phrase2: &str) -> Self {
        Character {
            name: String::from(name),
            phrase1: String::from(phrase1),
            phrase2: String::from(phrase2),
            level: 0,
        }
    }
    // Print out the character’s primary or secondary catch phrase based on the value of the phrase_num argument.  If phrase_num is 1, print out the character’s primary catchphrase (phrase1).  If phrase_num is 2, print out the character’s secondary catchphrase (phrase2).
    fn speak(&self, phrase_num: u8) {
        match phrase_num {
            1 => println!("{}", self.phrase1),
            2 => println!("{}", self.phrase2),
            _ => println!("Invalid operation"),
        }
    }
    // Set the character’s level property to the value of the new_level argument.  Print out the character’s new level.
    fn set_level(&mut self, new_level: u32) {
        self.level = new_level;
        println!("{}'s new level is {}", self.name, self.level);
    }
}

fn main() {
    let spiderman = Character::new(
        "Spiderman",
        "My Spider-Sense is tingling",
        "Your friendly neighbourhood spiderman",
    );

    let mut kung_fu_panda = Character::new(
        "Kung Fu Panda",
        "Skadoosh",
        "You have been blinded by pure awesomeness!",
    );

    spiderman.speak(1);
    kung_fu_panda.set_level(2);
    kung_fu_panda.speak(2);
}
