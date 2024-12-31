mod clean;
mod decrypt;
mod encrypt;
mod sync;

use crate::clean::clean_up_files;
use crate::decrypt::decrypt;
use crate::encrypt::encrypt;
use crate::sync::sync;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let output = Command::new("gpg").arg("--version").output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let args: Vec<String> = env::args().collect();

                if args.len() < 2 {
                    println!("Usage: cryptify <command> [options]");
                    println!("Available commands:");
                    println!("  init           Initialize cryptify configuration");
                    println!("  add <path>     Add a file or directory to encrypt");
                    println!("  encrypt        Encrypt added files");
                    println!("  decrypt        Decrypt files");
                    println!(
                        "  sync           Distribute decrypted files to predefined platform paths"
                    );
                    println!("  clean          Clean up temporary files");
                    return;
                }

                match args[1].as_str() {
                    "init" => init(),
                    "sync" => sync(),
                    "add" => {
                        if args.len() < 3 {
                            println!("Usage: cryptify add <path>");
                        } else {
                            add(&args[2]);
                        }
                    }
                    "encrypt" => encrypt(),
                    "key" => set_key(&args[2]),
                    "decrypt" => decrypt(),
                    "clean" => clean_up_files(),
                    _ => println!("Unknown command. Use 'cryptify' for help."),
                }
            } else {
                println!("GnuPG is not found. Installing it using Homebrew...");
                install_gnupg();
            }
        }
        Err(_) => {
            println!("GnuPG is not found. Installing it using Homebrew...");
            install_gnupg();
        }
    }
}

fn set_key(key: &str) {
    let key = key.trim();

    if key.is_empty() {
        println!("Key cannot be empty. Initialization aborted.");
        return;
    }

    match File::create(".cryptify-key") {
        Ok(mut file) => {
            if let Err(err) = file.write_all(key.as_bytes()) {
                eprintln!("Failed to write key to file: {}", err);
            } else {
                println!("Key has been saved to .cryptify-key.");
            }
        }
        Err(err) => {
            eprintln!("Failed to create .cryptify-key file: {}", err);
        }
    }
}

fn install_gnupg() {
    let install_output = Command::new("brew").arg("install").arg("gnupg").output();

    match install_output {
        Ok(output) => {
            if output.status.success() {
                println!("GnuPG has been successfully installed.");
                println!("You can now use cryptify commands.");
            } else {
                eprintln!(
                    "Failed to install GnuPG: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(err) => {
            eprintln!("Error executing brew: {}", err);
        }
    }
}

fn init() {
    println!("Initializing cryptify configuration...");

    print!("Input key: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read input");

    set_key(&key);
}

fn add(path: &str) {
    println!("Adding path to encryption list: {}", path);
    // TODO: Add logic to store the path for encryption
}
