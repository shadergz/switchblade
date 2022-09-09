use sha2::{Digest, Sha256};

use std::io::{self, Write};

// Dev note: Use process::exit() to exit from the current process
// use std::process;
// Display the byte array representation from the vector content

fn display_bytes(bytes: &Vec<u8>) {
    print!("Bytes array: ");
    for (_, byte) in bytes.iter().enumerate() {
        print!("\\{:#02x}", byte);
    }
    println!();
}

fn display_sha2(bytes: &Vec<u8>) {
    let mut sha_obj = Sha256::new();
    sha_obj.update(bytes);
    let digest = sha_obj.finalize();
    print!("SHA-256 digest: ");
    for value in digest {
        print!("{:x}", value);
    }
    println!();
}

fn display_hex(bytes: &Vec<u8>) {
    print!("Hex array: ");
    for (_, byte) in bytes.iter().enumerate() {
        print!("{:#02x}", byte);
    }
    println!();
}

fn display_octal(bytes: &Vec<u8>) {
    print!("Octal array: ");
    for (_, byte) in bytes.iter().enumerate() {
        print!("{:#o}", byte);
    }
    println!();
}

// Display a C array from an input vector
fn display_c_array(bytes: &Vec<u8>) {
    let array_len = bytes.len();
    print!("C Array: unsigned char c_array = {{");
    for (pos, byte) in bytes.iter().enumerate() {
        if pos + 1 < array_len {
            print!("{:#02x}, ", byte);
        } else {
            print!("{:#02x}", byte);
        }
    }
    println!("}};");
}

fn display_string(string_data: &str) {
    println!("UTF-8 Representation: {}", string_data.to_string());
}

fn display_check(string_data: &str) {
    print!("Check ascii: ");
    if string_data.is_ascii() {
        println!("is ascii");
    } else {
        println!("isn't ascii")
    }
}

// Try to parser the string content to a integer number
fn str_to_i32(string_data: &str) -> i32 {
    let result = string_data.parse::<i32>();
    let number = match result {
        Ok(n) => n,
        Err(_e) => 0,
    };
    return number;
}

fn display_integer(number: i32) {
    println!("Integer: i32({})", number);
    println!("Absolute value: i32({})", number.abs());
    println!("Log10^{} = {}", f64::from(number).log10(), number);
    println!("Scientific notation: {:#e}", number);
}

fn display_ipv4(number: i32) {
    println!(
        "IPv4: {:X}.{:X}.{:X}.{:X}",
        number >> 24 & 0xff,
        number >> 16 & 0xff,
        number >> 8 & 0xff,
        number & 0xff
    );
}

fn display_rgb(number: i32) {
    println!(
        "RGB: #{:X}{:X}{:X} - ({0}, {1}, {2})",
        number >> 16 & 0xff,
        number >> 8 & 0xff,
        number & 0xff
    );
}

fn display_info(content_string: &mut String) {
    // content_string = "NULL".to_string();
    let raw_string = content_string.as_str();
    let content_bytes = raw_string.as_bytes();
    let bytes_array = content_bytes.to_vec();
    let number = str_to_i32(&raw_string);

    display_string(&raw_string);
    display_check(&raw_string);
    display_bytes(&bytes_array);
    display_hex(&bytes_array);
    display_octal(&bytes_array);
    display_sha2(&bytes_array);
    display_c_array(&bytes_array);

    if number > 0 {
        display_integer(number);
        display_ipv4(number);
        display_rgb(number);
    }
}

fn main() -> io::Result<()> {
    let mut sblade_command = String::new();

    loop {
        print!("sblade> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut sblade_command)
            .expect("Couldn't read from standard input");

        // Truncate the '\n' character
        sblade_command.truncate(sblade_command.len() - 1);

        match &sblade_command as &str {
            "exit" | "quit" => break,
            "" => println!("Hey! make this easy"),
            _ => {
                display_info(&mut sblade_command);
            }
        }

        /* Clear the string
            Necessary doesn't nothing more that a truncate into length 0, the string by itself
            isn't deallocated or something else
        */
        sblade_command.clear();
    }

    Ok(())
}
