use std::{thread, time, vec::Vec};

use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    Rng,
};
pub mod bunny;

pub const MAXPOPULATION: usize = 1000;

pub fn gameloop() {
    let mut colony: Vec<bunny::Bunny> = Vec::new();

    for x in 0..5 {
        let temp: bunny::Sex = rand::random();
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
        if ispopulationdead(&mut colony) {
            break;
        }

        //iterate and increment age.
        for x in colony.iter_mut() {
            x.incrementage();
        }

        // retain all those bunnies in the vector who should not die, drop those who should.
        colony.retain(|i| i.shoulddie() == false);
        println!(
            "DEBUG: post-age retain population is {:?}",
            colony.len().to_string()
        );
        dosleep(2);

        if ispopulationdead(&mut colony) {
            break;
        }

        //breeding
        breed(&mut colony);

        println!(
            "DEBUG: post breed population is {:?}",
            colony.len().to_string()
        );
        dosleep(2);

        if colony.len() >= MAXPOPULATION {
            cull(&mut colony);
            println!(
                "DEBUG: cull survivor population is {:?}",
                colony.len().to_string()
            );
            dosleep(2);
        }

        //call infect if there are ghouls
        if colony.iter().find(|&x| x.ghoul == false) != None {
            infect(&mut colony);
        }
    }
}

//figure out who is suitable for breeding and populate the referenced vector
fn breed(colony: &mut Vec<bunny::Bunny>) {

    //find 1 male who is atleast 2 years old and gather indexes of all females who are atleast 2 years old.
    //let suitablemale = colony.iter().find(|&x| x.age >= 2 && x.sex == Sex::Male && x.ghoul == false);

    if colony
        .iter()
        .find(|&x| x.age >= 2 && x.sex == bunny::Sex::Male && x.ghoul == false)
        != None
    {
        for j in 0..colony.len() {
            if colony[j].age >= 2 && colony[j].sex == bunny::Sex::Female && colony[j].ghoul == false
            {
                let temp: bunny::Sex = rand::random();
                colony.push(bunny::Bunny {
                    sex: temp,
                    color: colony[j].color,
                    name: nameselector(&temp),
                    age: 0,
                    ghoul: ghoulchance(),
                });
                colony[colony.len() - 1].announcebirth();
            }
        }
    }
}

pub fn dosleep(time: u64) {
    thread::sleep(time::Duration::from_secs(time));
}

fn nameselector(element: &bunny::Sex) -> &'static str {
    let mut rng = rand::thread_rng();
    match element {
        bunny::Sex::Male => return &bunny::MALENAMES.choose(&mut rng).unwrap(),
        bunny::Sex::Female => return &bunny::FEMALENAMES.choose(&mut rng).unwrap(),
    };
}

fn cull(colony: &mut Vec<bunny::Bunny>) {
    let cullsize: usize = colony.len() / 2;
    for _victim in 0..=cullsize {
        let mut rng = rand::thread_rng();
        colony.remove(rng.gen_range(0, colony.len()));
    }
}
//let rng: usize = rand::thread_rng().gen_range(0, colony.len()-1);
fn infect(colony: &mut Vec<bunny::Bunny>) {
    //get ghoulcount
    let ghoulcount = colony.iter().filter(|&n| n.ghoul == true).count();
    if ghoulcount == 0 {
        return;
    }
    println!("INFECTING {:?} BUNNIES", ghoulcount);
    let mut ghoulsmade = 0;

    if ghoulcount < colony.len() / 2 {
        while ghoulsmade != ghoulcount {
            //infect
            let rng: usize = rand::thread_rng().gen_range(0, colony.len() - 1);
            if colony[rng].ghoul == false {
                colony[rng].becomeinfected();
            }
            //
            ghoulsmade += 1;
        }
    } else {
        for bunny in colony.iter_mut() {
            bunny.becomeinfected();
        }
    }
}

fn ghoulchance() -> bool {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..50);
    let throw = die.sample(&mut rng);
    if throw == 1 {
        return true;
    } else {
        return false;
    }
}

fn ispopulationdead(colony: &mut Vec<bunny::Bunny>) -> bool {
    if colony.is_empty() {
        println!("DEBUG: no colony alive");
        return true;
    } else {
        return false;
    }
}
