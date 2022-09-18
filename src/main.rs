use std::io; //import the whole shebang?
use std::cmp::Ordering;
use rand::Rng; //import a specific trait.

//mod double;
pub use crate::double;


struct Fir16 {
    taps: [i64; 16],
    last_data: [i64; 15],
}

impl Fir16 {
    //Pull sample, run FIR on it and 15 prev samples, return result
    fn step(&mut self, sample: i64) -> i64 {
        let mut output = self.taps[0] * sample;
        for idx in 1..15 {
            output += self.taps[idx] * self.last_data[idx];
        }
        self.last_data[0] = sample;
        for idx in 0..14 {
            self.last_data[idx+1] = self.last_data[idx];
        }
        output
    }

    fn give_fir() -> Self {
        Self {
            taps:      [1,1,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
            last_data: [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0],
        }
    }

}

fn main() {
    let mut doubleLast = double::doubleLast;

    let A = doubleLast.step(2);
    let B = doubleLast.step(3);
    let C = doubleLast.step(4);

    println!("Our result is: {:?}", A);
    println!("Our result is: {:?}", B);
    println!("Our result is: {:?}", C);

    let mut data = [0,1,0,0,1,2,0,0];
    for idx in 0..data.len() {
        data[idx] = my_fir.step(data[idx]);

    }
    println!("Our result is: {:?}", data);

}

fn guess_done_with() {
    let mut stop = false;
    let sn = rand::thread_rng().gen_range(1..=1024); //..= is range expression

    while !stop
    {
        let mut x = String::new();

        io::stdin()
            .read_line(&mut x)
            .expect("Wrong Format???");

        let xn: u32 = match x.trim().parse()
        {
            Ok(num) => num,
            Err(_) => { println!("({x} isn't a valid number)", x=x); 0},
        };

        match xn
        {
            1 => println!("You guessed (v normie): {}", xn),
            16 | 64 | 1024  => println!("You guessed (v round): {xn}", xn=xn),
            _ => println!("You guessed (snowflake): {}", xn),
        }

        println!("You guessed: {xn}", xn=xn);
        match xn.cmp(&sn)
        {
        Ordering::Less => println!("Less...: {xn}", xn=xn),
        Ordering::Greater => println!("Greater: {xn}", xn=xn),
        Ordering::Equal => { println!("Yes!! ({xn} is right)", xn=xn); stop = true;},
        }
    }
}
