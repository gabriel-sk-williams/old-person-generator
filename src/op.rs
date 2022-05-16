mod roll;

pub struct OldPerson {
    pub name: String,
    pub age: usize,
    pub strength: usize,
    pub sight: usize,
    pub mobility: usize,
    pub wisdom: usize,
    pub haggard: usize,
    pub ailments: Vec<String>
}

impl OldPerson {
    pub fn create(gender: &str) -> OldPerson {
        let age = roll::age();
        let ailments = roll::ailments(age);

        let str_mod: f64 = if ailments.contains(&String::from("Frail")) {0.8} else {1.0};
        let sight_mod: f64 = if ailments.contains(&String::from("Cataracts")) {0.8} else {1.0};
        let mob_mod: f64 = if ailments.contains(&String::from("Mobile-impaired")) {0.8} else {1.0};
        let wis_mod: f64 = if ailments.contains(&String::from("Alzheimer's")) {0.8} else {1.0};

        OldPerson {
            name: roll::name(gender),
            age: age,
            strength: roll::stat(age, str_mod), // Frail
            sight: roll::stat(age, sight_mod), // Cataracts
            mobility: roll::stat(age, mob_mod), // Mobile-impaired
            wisdom: roll::stat(age, wis_mod), // Alzheimer's
            haggard: roll::stat(age, 1.0),
            ailments: ailments
        }
    }

    pub fn describe(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Strength: {}", self.strength);
        println!("Sight: {}", self.sight);
        println!("Mobility: {}", self.mobility);
        println!("Wisdom: {}", self.wisdom);
        println!("Haggard: {}", self.haggard);
        println!("Ailments: {:?}", self.ailments);
        println!("\r");
    }
}