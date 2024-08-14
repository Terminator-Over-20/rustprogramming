use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::OpenOptions;
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::io::Error;
#[allow(unused_imports)]
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write},
};
use ureq;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestPayload {
    messages: Vec<Message>,
    temperature: f32,
    top_p: f32,
    max_tokens: u32,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct ResponsePayload {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// Total amount of lines in a file
#[allow(unused)]
fn file_lines(name: &str) -> io::Result<i32> {
    let mut count = 0;
    for line in BufReader::new(File::open(&name)?).lines() {
        line?;
        count += 1;
    }
    Ok(count as i32)
}
// Slice from a file
#[allow(unused)]
fn file_slice(name: &str) -> String {
    let mut slice = String::new();
    let mut start = String::new();
    let mut end = String::new();

    if let Ok(fsize) = file_lines(&name) {
        let file = File::open(&name).unwrap(); // opening the file
        let reader = BufReader::new(file); // bufffer to iterate the file
                                           //let mut count: i32 = 0; // i32 count for the For loop in the match statement
                                           // let mut file_index: i32 = 1; // Keeps track of the index in the file before the Start number given

        loop {
            start.clear();
            end.clear();

            println!("Please the index from the file you want to Start");
            io::stdin()
                .read_line(&mut start)
                .expect("failed to read line"); // Start Number  from input
            println!("Please the index from the file you want to End");
            io::stdin()
                .read_line(&mut end)
                .expect("failed to read line"); // End number from input

            let mut usize_start = start.trim();
            let mut usize_end = end.trim();
            let mut count: i32 = 0; // i32 count for the For loop in the match statement
            let mut file_index: i32 = 0; // Keeps track of the index in the file before the Start number given

            match usize_start.parse::<i32>() {
                // Converting the number input into i32 after trim
                Ok(st) => match usize_end.parse::<i32>() {
                    // Converting the number input into i32 after trim
                    Ok(en) => {
                        if st <= fsize && st != 0 && en != 0 && en <= fsize {
                            file_index = st - 1;
                            count = (en - st) + 1; // count for how many lines to read before entering the Else-if statement
                            break for line in reader.lines().skip(file_index as usize) {
                                // while the file_index is not equal to start then skip the line till one before the start value
                                // if statement to make sure you are sending back the lines of code
                                if count != 0 {
                                    let fline: &String =
                                        &line.expect("No Lines Where Read From The File");
                                    
                                    //fsize works , fline works//
                                    slice.push_str(fline);
                                    count -= 1;
                                }
                            };
                        }
                        //
                        else {
                            if st > fsize {
                                println!("Please check your Starting number");
                                file_index += 1;
                            } else if en > fsize {
                                file_index += 1;
                                println!("Please check your Ending number");
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error:: {:?}, Please input a valid Ending number", e);
                    }
                }, // match end bracket

                Err(e) => {
                    println!("Error:: {:?}, Please input a valid Starting number", e);
                }
            } // match end bracket
        }
    }
    slice
}

//Open File
#[allow(unused)]
fn open_file() -> String {
    let mut file_name = String::new();
    loop {
        file_name.clear();
        print!("What is the name of the file you want me to open? (Please Type File Name and Press Enter Twice");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut file_name)
            .expect("failed to read line");
        let name = file_name.trim().to_string();
        let file_result = File::open(name);
        //let file_result: Result<File,std::io::Error> = Ok(file);
        match file_result {
            Ok(file) => break println!("File opened successfully: {:?}", file),

            Err(err) => println!("Error opening file: {}", err),
        }
    }
    // Open File -> total File or Slice
    file_name = file_name.trim().to_string();
    let mut file_content = String::new();
    let mut user_input = String::new();
    loop {
        println!("Do you want to pass the whole file or just a slice, Type ::  Type File or Slice and Press Enter Twice");
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line");
        match user_input.trim().parse::<String>() {
            Ok(uf) => {
                if uf == "File" || uf == "file" {
                    let mut data_file = File::open(&file_name).unwrap();
                    data_file.read_to_string(&mut file_content).unwrap();
                    break file_content;
                } else if "Slice" == uf || "slice" == uf {
                    break file_slice(&file_name);
                } else {
                    println!("Please try again");
                }
            }

            Err(_) => println!(
                "Please type the File for whole contents or Slice for a slice from your file"
            ),
        }
        user_input.clear();
    }
}
fn call_user() -> String {
    let mut user_input = String::new();
    loop {
        println!("Write the code you want to input and Press Enter Twice");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        match user_input.trim().parse::<String>() {
            Ok(user_input) => break user_input,
            Err(e) => println!("Error:: {:}, Please try again", e),
        }
    }
}

fn get_user_input() -> String {
    let mut user_input = String::new();

    loop {
        println!("Do you want to open a File or want to write an Input? Type:: File or Input and Press Enter Twice");
        // read user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        match user_input.trim().parse::<String>() {
            Ok(uf) => {
                if uf == "File" || uf == "file" {
                    // Comparing to the words file
                    break open_file(); // Call open_file , returns String
                } else if "Input" == uf || "input" == uf {
                    // Comparing to the words input
                    break call_user(); // Call user function , returns String
                } else {
                    println!("Please try again");
                }
            }
            Err(_) => println!("Error"),
        }
        user_input.clear();
    }
}

fn get_user_input_2(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        input.push_str(&line);
    }

    input.trim().to_string()
}


fn generate_prompt(user_question: &str, language: &str, option: &str) -> String {
    match option {
        "1" => format!("You are a master of code completion, look into user question: {}. Try to help him as much as possible to complete the code, he prefers the answer in language {}.", user_question, language),
        "2" => format!("You are a master of code explanation, look into user question: {}. The code is written in {}, try to help him as much as possible by explaining the code given in simple words.", user_question, language),
        "3" => format!("You are a master of code refactoring suggestions, look into user question: {}, try to help him as much as possible by giving him code refactoring suggestions. The code is written in {}.", user_question, language),
        _ => String::from("Invalid option provided."),
    }
}
fn file_code_part() {
    let lang = get_user_input_2("Please enter the coding language and press Enter Twice: ");
    println!("\n");

    let prompt_input = get_user_input();

    if let Err(err) = language_model(&prompt_input, &lang, "1") {
        eprintln!("Error: {}", err);
    }
}
fn language_model(
    prompt_input: &str,
    language: &str,
    option: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the API endpoint and API key from environment variables
    let api_endpoint = env::var("API_ENDPOINT").expect("API_ENDPOINT not set in .env file");
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env file");

    let generated_prompt = generate_prompt(prompt_input, language, option);

    // Construct the request payload
    let payload = RequestPayload {
        messages: vec![Message {
            role: "user".to_string(),
            content: generated_prompt.clone(),
        }],
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 800,
    };

    // Send the POST request using ureq
    #[allow(unused)]
    let response = ureq::post(&api_endpoint)
        .set("Content-Type", "application/json")
        .set("api-key", &api_key)
        .send_json(&payload)?;


    // Send the request again to parse the JSON
    let response = ureq::post(&api_endpoint)
        .set("Content-Type", "application/json")
        .set("api-key", &api_key)
        .send_json(&payload)?;

    // Handle the response
    let response_from_server = match response.into_json::<ResponsePayload>() {
        Ok(response_payload) => Ok(response_payload),
        Err(e) => Err(e),
    };
    let text = match response_from_server {
        Ok(r) => r.choices[0].message.content.clone(),
        Err(_) => panic!(),
    };
    println!("{}", text);

    Ok(())
}
fn language_model_2(
    prompt_input: &str,
    language: &str,
    option: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n");
    dotenv().ok();

    let api_endpoint = env::var("API_ENDPOINT")?;
    let api_key = env::var("API_KEY")?;

    let generated_prompt = generate_prompt(prompt_input, language, option);

    let payload = RequestPayload {
        messages: vec![Message {
            role: "user".to_string(),
            content: generated_prompt.clone(),
        }],
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 800,
    };

    let response = ureq::post(&api_endpoint)
        .set("Content-Type", "application/json")
        .set("api-key", &api_key)
        .send_json(&payload)?;

    let response_from_server: ResponsePayload = response.into_json()?;
    let text = &response_from_server.choices[0].message.content;

    println!("{}", text);

    // Open the chat history file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("chat_history")?;

    // Write the prompt and response to the file
    writeln!(file, "Prompt: {}\n", generated_prompt)?;
    writeln!(file, "Response: {}\n", text)?;

    Ok(())
}

fn language_model_2_3(
    prompt_input: &str,
    language: &str,
    option: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n");
    dotenv().ok();

    let api_endpoint = env::var("API_ENDPOINT")?;
    let api_key = env::var("API_KEY")?;

    let generated_prompt = generate_prompt(prompt_input, language, option);

    let payload = RequestPayload {
        messages: vec![Message {
            role: "user".to_string(),
            content: generated_prompt.clone(),
        }],
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 800,
    };

    let response = ureq::post(&api_endpoint)
        .set("Content-Type", "application/json")
        .set("api-key", &api_key)
        .send_json(&payload)?;

    let response_from_server: ResponsePayload = response.into_json()?;
    let text = &response_from_server.choices[0].message.content;

    // Find the first code block (i.e., text between ``` markers)
    let start = text.find("```").map(|s| s + 3).unwrap_or(0); // Skip the initial ```
    let end = text[start..].find("```").map(|e| start + e).unwrap_or(text.len());

    let refactored_code = text[start..end].trim(); // Extract and trim the code block

    // Split original and refactored code into lines
    let original_lines: Vec<&str> = prompt_input.lines().collect();
    let refactored_lines: Vec<&str> = refactored_code.lines().collect();

    // Determine the maximum number of lines to print
    let max_lines = original_lines.len().max(refactored_lines.len());

    // Print original and refactored code side by side with the separator moved two spaces to the right
    println!("{:<42} | {:<40}", "Original Code", "Refactored Code");
    println!("{:-<42}-+-{:-<40}", "", "");

    for i in 0..max_lines {
        let original_line = original_lines.get(i).unwrap_or(&"");
        let refactored_line = refactored_lines.get(i).unwrap_or(&"");
        println!("{:<42} | {:<40}", original_line, refactored_line);
    }

    // Append to chat history file
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("chat_history")?;

    writeln!(file, "Prompt: {}\n", generated_prompt)?;
    writeln!(file, "Response: {}\n", refactored_code)?;

    Ok(())
}




fn handle_code_task(option: &str) {
    let mut lang = String::new();
    let mut input_method = String::new();
    let mut prompt_input = String::new();
    loop{
        lang.clear();
        input_method.clear();
        prompt_input.clear();
        let mut lang = get_user_input_2("Please enter the coding language: (Press Enter Twice) ");
        println!("\n");
        let mut input_method = get_user_input_2("Would you like to (1) type your own code or (2) read and use the code from the 'input' file? Please enter 1 or 2 and Press Enter Twice: ");
        println!("\n");

        match input_method.as_str() {
            "1" =>  {
                prompt_input = call_user();
                break
            },
            "2" => {
                prompt_input = open_file();
                break
                 
            },
            _=> {
                println!("Invalid choice. Please try again.\n");
                "Error".to_string()
            },
        };
        
    }
    if let Err(err) = language_model_2(&prompt_input, &lang, option) {
        eprintln!("Error: {}", err);
    }
}


fn display_chat_history() {
    let mut file = match File::open("chat_history") {
        Ok(file) => file,
        Err(_) => {
            println!("No chat history found.");
            return;
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read the file");

    if contents.is_empty() {
        println!("No chat history found.");
    } else {
        println!("\nChat History:\n");
        println!("{}", contents);
    }
}



fn clear_chat_history() {
    let mut file = File::create("chat_history").expect("Could not clear chat history");
    file.set_len(0).expect("Failed to clear chat history");
    println!("Chat history cleared.");
}


fn print_help() {
    println!("\nHelp - AI Code Assistant\n");
    println!("1. Code Completion: Provide code and get assistance with code completion.");
    println!("2. Code Explanation: Provide code and receive an explanation in simple terms.");
    println!(
        "3. Refactoring Suggestions: Provide code and receive suggestions on how to improve it."
    );
    println!("4. Chat History: View previous prompts and responses.");
    println!("5. Clear Chat History: Remove all saved prompts and responses.");
    println!("6. Exit: Close the application.\n");
    println!("For options 1-3, you can either type your own code or use the contents of the 'input' file as your input.\n");
}

fn handle_refactoring_suggestions() {
    let lang = get_user_input_2("Please enter the coding language: (Press Enter Twice)");
    println!("\n");

    let prompt_input = loop {
        let input_method = get_user_input_2("Would you like to (1) type your own code or (2) read and use the code from the 'input' file? Please enter 1 or 2 and Press Enter Twice: ");
        println!("\n");

        let prompt_input = match input_method.as_str() {
            "1" => {
                println!("Write the code you want to input (press Enter twice to finish):");
                let mut user_code = String::new();
                loop {
                    let mut line = String::new();
                    io::stdin().read_line(&mut line).expect("Failed to read line");
                    if line.trim().is_empty() {
                        break;
                    }
                    user_code.push_str(&line);
                }
                user_code
            }
            "2" => open_file(),
            _ => {
                println!("Invalid choice. Please try again.\n");
                continue;
            },
        };

        break prompt_input;
    };

    if let Err(err) = language_model_2_3(&prompt_input, &lang, "3") {
        eprintln!("Error: {}", err);
    }
}



fn main() {
    loop {
        println!("\n");
        println!("AI Code Assistant");
        println!("\n");
        println!("1. Code Completion");
        println!("2. Code Explanation");
        println!("3. Refactoring Suggestions");
        println!("4. Display Chat History");
        println!("5. Delete Chat History");
        println!("6. Help");
        println!("7. Exit");
        println!("\n");
        print!("Choose an option and Press Enter Twice: ");
        io::stdout().flush().unwrap();

        let choice = get_user_input_2("");
        println!("\n");

        match choice.as_str() {
            "1" => file_code_part(),
            "2" => handle_code_task(&choice),
            "3" => handle_refactoring_suggestions(), // Updated to call the refactoring function
            "4" => display_chat_history(), // Updated to call the refactoring function
            "5" => clear_chat_history(), // Updated to call the refactoring function
            "6" => print_help(), // Updated to call the refactoring function
            "7" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}