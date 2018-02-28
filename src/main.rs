extern crate rand;
#[macro_use]
extern crate rand_derive;

use rand::Rng;
use std::vec::Vec;
use std::{thread, time};

//create a constant array of names that will have a static lifetime
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
    fn announcebirth(&self) {

        println!("Bunny {} was born!" ,&self.name);
        dosleep(1);

    }

	fn incrementage(&mut self) {

		self.age += 1; //increment age

	}
}

impl<'a> Drop for Bunny<'a> {
	fn drop(&mut self) {
		println!("Dropping {}", self.name);
		dosleep(1);
	}
}


fn dosleep(time: u64) {

    thread::sleep(time::Duration::from_secs(time));

}

fn gameloop(bunnies: &mut Vec<Bunny>, names2: [&'static str; 20]) {

	let mut rng = rand::thread_rng();

	for x in 0..5 { //looping through vector of Bunny structs 5 times. each iteration pushes a Bunny struct with populated fields to the vector bunnies.
		&mut bunnies.push( Bunny { sex: rng.gen(),
							  	   color: rng.gen(),
							  	   name: rng.choose(&names2).unwrap(), // https://habrahabr.ru/post/274485/
							  	   age: 0,
							  	   ghoul: false
						  		 });
		&mut bunnies[x].announcebirth();
	}

	let mut turn: u64 = 1;

	loop {

		println!("Beginning of turn {}", &turn);
		dosleep(2);

		for x in bunnies.iter_mut() {

			x.incrementage();
			println!("Before: {:?}", x.name);

			if x.age > 10 {
				//bunnies.remove_item(); ok figure this &#it out too
			}
		}

		turn += 1;
	}

}

fn greet() {

	println!("///////////////graduation in rust////////////////////");

}

fn main() {

	greet();

    let mut bunnies: Vec<Bunny> = Vec::new(); // declaring a mutable vector of structs called bunnies
	gameloop(&mut bunnies, NAMESLIST); // wtf is going on?

	println!("MAIN FUNCTION");

	for bunny in bunnies {

		println!("DEBUG: {} in main scope", bunny.name);

	}

	println!("PROGRAM TERMINATED");
}
