extern crate rand;
#[macro_use]
extern crate rand_derive;

mod game;

fn main()
{
	println!("///////////////GRADUATION IN RUST////////////////////");

	game::gameloop();

	println!("DEBUG: ENTERING MAIN FUNCTION");

	println!("PROGRAM TERMINATED");
}
