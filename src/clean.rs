use std::fs;
use std::path::Path;

pub fn clean_up_files() {
    delete_file("release/app-keystore.jks");

    delete_files_in_pattern("packages/app/android/app/src/*/google-services.json");

    delete_files_in_pattern("packages/app/ios/config/*/GoogleService-Info.plist");
    delete_files_in_pattern("packages/app/ios/config/*/firebase_app_id_file.json");

    delete_file("packages/app/ios/Runner/GoogleService-Info.plist");

    delete_files_in_pattern("packages/library/core/.env_*");

    println!("Cleanup completed successfully!");
}

fn delete_file(file_path: &str) {
    if Path::new(file_path).exists() {
        match fs::remove_file(file_path) {
            Ok(_) => println!("Deleted file: {}", file_path),
            Err(err) => eprintln!("Failed to delete file {}: {}", file_path, err),
        }
    } else {
        println!("File not found, skipping: {}", file_path);
    }
}

fn delete_files_in_pattern(pattern: &str) {
    match glob::glob(pattern) {
        Ok(paths) => {
            for entry in paths.filter_map(Result::ok) {
                if entry.is_file() {
                    match fs::remove_file(&entry) {
                        Ok(_) => println!("Deleted file: {:?}", entry),
                        Err(err) => eprintln!("Failed to delete file {:?}: {}", entry, err),
                    }
                }
            }
        }
        Err(err) => eprintln!("Failed to read pattern {}: {}", pattern, err),
    }
}
