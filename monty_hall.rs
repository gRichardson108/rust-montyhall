extern crate rand;

use rand::Rng;

#[derive(Debug)]
struct MontyHall<'a> {
    doors: &'a mut [i32; 3],
}

impl MontyHall<'_> {
    pub fn generate_prizes(&mut self){
        let car = rand::thread_rng().gen_range(0,3);
        self.doors[car] = 1;
        // for mut x in self.doors {
        //     if x == 1 {
        //         continue;
        //     }
        //     x = 2;
        // }
    }
}

fn main(){
    let mut mhall = MontyHall { doors: &mut[0; 3]};
    mhall.generate_prizes();
    println!("MontyHall: {:?}", mhall);
}