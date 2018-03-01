extern crate rand;
#[macro_use]
extern crate rand_derive;

use rand::Rng;
use std::vec::Vec;
use std::{thread, time};

//create a constant array of names that will have a static lifetime, we will use this all the time.
const NAMESLIST: [&'static str; 20] = ["Daedalus", "Icaros", "Ares", "Dionysus", "Hades", "Hephaestus",
							   		   "Zeus", "Chronos", "Thanatos", "Atlas", "Talos", "Typhon",
							   		   "Aergia", "Caerus", "Odysseus", "Kratos", "Prometheus", "Proteus",
							   		   "Helios", "Asclepius"
							  		  ];

#[derive(Debug, Rand)]
enum Sex { // 50% creation chance

	Male,
	Female

}

#[derive(Debug, Rand)]
enum Color { //inherited from mother object

	White,
	Brown,
	Black,
	Spotted
}

struct Bunny<'a> { //lifetime declared as a to not outlive the name

	sex: Sex, //50% creation chance
	color: Color, //inherited from parents
	name: &'a str, // randomly chosen from list, lifetime declared as a
	age: u64,
	ghoul: bool //probability of true is 2% on creation
}

impl<'a> Bunny<'a> {

	//announces birth on creation
    fn announcebirth(&self) {

        println!("Bunny {} was born!" ,&self.name);
        dosleep(1);

    }

	//increment age
	fn incrementage(&mut self) {

		self.age += 1;

	}
	// this method checks the age.
	fn shoulddie(&self) -> bool {

		if self.ghoul == true && self.age > 50 {
			println!("ghoul {} should die, age: {}", self.name, self.age);
			return true;
		} else if self.ghoul == false && self.age > 10 {
			println!("{} should fucking die, age: {}", self.name, self.age);
			return true;
		}
		else {
			println!("{} shouldnt die, age: {}", self.name, self.age);
			return false;
		}
	}
}
// explicitly tell us if the object is being dropped, for debugging.
impl<'a> Drop for Bunny<'a> {
	fn drop(&mut self) {
		println!("Dropping {}", self.name);
		dosleep(1);
	}
}


fn dosleep(time: u64) {

    thread::sleep(time::Duration::from_secs(time));

}

// main gameloop, this is the core of this program
fn gameloop(bunnies: &mut Vec<Bunny>, names2: [&'static str; 20]) {

	let mut rng = rand::thread_rng();

	for x in 0..5 { //looping through vector of Bunny structs 5 times. each iteration pushes a Bunny struct with populated fields to the vector bunnies.
		bunnies.push( Bunny { sex: rng.gen(),
							  	   color: rng.gen(),
							  	   name: rng.choose(&names2).unwrap(), // https://habrahabr.ru/post/274485/
							  	   age: 0,
							  	   ghoul: false
						  		 });
		bunnies[x].announcebirth();
	}

	let mut turn: u64 = 1;

	loop {

		println!("Beginning of turn {}", &turn);
		dosleep(2);

		println!("DEBUG: {:?}", bunnies.len().to_string());

		// check if there are bunnies left, game over if the vector is empty.
		if bunnies.is_empty() {
			break;
		}

		//iterate and increment age.
		for x in bunnies.iter_mut() {
			x.incrementage();
		}

		// retain all those bunnies in the vector who should not die, drop those who should.
		bunnies.retain(|i|i.shoulddie() == false);

		println!("DEBUG after: {:?}", bunnies.len().to_string());

		turn += 1;
	}

}

fn greet() {
	println!("///////////////GRADUATION IN RUST////////////////////");
}

fn main() {

	greet();

    let mut bunnies: Vec<Bunny> = Vec::new(); // declaring a mutable vector of structs called bunnies
	gameloop(&mut bunnies, NAMESLIST);

	println!("DEBUG: ENTERING MAIN FUNCTION");

	if bunnies.is_empty() == false {

		println!("DEBUG: THERE ARE STILL {:?} OOBJECTS IN VECTOR, CHECK PROGRAM LOGIC", bunnies.len().to_string());

	}

	println!("PROGRAM TERMINATED");
}
