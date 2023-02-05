use std::io;

fn main() {
    let (unit_from, unit_to) = get_unit_choices();
    let temp_from = get_temp_input();

    let converted_temp = convert_temp(temp_from, unit_from, unit_to);

    println!("{temp_from}{unit_from} = {converted_temp}{unit_to}");
}

fn get_unit_choices() -> (char, char) {
    let unit_from;
    let unit_to;

    loop {
        println!("Please input the unit you want to convert from (F/C):");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        unit_from = match user_input.trim().parse() {
            Ok(char) => match char {
                'C' => char,
                'F' => char,
                _ => continue,
            },
            Err(_) => continue,
        };

        break;
    }

    loop {
        println!("Please input the unit you want to convert to (F/C):");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        unit_to = match user_input.trim().parse() {
            Ok(char) => match char {
                'C' => char,
                'F' => char,
                _ => continue,
            },
            Err(_) => continue,
        };

        break;
    }

    return (unit_from, unit_to);
}

fn get_temp_input() -> f64 {
    let temperature;

    loop {
        println!("Please input the temperature you want to convert from:");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        temperature = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    return temperature;
}

fn convert_temp(temp_from: f64, unit_from: char, unit_to: char) -> f64 {
    if unit_from == unit_to {
        return temp_from;
    }

    if unit_from == 'C' && unit_to == 'F' {
        return convert_to_fahrenheit(temp_from);
    }

    return convert_to_celsius(temp_from);
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * (9.0 / 5.0) + 32.0;
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * (5.0 / 9.0);
}
