use std::io; // bring io library into scope so we can take input from the user

#[derive(Debug)] // so we can print the enum
enum Conversion {
    // enum to represent the two types of conversion
    FahrenheitToCelsius, // 1
    CelsiusToFahrenheit, // 2
}
fn main() {
    'temp: loop { //put the whole program in a loop so we can ask the user if they want to convert another temperature and gave it a label so we can break out of it
        let choice = loop { // put the first part of the program in a loop so we can ask the user to choose a conversion type and loop until they enter a valid choice
            println!("what temp do you want to convert?");
            println!("1. Fahrenheit to Celsius");
            println!("2. Celsius to Fahrenheit");
            let mut choice = String::new(); // create a new mutable string to store the user's choice

            io::stdin() // read the user's choice into the string with error handling
                .read_line(&mut choice) 
                .expect("Failed to read line"); 

            match choice.trim().parse() {
                // match the user's choice to the enum
                Ok(1) => break Conversion::FahrenheitToCelsius, // if the user enters 1, return the first enum
                Ok(2) => break Conversion::CelsiusToFahrenheit, // if the user enters 2, return the second enum
                _ =>
                // if the user enters anything else, return an error
                {
                    println!("Invalid choice, please enter 1 or 2")
                }
            }
        };

        println!("You want to convert {:?}", choice);


        let temp: f64; // create a variable to store the user's temperature
        loop { // put the second part of the program in a loop so we can ask the user to enter a temperature and loop until they enter a valid temperature
            let mut temp_str = String::new(); // create a new mutable string to store the user's temperature
            println!("Enter the temperature");
            io::stdin()
                .read_line(&mut temp_str)
                .expect("Failed to read line"); // read the user's temperature into the string with error handling

            match temp_str.trim().parse() {
                // parse the user's temperature into a float so we can get rid of the newline character and or whitespace
                Ok(num) => {
                    temp = num;
                    break;
                } // if the user enters a valid number, return the number
                Err(_) => {
                    // if the user enters anything else, return an error
                    println!("Invalid temperature. Please enter a number");
                }
            }
        }

        let (result, unit) = match choice { 
            // match the user's choice to the enum
            Conversion::FahrenheitToCelsius => ((temp - 32.0) * 5.0 / 9.0, "C"),// if the user enters 1, convert the temperature to celsius
            Conversion::CelsiusToFahrenheit => ((temp * 9.0 / 5.0) + 32.0, "F"), // if the user enters 2, convert the temperature to fahrenheit
        };

        println!("The result is {}° {}", result, unit); // print the result

        println!("Do you want to convert another temperature? (yes/no)"); // ask the user if they want to convert another temperature

        loop { // put the third part of the program in a loop so we can ask the user if they want to convert another temperature and loop until they enter a valid answer
            let mut again = String::new(); // create a new mutable string to store the user's answer
            io::stdin()
                .read_line(&mut again)
                .expect("Failed to read line"); // read the user's answer into the string with error handling

            match again.trim().to_lowercase().as_str() {
                "yes" => {
                    break;
                }
                "no" => {
                    println!("See you later!");
                    break 'temp;
                }
                _ => {
                    println!("Invalid answer, please enter yes or no");
                }
            }
        }
    }
}
