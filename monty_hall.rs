extern crate rand;

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct MontyHall<'a> {
    doors: &'a mut [u32; 3],
}

trait Player {
    fn new() -> Self;
    fn get_choice(&self) -> usize;
    fn make_choice(&mut self, available_doors: &Vec<usize>) -> usize;
}

#[derive(Debug)]
struct SwappingPlayer {
    current_choice: usize,
}

#[derive(Debug)]
struct NonSwappingPlayer {
    current_choice: usize,
}

impl Player for SwappingPlayer {
    fn new() -> Self {
        SwappingPlayer {
            current_choice: rand::thread_rng().gen_range(0, 3),
        }
    }

    fn get_choice(&self) -> usize {
        self.current_choice
    }

    fn make_choice(&mut self, available_doors: &Vec<usize>) -> usize {
        let result = available_doors
            .iter()
            .find(|&&door| (door != self.get_choice()));
        match result {
            Some(&i) => {
                self.current_choice = i;
                i
            }
            None => panic!(
                "SwappingPlayer unable to swap choices from available_doors {:?}",
                available_doors
            ),
        }
    }
}

impl Player for NonSwappingPlayer {
    fn new() -> Self {
        NonSwappingPlayer {
            current_choice: rand::thread_rng().gen_range(0, 3),
        }
    }

    fn get_choice(&self) -> usize {
        self.current_choice
    }

    fn make_choice(&mut self, _available_doors: &Vec<usize>) -> usize {
        self.current_choice
    }
}

impl MontyHall<'_> {
    fn generate_prizes(&mut self) {
        let car = rand::thread_rng().gen_range(0, 3);
        self.doors[car] = 1; // 1 is a car
        for x in self.doors.iter_mut() {
            if *x == 1 {
                continue;
            }
            *x = 2; // 2 is an unrevealed goat
        } // 3 is a revealed goat
    }

    fn reveal_door<T: Player>(&mut self, player: &mut T) {
        let choices: Vec<(usize, &u32)> = self
            .doors
            .iter()
            .enumerate()
            .filter(|(i, door)| (**door == 2 && *i != player.get_choice()))
            .collect();
        let revealed = choices.choose(&mut rand::thread_rng());
        match revealed {
            Some((i, _)) => self.doors[*i] = 3,
            None => panic!("No choices available!"),
        }
    }

    fn get_available_doors(&self) -> Vec<usize> {
        let choices: Vec<usize> = self
            .doors
            .iter()
            .enumerate()
            .filter(|(_, door)| (**door != 3))
            .map(|(i, _)| i)
            .collect();
        choices
    }

    fn player_wins<T: Player>(&self, player: &mut T) -> bool {
        player.make_choice(&self.get_available_doors());
        let prize = self.doors[player.get_choice()];
        prize == 1
    }
}

#[derive(Debug)]
struct TrialSet {
    num_success: usize,
    num_failure: usize,
}

fn run_trials<T: Player>(num_trials: usize) -> TrialSet {
    let mut trials = TrialSet {
        num_success: 0,
        num_failure: 0,
    };
    let mut current_trial = 0;

    while current_trial < num_trials {
        let mut player = T::new();
        let mut mhall = MontyHall { doors: &mut [0; 3] };
        mhall.generate_prizes();
        mhall.reveal_door(&mut player);
        if mhall.player_wins(&mut player){
            trials.num_success += 1;
        } else {
            trials.num_failure += 1;
        }
        current_trial += 1;
    }

    trials
}

fn main() {
    let results = run_trials::<SwappingPlayer>(10_000);
    println!("SwappingPlayer Results: {:?}", results);

    let results = run_trials::<NonSwappingPlayer>(10_000);
    println!("NonSwappingPlayer Results: {:?}", results);
}
