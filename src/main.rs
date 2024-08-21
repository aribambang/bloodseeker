use std::{fs, io::Error, time::Instant};

fn extract_errors (text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let start = Instant::now();
    
    let text = fs::read_to_string("logs.txt")?; // try operator
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    // let text = fs::read_to_string("logs.txt").expect("Failed to read file logs.txt");
    // let error_logs = extract_errors(text.as_str());
    // fs::write("errors.txt", error_logs.join("\n")).expect("Writing of errors.txt failed");
    

    // match fs::read_to_string("logs.txt") {
    //     Ok(text_that_was_read) => {
    //         let error_logs = extract_errors(text_that_was_read.as_str());
            
    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => {
    //                 println!("Wrote errors.txt");
    //             }
    //             Err(reason_write_failed) => {
    //                 println!("Writing of errors.txt failed: {}", reason_write_failed);
    //             }
    //         }
    //     }
    //     Err(why_this_failed) => {
    //         println!("{}", why_this_failed);
    //     }
    // }


    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
    Ok(())
}