extern crate rand;

use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct MontyHall<'a> {
    doors: &'a mut [u32; 3],
}

trait Player {
    fn get_choice(&self)->usize;
    fn make_choice(&mut self, available_doors: &Vec<usize>)->usize;
}

#[derive(Debug)]
struct SwappingPlayer {
    current_choice: usize,
}

impl Player for SwappingPlayer {
    fn get_choice(&self)->usize {
        self.current_choice
    }

    fn make_choice(&mut self, available_doors: &Vec<usize>)->usize {
        let result = available_doors.iter()
        .find(|&&door| (door != self.get_choice()));
        match result {
            Some(&i) => {
                self.current_choice = i;
                i
            },
            None => panic!("SwappingPlayer unable to swap choices from available_doors {:?}", available_doors),
        }
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
        let choices: Vec<(usize, &u32)> = self.doors.iter().enumerate()
            .filter(|(i, door)| (**door == 2 && *i != player.get_choice()))
            .collect();
        println!("Choices: {:?}", choices);
        let revealed = choices.choose(&mut rand::thread_rng());
        match revealed {
            Some((i, _)) => self.doors[*i] = 3,
            None => panic!("No choices available!")
        }
    }

    fn get_available_doors(&self) -> Vec<usize> {
        let choices: Vec<usize> = self.doors.iter().enumerate()
            .filter(|(_, door)| (**door != 3 ) )
            .map(|(i, _)| i)
            .collect();
        choices
    }

    fn player_wins(&self, player: &mut dyn Player) -> bool {
        player.make_choice(&self.get_available_doors());
        let prize = self.doors[player.get_choice()];
        
        prize == 1
    }
}

fn main(){
    let mut mhall = MontyHall { doors: &mut[0; 3]};
    let player = SwappingPlayer { current_choice: rand::thread_rng().gen_range(0,3)};
    println!("Player: {:?}", player);
    mhall.generate_prizes();
    println!("MontyHall base: {:?}", mhall);
    mhall.reveal_door(&player);
    println!("MontyHall reveal: {:?}", mhall);
    println!("Player wins? {:?}", mhall.player_wins(&mut player));
}