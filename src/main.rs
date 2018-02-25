extern crate rand;
#[macro_use]
extern crate rand_derive;



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

use rand::Rng;
use std::vec::Vec;
use std::{thread, time};

struct Bunny<'a> { //lifetime declared as a to not outlive the name

	sex: Sex, //50% creation chance
	color: Color, //inherited from parents
	name: &'a str, // randomly chosen from list, lifetime declared as a
	age: u64,
	radioactive_mutant_vampire_bunny: bool //probability of true is 2% on creation
}

impl<'a> Bunny<'a> {
    fn announcebirth(&self) {

        println!("Bunny {} was born!" ,self.name);
        dosleep(&1);

    }

    fn die(&self) {

        if &self.radioactive_mutant_vampire_bunny == &false {

            if &self.age > &10 {

                println!("Bunny {} has died!" ,self.name);
                dosleep(&1);
                drop(&self); //destruct the object and delete it

        } else {

            if &self.age > &50 {

                println!("Radioactive Mutant Vampire Bunny {} has died!" ,self.name);
                dosleep(&1);
                drop(&self); //destruct the object and delete it

                }
            }
        }
    }
}


fn dosleep(time: &u64) {

    thread::sleep(time::Duration::from_secs(*time));

}

fn main() {

    let listofnames: [&str; 20] = ["Daedalus", "Icaros", "Ares", "Dionysus", "Hades", "Hephaestus",
                                   "Zeus", "Chronos", "Thanatos", "Atlas", "Talos", "Typhon",
                                   "Aergia", "Caerus", "Odysseus", "Kratos", "Prometheus", "Proteus",
                                   "Helios", "Asclepius"
                                  ];

    println!("///////////////graduation in rust////////////////////");
    let mut rng = rand::thread_rng();
    let mut bunnies = Vec::new();

    for _x in 0..5 {
        bunnies.push( Bunny { sex: rng.gen(),
                              color: rng.gen(),
                              name: rng.choose(&listofnames).unwrap(), // https://habrahabr.ru/post/274485/
                              age: 0,
                              radioactive_mutant_vampire_bunny: false
                            })
    }


    let mut turn: u64 = 0;

    loop { //main loop

        println!("new turn nr. {} has begun!", &turn);
        dosleep(&1);

        for bunny in bunnies {

            bunny.die();
            bunny.announcebirth();

        }

        if  &bunnies.is_empty() == &true {

            println!("All bunnies have died, the turn is {}. program will now terminate!", &turn);
            dosleep(&1);
            break;

        }

        turn += 1;
        dosleep(&2);
    }


}
