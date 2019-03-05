use std::{thread, time, vec::Vec};

use rand::seq::SliceRandom;

pub mod bunny;

pub const MAXPOPULATION: u64 = 1000;

pub fn gameloop() {
    let mut colony: Vec<bunny::Bunny> = Vec::new();
    let mut rng = rand::thread_rng();

    for _x in 0..5 {
        colony.push(bunny::Bunny {
            sex: rand::random(),
            color: rand::random(),
            name: &bunny::MALENAMES.choose(&mut rng).unwrap(),
            age: 0,
            ghoul: false,
        });
    }

    let mut turn: u64 = 0;

    loop {
        println!("Beginning of turn {}", &turn);
        dosleep(2);

        turn += 1;

        #[cfg(debug_assertations)]
        println!("DEBUG beginning: {:?}", colony.len().to_string());

        //iterate and increment age.
        for x in colony.iter_mut() {
            x.incrementage();
        }

        // retain all those bunnies in the vector who should not die, drop those who should.
        colony.retain(|i| i.shoulddie() == false);
        println!("DEBUG after retain: {:?}", colony.len().to_string());

        // check if there are bunnies left, game over if the vector is empty.
        if colony.is_empty() {
            println!("DEBUG: no colony alive");
            break;
        }

        // if colony.len() >= MAXPOPULATION {
        //     for i
        // }

        //breeding
        breed(&mut colony);

        //call infect if there are ghouls
        //infect()
    }
}

//figure out who is suitable for breeding and populate the referenced vector
fn breed(colony: &mut Vec<bunny::Bunny>) {
    let mut rng = rand::thread_rng();

    //find 1 male who is atleast 2 years old and gather indexes of all females who are atleast 2 years old.
    //let suitablemale = colony.iter().find(|&x| x.age >= 2 && x.sex == Sex::Male );

    for i in 0..colony.len() {
        if colony[i].age >= 2 && colony[i].sex == bunny::Sex::Male && colony[i].ghoul == false {
            for j in 0..colony.len() {
                if colony[j].age >= 2
                    && colony[j].sex == bunny::Sex::Female
                    && colony[j].ghoul == false
                {
                    colony.push(bunny::Bunny {
                        sex: rand::random(),
                        color: colony[j].color,
                        name: //nameselector(&self.sex),//&bunny::MALENAMES.choose(&mut rng).unwrap(),
                        age: 0,
                        ghoul: false,
                    });
                }
            }
        }
    }
}

pub fn dosleep(time: u64) {
    thread::sleep(time::Duration::from_secs(time));
}

pub fn nameselector(element: &bunny::Bunny) -> &'static str {
    let mut rng = rand::thread_rng();

    match &element.sex {
        Male => &bunny::MALENAMES.choose(&mut rng).unwrap(),
        _ => &bunny::FEMALENAMES.choose(&mut rng).unwrap()
    }
}
