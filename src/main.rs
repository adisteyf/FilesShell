extern crate ctrlc;
extern crate colored;
use std::io;
use std::str;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::process::Command;
use std::fs;
use std::vec::Vec;
use std::env;
use crate::colored::Colorize;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let user: String;
    let mut pwd: String;

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    match env::var("USER") {
        Ok(val) => {
            user = val.clone();
        },
        Err(_) => user = String::from("error")
    }

    loop {
        let mut line = String::new();
        let mut output = Command::new("pwd")
            .output()
            .expect("error during run command");
            output.stdout.pop();
        pwd = String::from_utf8_lossy(&output.stdout).to_string();
        pwd=pwd.replace(format!("/home/{}", user).as_str(), "~");

        let prefix = format!("\n˥({u})\n˩ {pwd} {prig} ", u=user.color("green"), pwd=pwd.yellow(), prig="≻".bold());
        // println!("|({u}@{h})", u=user, h=host);
        // print!("└{}➔ ", pwd);
        print!("{}", prefix);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
    
        // io::stdin().read_line(&mut line)
        //                 .expect("ERROR DURING READLINE");
        match io::stdin().read_line(&mut line) {
            Ok(_) => {},
            Err(err) => println!("Could not parse input: {}", err)
        }
    
        if line.bytes().any(|b| b == 24) {
            println!("Detected Ctrl+X (^X) key combination. Exiting...");
            break;
        }

        line.pop();

        let main_command: &str = line.split_whitespace().next().unwrap_or_default();
        if fs::metadata(format!("/bin/{}", main_command)).is_ok() && main_command != "" {
            let mut i = 0;
            let mut has_param = false;
            let mut params = Vec::new();
            for word in line.split_whitespace() {
                if has_param {
                    params.push(word);
                }
                if i == 1 && word == ":" {
                    has_param=true;
                }
                i+=1;
            }

            if params.is_empty() {
                let output = Command::new(format!("/bin/{}", main_command))
                    .output()
                    .expect("error during run command");
                println!("{}", String::from_utf8_lossy(&output.stdout));
                if !output.status.success() {
                    println!("{}", output.status);
                }
            } else {
                let output = Command::new(format!("/bin/{}", main_command))
                    .args(&params)
                    .output()
                    .expect("error during run command");
                println!("{}", String::from_utf8_lossy(&output.stdout));
                if !output.status.success() {
                    println!("{}", output.status);
                }
            }
        } else if main_command == "cd" {
            let output = Command::new("/bin/pwd")
                .output()
                .expect("error during run command");
            let mut i = 0;
            for word in line.split_whitespace() {
                if i==1 {
                    let mut pwd_out = String::new();
                    pwd_out.push_str(match str::from_utf8(&output.stdout) {
                        Ok(val) => val,
                        Err(_) => panic!("error during push pwd_out for cd"),
                    });
                    pwd_out.pop();
                    //assert!(env::set_current_dir(format!("{a}/{b}", a=pwd_out.as_str(), b=word)).is_ok());
                    if !env::set_current_dir(format!("{a}/{b}", a=pwd_out.as_str(), b=word)).is_ok() {
                        let mut word_st = String::from(word);
                        match env::var("HOME") {
                            Ok(val) => {
                                word_st = word_st.replace("~", &val);
                                val
                            },
                            Err(e) => String::from("error")
                        };
                        if !env::set_current_dir(&word_st).is_ok() {
                            println!("directory '{}' not found or access denied!", word_st.as_str());
                        }
                    }
                    break;
                }
                i+=1;
            }
        }
        
        
        
        
        else {
            match line.as_str() {
                "" => {},
                "exit" => break,
                _ => println!("command '{}' not found!", line.as_str())
            }
        }
    }
}