use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn encrypt() {
    let passphrase = match get_passphrase() {
        Some(pass) => pass,
        None => {
            eprintln!("ENCRYPT_KEY is empty");
            return;
        }
    };

    encrypt_env(&passphrase);
    encrypt_secrets(&passphrase);
}

fn encrypt_env(passphrase: &str) {
    let environments = ["dev", "staging", "prod"];
    for environment in &environments {
        let input_file = format!("packages/library/core/.env_{}", environment);
        let output_file = format!("release/config/{}/env_{}.gpg", environment, environment);

        encrypt_file(&passphrase, &input_file, &output_file);
    }
}

fn encrypt_secrets(passphrase: &str) {
    let environments = ["dev", "staging", "prod"];
    for environment in &environments {
        // Encrypt Release keystore
        let keystore_input = "release/app-keystore.jks";
        let keystore_output = "release/app-keystore.jks.gpg";
        encrypt_file(&passphrase, keystore_input, keystore_output);

        // Encrypt Google Services key (Android)
        let android_input = format!(
            "packages/app/android/app/src/{}/google-services.json",
            environment
        );
        let android_output = format!("release/config/{}/google-services.json.gpg", environment);
        encrypt_file(&passphrase, &android_input, &android_output);

        // Encrypt Google Services key (iOS)
        let ios_input = format!(
            "packages/app/ios/config/{}/GoogleService-Info.plist",
            environment
        );
        let ios_output = format!(
            "release/config/{}/GoogleService-Info.plist.gpg",
            environment
        );
        encrypt_file(&passphrase, &ios_input, &ios_output);

        // Encrypt firebase_app_id_file (iOS)
        let firebase_input = format!(
            "packages/app/ios/config/{}/firebase_app_id_file.json",
            environment
        );
        let firebase_output = format!(
            "release/config/{}/firebase_app_id_file.json.gpg",
            environment
        );
        encrypt_file(&passphrase, &firebase_input, &firebase_output);
    }
}

fn encrypt_file(passphrase: &str, input: &str, output: &str) {
    if Path::new(output).exists() {
        let temp_decrypted = format!("temp_decrypted_{}.tmp", output);

        let decrypt_status = Command::new("gpg")
            .arg("--batch")
            .arg("--yes")
            .arg("--quiet")
            .arg("--passphrase")
            .arg(passphrase)
            .arg("--output")
            .arg(&temp_decrypted)
            .arg("--decrypt")
            .arg(output)
            .status();

        if decrypt_status.is_ok() && decrypt_status.unwrap().success() {
            if let Ok(decrypted_content) = fs::read(&temp_decrypted) {
                if let Ok(input_content) = fs::read(input) {
                    if decrypted_content == input_content {
                        println!("No changes detected in {}, skipping encryption.", input);
                        let _ = fs::remove_file(&temp_decrypted);
                        return;
                    }
                }
            }
            let _ = fs::remove_file(&temp_decrypted);
        }
    }

    let encryption_status = Command::new("gpg")
        .arg("--batch")
        .arg("--yes")
        .arg("--passphrase")
        .arg(passphrase)
        .arg("--cipher-algo")
        .arg("AES256")
        .arg("--symmetric")
        .arg("--output")
        .arg(output)
        .arg(input)
        .status();

    if encryption_status.is_ok() && encryption_status.unwrap().success() {
        println!("Successfully encrypted {} to {}", input, output);
    } else {
        eprintln!("Failed to encrypt {} to {}", input, output);
    }
}

pub fn get_passphrase() -> Option<String> {
    let key_file = ".cryptify-key";
    if Path::new(key_file).exists() {
        match fs::read_to_string(key_file) {
            Ok(passphrase) => {
                let trimmed = passphrase.trim().to_string();
                if !trimmed.is_empty() {
                    return Some(trimmed);
                }
            }
            Err(err) => {
                eprintln!("Failed to read .cryptify-key file: {}", err);
            }
        }
    }

    if let Ok(passphrase) = std::env::var("ENCRYPT_KEY") {
        if !passphrase.is_empty() {
            return Some(passphrase);
        }
    }

    print!("Enter ENCRYPT_KEY: ");
    io::stdout().flush().unwrap();
    let mut passphrase = String::new();
    if io::stdin().read_line(&mut passphrase).is_ok() {
        return Some(passphrase.trim().to_string());
    }

    None
}
