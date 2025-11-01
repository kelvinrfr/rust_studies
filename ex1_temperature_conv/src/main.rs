use std::io;

fn main() {
    println!("Welcome to the Temperature converter.");
    
    loop {
        println!("Select the source temperature: (C/F)");

        let mut temperature_type= String::new();
        io::stdin()
            .read_line(&mut temperature_type)
            .expect("Failed to read line");
        let temperature_type = temperature_type.trim();

        let mut temperature_value = String::new();
        io::stdin()
            .read_line(&mut temperature_value)
            .expect("Failed to read line");
        let temperature_value = temperature_value.trim();

        let temperature_value: f32 = match temperature_value.parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Error converting temperature to number. Try again.");
                continue;
            }
        };

        if temperature_type == "C" {
            let fahrenheit_value = ((temperature_value * 9.0) / 5.0) + 32.0;
            println!("Celsius to Fareheint: {fahrenheit_value}");
        }
        else if temperature_type == "F" {
            let celsius_value = ((temperature_value - 32.0) * 5.0) / 9.0;
            println!("Celsius to Fareheint: {celsius_value}");
        }
        else {
            println!("Invalid temperature type. Try again!");
            continue;
        }

        break;
    }
}
