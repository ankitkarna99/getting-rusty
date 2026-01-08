use std::{fs, result};
use std::io::Error;

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    
    let mut errors = Vec::new();

    for line in split_text {
        if line.starts_with("ERROR") {
            errors.push(line);
        }
    }

    return errors;
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(&text);
    fs::write("errors.txt", error_logs.join("\n"))?;

    println!("{}", text);

    return Ok(());


    // let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
    // let error_logs = extract_errors(&text);
    // fs::write("errors.txt", error_logs.join("\n")).expect("Failed to write errors.txt");

    // match divide(5.0, 0.0) {
    //     Ok(result) => {
    //         println!("Result: {}", result);
    //     },
    //     Err(error) => {
    //         println!("Error: {}", error)
    //     } 
    // }

    // match validate_email(String::from("email@example.com")) {
    //     Ok(_) => {
    //         println!("Email is valid");
    //     },
    //     Err(error) => {
    //         println!("Error: {}", error);
    //     }
    // }


}

// fn validate_email(email: String) -> Result<(), Error> {
//     if email.contains("@") {
//         return Ok(());
//     }

//     return Err(Error::other("Invalid email"));
// }



// fn divide(a: f64, b: f64) -> Result<f64, Error> {
//     if b == 0.0 {
//         return Err(Error::other("Cannot divide by zero")); 
//     }

//     return Ok(a/b);
// } 