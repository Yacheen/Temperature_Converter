use std::io;
fn main() {
    
    // get converter choice
    let mut choice = String::new();
    println!("What converter would you like to use?");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    
    let choice: String = match choice.trim().parse() {
        Ok(guess) => guess,
        Err(_) => "ERROR".to_string(),
    };

    //get number choice
    println!("What number do you wish to be converted?");
    let mut number_choice = String::new();

    io::stdin()
        .read_line(&mut number_choice)
        .expect("Failed to read line");

    let number_choice:String = match number_choice.trim().parse() {
        Ok(doesntmatterwhaticallthis) => doesntmatterwhaticallthis,
        Err(_) => "ERROR".to_string(),
    };

    if choice == "fahrenheit".to_string() {
        println!("{} converted to {} is {:.2}", number_choice, choice, convert_to_fahrenheit(number_choice.parse::<f64>().unwrap()));
    }

    else if choice == "celcius".to_string() {
        println!("{} converted to {} is {:.2}", number_choice, choice, convert_to_celcius(number_choice.parse::<f64>().unwrap()));
    }
    else {
        println!("That was either not a correct choice or number. please try again. closing program...");
    }

}

fn convert_to_fahrenheit(temperature:f64) -> f64 {
    return temperature * 9.0/5.0 + 32.0
}

fn convert_to_celcius(temperature:f64) -> f64 {
    return (temperature - 32.0) * 5.0/9.0
}