extern crate rand;

use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct MontyHall<'a> {
    doors: &'a mut [u32; 3],
}

trait Player {
    fn get_choice(&self)->u32;
}

#[derive(Debug)]
struct SwappingPlayer {
    currentChoice: u32,
}

impl Player for SwappingPlayer {
    fn get_choice(&self)->u32 {
        self.currentChoice
    }
}

impl MontyHall<'_> {
    fn generate_prizes(&mut self) {
        let car = rand::thread_rng().gen_range(0,3);
        self.doors[car] = 1; // 1 is a car
        for x in self.doors.iter_mut() {
            if *x == 1 {
                continue;
            }
            *x = 2; // 2 is an unrevealed goat
        } // 3 is a revealed goat
    }

    fn reveal_door(&mut self, player: &dyn Player) {
        let choices: Vec<&u32> = self.doors.iter()
            .filter(|door| (**door == 2 && **door != player.get_choice()))
            .collect();
        let revealed = choices.choose(&mut rand::thread_rng());
        match revealed {
            Some(x) => self.doors[**x as usize] = 3,
            None => panic!("No choices available!")
        }
    }
}

fn main(){
    let mut mhall = MontyHall { doors: &mut[0; 3]};
    let player = SwappingPlayer { currentChoice: rand::thread_rng().gen_range(0,3)};
    println!("Player: {:?}", player);
    mhall.generate_prizes();
    println!("MontyHall base: {:?}", mhall);
    mhall.reveal_door(&player);
    println!("MontyHall reveal: {:?}", mhall);
}