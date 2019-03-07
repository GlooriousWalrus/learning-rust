use std::{thread, time, vec::Vec};

use rand::seq::SliceRandom;

pub mod bunny;

pub const MAXPOPULATION: usize = 1000;

pub fn gameloop() {
    let mut colony: Vec<bunny::Bunny> = Vec::new();
    let mut rng = rand::thread_rng();

    for x in 0..5 {
        let mut temp: bunny::Sex = rand::random();
        colony.push(bunny::Bunny {
            sex: temp,
            color: rand::random(),
            name: nameselector(&temp),
            age: 0,
            ghoul: false,
        });
        colony[x].announcebirth();
    }

    let mut turn: u64 = 0;

    loop {
        println!("Beginning of turn {}", &turn);
        dosleep(2);

        turn += 1;

        #[cfg(debug_assertations)]
        println!("DEBUG: {:?} bunnies alive", colony.len().to_string());

        // check if there are bunnies left, game over if the vector is empty.
        if ispopulationdead(&mut colony) { break; }

        //iterate and increment age.
        for x in colony.iter_mut() {
            x.incrementage();
        }

        // retain all those bunnies in the vector who should not die, drop those who should.
        colony.retain(|i| i.shoulddie() == false);
        println!("DEBUG: post-age retain population is {:?}", colony.len().to_string());
        dosleep(2);

        if ispopulationdead(&mut colony) { break; }

        //breeding
        breed(&mut colony);

        println!("DEBUG: post breed population is {:?}", colony.len().to_string());
        dosleep(2);

        if colony.len() >= MAXPOPULATION {
            cull(&mut colony);
            println!("DEBUG: cull survivor population is {:?}", colony.len().to_string());
            dosleep(2);
        }

        //call infect if there are ghouls
        //infect()
    }
}

//figure out who is suitable for breeding and populate the referenced vector
fn breed(colony: &mut Vec<bunny::Bunny>) {
    let mut rng = rand::thread_rng();

    //find 1 male who is atleast 2 years old and gather indexes of all females who are atleast 2 years old.
    //let suitablemale = colony.iter().find(|&x| x.age >= 2 && x.sex == Sex::Male && x.ghoul == false);

        if colony.iter().find(|&x| x.age >= 2 && x.sex == bunny::Sex::Male && x.ghoul == false) != None {
            for j in 0..colony.len() {
                if colony[j].age >= 2
                    && colony[j].sex == bunny::Sex::Female
                    && colony[j].ghoul == false
                {
                    let mut temp: bunny::Sex = rand::random();
                    colony.push(bunny::Bunny {
                        sex: temp,
                        color: colony[j].color,
                        name: nameselector(&temp),
                        age: 0,
                        ghoul: false,
                    });
                    colony[colony.len()-1].announcebirth();
                }
            }
        }
    }

pub fn dosleep(time: u64) {
    thread::sleep(time::Duration::from_secs(time));
}


pub fn nameselector(element: &bunny::Sex) -> &'static str {
    let mut rng = rand::thread_rng();
    match element {
        bunny::Sex::Male => return &bunny::MALENAMES.choose(&mut rng).unwrap(),
        bunny::Sex::Female => return &bunny::FEMALENAMES.choose(&mut rng).unwrap(),
        _ => panic!("this is a terrible mistake!"),
    };
}

pub fn cull(colony: &mut Vec<bunny::Bunny>) {
    use rand::Rng;
    let mut cullsize: usize = colony.len()/2;
    for _victim in 0..=cullsize {
        let mut rng = rand::thread_rng();
        let kek = colony.remove(rng.gen_range(0, colony.len()));
    }
}

pub fn ispopulationdead(colony: &mut Vec<bunny::Bunny>) -> bool {
    if colony.is_empty() {
        println!("DEBUG: no colony alive");
        return true;
    } else {
        return false;
    }
}
