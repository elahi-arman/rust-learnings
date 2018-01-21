use std::io;

fn main() {

    println!("You can type q to quit");

    loop {
        println!("Enter the temperature to convert as <Num>:");

        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        temperature = temperature.trim().to_ascii_lowercase();

        if temperature == "Q" || temperature == "q"{
            break;
        }

        let temperature: u32 = match temperature.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't parse that number");
                continue;
            }
        };

        println!("Convert to F or C?");
        let mut scale = String::new();

        io::stdin().read_line(&mut scale)
            .expect("Failed to read line");

        //NOTE: always need to trim the read_lines from stdin
        // the & converts from String -> &str and the [..] gets the full index
        match &scale[..] {
            "Q" => break,
            "F" => println!("{}", temperature * 9 / 5 + 32),
            "C" => println!("{}", temperature - 32 * 5 / 9),
            _ => {
                println!("Couldn't understand that scale");
                continue;
            }
        }
    }

}
