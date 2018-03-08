use game;
//create a constant array of names that will have a static lifetime, we will use this all the time. many functions reference to its element values.
pub const NAMESLIST: [&'static str; 20] =
[
"Daedalus", "Icaros", "Ares", "Dionysus", "Hades", "Hephaestus",
"Zeus", "Chronos", "Thanatos", "Atlas", "Talos", "Typhon",
"Aergia", "Caerus", "Odysseus", "Kratos", "Prometheus", "Proteus",
"Helios", "Asclepius"
];

#[derive(Debug, Rand, PartialEq, Clone)]
pub enum Sex
{ // 50% creation chance
	Male,
	Female
}

#[derive(Debug, Rand, PartialEq, Clone)]
pub enum Color
{ //inherited from mother object
	White,
	Brown,
	Black,
	Spotted
}

#[derive(PartialEq, Clone)]
pub struct Bunny<'a>
{ //lifetime declared as a to not outlive the name
	pub sex: Sex, //50% creation chance
	pub color: Color, //inherited from parents
	pub name: &'a str, // randomly chosen from list, lifetime declared as a
	pub age: u64,
	pub ghoul: bool //probability of true is 2% on creation
}

impl<'a> Bunny<'a>
{
	//announces birth on creation
    pub fn announcebirth(&self)
	{
        println!("Bunny {} was born!" ,&self.name);
        game::dosleep(1);
    }

	//increment age
	pub fn incrementage(&mut self)
	{
		self.age += 1;
	}
	// this method checks the age.
	pub fn shoulddie(&self) -> bool
	{
		if self.ghoul == true && self.age > 50
		{
			println!("ghoul {} should die, age: {}", self.name, self.age);
			return true;
		}
		else if self.ghoul == false && self.age > 10
		{
			println!("{} should fucking die, age: {}", self.name, self.age);
			return true;
		}
		else
		{
			println!("{} shouldnt die, age: {}", self.name, self.age);
			return false;
		}
	}
}
// explicitly tell us if the object is being dropped, for debugging.
impl<'a> Drop for Bunny<'a>
{
	fn drop(&mut self)
	{
		println!("Dropping {}", self.name);
		game::dosleep(1);
	}
}
