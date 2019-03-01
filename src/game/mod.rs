use std::{thread, time, vec::Vec};

use rand::seq::SliceRandom;

pub mod bunny;

pub fn gameloop() {
    let mut colony: Vec<bunny::Bunny> = Vec::new();
    let mut rng = rand::thread_rng();

    for x in 0..5 {
        colony.push(bunny::Bunny {
            sex: rand::random(),
            color: rand::random(),
            name: &bunny::MALENAMES.choose(&mut rng).unwrap(),
            age: 0,
            ghoul: rand::random(),
        });

        colony[x].announcebirth();
    }

    let mut turn: u64 = 0;

    loop {
        println!("Beginning of turn {}", &turn);
        dosleep(2);

        turn += 1;

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

        //breeding
        breed(&mut colony);

        //call infect if there are ghouls
        //infect()
    }
}

//figure out who is suitable for breeding and populate the referenced vector
fn breed(colony: &mut Vec<bunny::Bunny>) {
    let mut _rng = rand::thread_rng();

    //find 1 male who is atleast 2 years old and gather indexes of all females who are atleast 2 years old.
    //let suitablemale = colony.iter().find(|&x| x.age >= 2 && x.sex == Sex::Male );

    if colony
        .iter()
        .find(|&male| male.age >= 2 && male.sex == bunny::Sex::Male)
        != None
    {
        if colony
            .iter()
            .find(|&female| female.age >= 2 && female.sex == bunny::Sex::Female)
            != None
        {
            createnewborn();
        } else {
            println!("Debug: no suitable female found");
        }
    } else {
        //do nothing
        println!("DEBUG: no suitable male found");
    }
}

fn createnewborn() {

}

pub fn dosleep(time: u64) {
    thread::sleep(time::Duration::from_secs(time));
}
