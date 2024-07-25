
use std::process::Command;
use std::io::{self, Read, Write};

fn executing_os_commands_linux(command_full:&str) {
    
    let parts: Vec<&str> = command_full.split_whitespace().collect();
    let actual_command= parts[0];
    let arg1 = parts[1];

    let output = Command::new(actual_command).arg(arg1).output().expect("failed to execute command");
    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
    
}

/// Accepting from the console
fn accept_linux_command_from__user() -> String{

let mut buffer = String::new();
print!(" Please input your Linux command ");
io::stdout().flush().unwrap();
io::stdin().read_line(&mut buffer).unwrap();
let command = buffer.trim().to_string();
buffer.clear();
return command
}


fn main(){
let full_command:String = accept_linux_command_from__user(); // read and give back the commands they want to run
executing_os_commands_linux(&full_command);  
 // ls -la
 // echo hello
}