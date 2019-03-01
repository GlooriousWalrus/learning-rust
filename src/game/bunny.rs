use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

//create a constant array of names that will have a static lifetime, we will use this all the time. Many functions reference to its element values.
pub const MALENAMES: [&'static str; 16] = [
    "Jacob",
    "Michael",
    "Matthew",
    "Joshua",
    "Christopher",
    "Nicholas",
    "Andrew",
    "Joseph",
    "Daniel",
    "Tyler",
    "William",
    "Brandon",
    "Ryan",
    "John",
    "Zachary",
    "David",
];

#[derive(Debug, PartialEq, Clone)]
pub enum Sex {
    Male,
    Female,
}

impl Distribution<Sex> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sex {
        match rng.gen_range(0, 2) {
            0 => Sex::Male,
            _ => Sex::Female,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    //inherited from mother object
    White,
    Brown,
    Black,
    Spotted,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0, 3) {
            0 => Color::White,
            1 => Color::Brown,
            2 => Color::Black,
            _ => Color::Spotted,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Bunny<'a> {
    //lifetime declared as a to not outlive the name
    pub sex: Sex,      //50% creation chance
    pub color: Color,  //inherited from parents
    pub name: &'a str, // randomly chosen from list, lifetime declared as a
    pub age: u64,
    pub ghoul: bool, //probability of true is 2% on creation
}

impl<'a> Bunny<'a> {
    //announces birth on creation
    pub fn announcebirth(&self) {
        if self.ghoul == true {
            println!(
                "A {:?} {:?} bunny ghoul named {:?} was born!",
                &self.sex, &self.color, &self.name
            );
        } else {
            println!(
                "A {:?} {:?} bunny named {:?} was born!",
                &self.sex, &self.color, &self.name
            );
        }
        super::dosleep(1);
    }

    // this method checks the age.
    pub fn shoulddie(&self) -> bool {
        if self.ghoul == true && self.age > 50 {
            println!("ghoul {} should die, age: {}", &self.name, &self.age);
            return true;
        } else if self.ghoul == false && self.age > 10 {
            println!("{} should die, age: {}", &self.name, &self.age);
            return true;
        } else {
            println!("{} aged to {}", &self.name, &self.age);
            return false;
        }
    }

    //increment age
    pub fn incrementage(&mut self) {
        self.age += 1;
    }
}
// explicitly tell us if the object is being dropped, for debugging.
impl<'a> Drop for Bunny<'a> {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
        super::dosleep(1);
    }
}
