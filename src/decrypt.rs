use crate::encrypt::get_passphrase;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn decrypt() {
    let passphrase = match get_passphrase() {
        Some(pass) => pass,
        None => {
            eprintln!("ENCRYPT_KEY is empty");
            return;
        }
    };

    #[cfg(debug_assertions)]
    eprintln!("Key: {}", passphrase);

    decrypt_env(&passphrase);
    decrypt_secrets(&passphrase);
}

fn decrypt_env(passphrase: &str) {
    let environments = ["dev", "staging", "prod"];
    for environment in &environments {
        let input_file = format!("release/config/{}/env_{}.gpg", environment, environment);
        let output_file = format!("packages/library/core/.env_{}", environment);

        decrypt_file(&passphrase, &input_file, &output_file);
    }
}

fn decrypt_secrets(passphrase: &str) {
    let environments = ["dev", "staging", "prod"];
    for environment in &environments {
        // Ensure iOS config folder exists
        let ios_config_path = format!("packages/app/ios/config/{}", environment);
        if !Path::new(&ios_config_path).exists() {
            if let Err(e) = fs::create_dir_all(&ios_config_path) {
                eprintln!("Failed to create directory {}: {}", ios_config_path, e);
                continue;
            }
        }

        // Decrypt Release keystore
        let keystore_input = "release/app-keystore.jks.gpg";
        let keystore_output = "release/app-keystore.jks";
        decrypt_file(&passphrase, keystore_input, keystore_output);

        // Decrypt Google Services key (Android)
        let android_input = format!("release/config/{}/google-services.json.gpg", environment);
        let android_output = format!(
            "packages/app/android/app/src/{}/google-services.json",
            environment
        );
        decrypt_file(&passphrase, &android_input, &android_output);

        // Decrypt Google Services key (iOS)
        let ios_input = format!(
            "release/config/{}/GoogleService-Info.plist.gpg",
            environment
        );
        let ios_output = format!("{}/GoogleService-Info.plist", ios_config_path);
        decrypt_file(&passphrase, &ios_input, &ios_output);

        // Decrypt firebase_app_id_file (iOS)
        let firebase_input = format!(
            "release/config/{}/firebase_app_id_file.json.gpg",
            environment
        );
        let firebase_output = format!("{}/firebase_app_id_file.json", ios_config_path);
        decrypt_file(&passphrase, &firebase_input, &firebase_output);
    }
}

fn decrypt_file(passphrase: &str, input: &str, output: &str) {
    if Path::new(input).exists() {
        let decryption_status = Command::new("gpg")
            .arg("--batch")
            .arg("--yes")
            .arg("--quiet")
            .arg("--passphrase")
            .arg(passphrase)
            .arg("--output")
            .arg(output)
            .arg("--decrypt")
            .arg(input)
            .status();

        if decryption_status.is_ok() && decryption_status.unwrap().success() {
            println!("Successfully decrypted {} to {}", input, output);
        } else {
            eprintln!("Failed to decrypt {} to {}", input, output);
        }
    } else {
        eprintln!("Input file {} does not exist, skipping decryption.", input);
    }
}
