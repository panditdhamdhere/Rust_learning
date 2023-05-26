// guessing game
use rand ::{thread_rng,Rng}

fn main() {
    println!("Welcome to the guessing game")
let secret_number = rand::thread_rngO().gen_range(1..101); 
println!("Secret Number is {}:", secret_number)


}