use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the slot machine");
    println!("Type 'Go' if you want to gamble once or 'GambleLoop' if you want to gamble until you win");
        let mut playerinput = String::new(); 
        io::stdin()
            .read_line(&mut playerinput)
            .expect("Failed to read line");
        playerinput = playerinput.trim().to_string().to_lowercase();
        if playerinput == "go" {
            println!("Success!");
            gamble();
            return;
    }
    if playerinput == "gambleloop" {
        println!("Slot machine results!");
        gambleloop() 
    }
        else {
            loop {
            println!("");
                playerinput.clear();
                    io::stdin()
                        .read_line(&mut playerinput)
                        .expect("Failed to read line");
                    playerinput = playerinput.trim().to_string().to_lowercase();
                if playerinput == "go" {
            println!("Slot machine results!");
            gamble();
            break; }
                    if playerinput == "gambleloop" {
                        println!("Success!");
                        gambleloop();
                        break;
                    }
        }
    }
    return;

}

fn gambleloop() {
    loop {
    let secret_number1 = rand::thread_rng().gen_range(1..=6);
    let secret_number2 = rand::thread_rng().gen_range(1..=6);
    let secret_number3 = rand::thread_rng().gen_range(1..=6);
    println!("The first number: {}", secret_number1);
    println!("The second number: {}", secret_number2);
    println!("The third number: {}", secret_number3);
    
    if secret_number1 == secret_number2 && secret_number1 != secret_number3 {
        println!("Slot 1 and 2 were the same but slot 3 was different.");
    }
    if secret_number1 == secret_number3 && secret_number1 != secret_number2 {
        println!("slot 1 and 3 were the same but slot 3 was different.")
    }
    if secret_number2 == secret_number3 && secret_number2 != secret_number1 {
        println!("slot 2 and 3 were the same but slot 1 was different.")
    }
    if secret_number1 != secret_number2 && secret_number1 != secret_number2 && secret_number2 != secret_number3 {
        println!("You lost, none of the numbers were the same")
    }
    if secret_number1 == secret_number2 && secret_number2 == secret_number3 && secret_number3 == secret_number3 {
        println!("All 3 slots were the same! YOU WON!!!");
        break;
    }
}
}

fn gamble() {
    let secret_number1 = rand::thread_rng().gen_range(1..=6);
    let secret_number2 = rand::thread_rng().gen_range(1..=6);
    let secret_number3 = rand::thread_rng().gen_range(1..=6);
    println!("The first number: {}", secret_number1);
    println!("The second number: {}", secret_number2);
    println!("The third number: {}", secret_number3);
    
    if secret_number1 == secret_number2 && secret_number1 != secret_number3 {
        println!("Slot 1 and 2 were the same but slot 3 was different.");
    }
    if secret_number1 == secret_number3 && secret_number1 != secret_number2 {
        println!("slot 1 and 3 were the same but slot 3 was different.")
    }
    if secret_number2 == secret_number3 && secret_number2 != secret_number1 {
        println!("slot 2 and 3 were the same but slot 1 was different.")
    }
    if secret_number1 != secret_number2 && secret_number1 != secret_number2 && secret_number2 != secret_number3 {
        println!("You lost, none of the numbers were the same")
    }
    if secret_number1 == secret_number2 && secret_number2 == secret_number3 && secret_number3 == secret_number3 {
        println!("All 3 slots were the same! YOU WON!!!");
    }
}
