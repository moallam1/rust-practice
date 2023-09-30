use std::io; // bring io library into scope so we can take input from the user

#[derive(Debug)] // so we can print the enum
enum Conversion { // enum to represent the two types of conversion
    FahrenheitToCelsius, // 1
    CelsiusToFahrenheit, // 2
}
fn main() {
    println!("what temp do you want to convert?");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    let mut choice = String::new(); // create a new mutable string to store the user's choice

    io::stdin().read_line(&mut choice).expect("Failed to read line"); // read the user's choice into the string with error handling

    

    let choice = match choice.trim().parse() { // match the user's choice to the enum
        Ok(1) => Conversion::FahrenheitToCelsius, // if the user enters 1, return the first enum
        Ok(2) => Conversion::CelsiusToFahrenheit, // if the user enters 2, return the second enum
        _ => { // if the user enters anything else, return an error
            println!("Invalid choice");
            return;
        },
        
    };

    println!("You want to convert {:?}", choice);

    println!("Enter the temperature");

    let mut temp = String::new(); // create a new mutable string to store the user's temperature

    io::stdin().read_line(&mut temp).expect("Failed to read line"); // read the user's temperature into the string with error handling

    println!("You want to convert {} degrees", temp);

    let temp: f64 = match temp.trim().parse() { // parse the user's temperature into a float so we can get rid of the newline character and or whitespace
        Ok(num ) => num, // if the user enters a valid number, return the number
        Err(_) => { // if the user enters anything else, return an error
            println!("Invalid temperature");
            return;
        },
    };

    let result = match choice { // match the user's choice to the enum
        Conversion::FahrenheitToCelsius => (temp - 32.0) * 5.0 / 9.0, // if the user enters 1, convert the temperature to celsius
        Conversion::CelsiusToFahrenheit => (temp * 9.0 / 5.0) + 32.0, // if the user enters 2, convert the temperature to fahrenheit
    };

    println!("The result is {}", result); // print the result
}