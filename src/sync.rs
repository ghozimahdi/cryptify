use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn sync() {
    modify_pbxproj();
}

fn modify_pbxproj() {
    let project_pbxproj_path = "packages/app/ios/Runner.xcodeproj/project.pbxproj";

    let mut pbxproj_content = String::new();
    let path = Path::new(project_pbxproj_path);
    if !path.exists() {
        eprintln!("File not found: {}", project_pbxproj_path);
        return;
    }
    let mut file = fs::File::open(path).expect("Failed to open project.pbxproj");
    file.read_to_string(&mut pbxproj_content)
        .expect("Failed to read project.pbxproj");

    let crashlytics_shell_script = r#"
# Extract the environment from the configuration name
environment=$(echo "$CONFIGURATION" | awk -F'-' '{print tolower($2)}')

# Define the directory for the firebase_app_id_file.json file
JSON_DIR="${PROJECT_DIR}/config/${environment}"

# Run the upload-symbols script with the input files
"${PODS_ROOT}/FirebaseCrashlytics/upload-symbols" --flutter-project "$PROJECT_DIR" --google-service-plist "${JSON_DIR}/GoogleService-Info.plist"
"#;

    let copy_shell_script = r#"
# Extract the environment from the configuration name
environment=$(echo "$CONFIGURATION" | awk -F'-' '{print tolower($2)}')

# Define the directory for the GoogleService-Info.plist file
PLIST_DIR="${PROJECT_DIR}/config/${environment}"

# Copy the correct GoogleService-Info.plist for the current environment into the app bundle
cp "${PLIST_DIR}/GoogleService-Info.plist" "${BUILT_PRODUCTS_DIR}/${PRODUCT_NAME}.app/GoogleService-Info.plist"
"#;

    let firebase_crashlytics_phase = format!(
        r#"
        /* [firebase_crashlytics] Crashlytics Upload Symbols */ = {{
            isa = PBXShellScriptBuildPhase;
            buildActionMask = 2147483647;
            files = (
            );
            inputFileListPaths = (
            );
            inputPaths = (
                "${{DWARF_DSYM_FOLDER_PATH}}/${{DWARF_DSYM_FILE_NAME}}",
                "${{DWARF_DSYM_FOLDER_PATH}}/${{DWARF_DSYM_FILE_NAME}}/Contents/",
                "${{DWARF_DSYM_FOLDER_PATH}}/${{DWARF_DSYM_FILE_NAME}}/Contents/Info.plist",
                "$({{TARGET_BUILD_DIR}})/$({{EXECUTABLE_PATH}})",
                "$({{PROJECT_DIR}})/config/$(echo \"$CONFIGURATION\" | awk -F'-' '{{print tolower($2)}}')/firebase_app_id_file.json",
                "$({{PROJECT_DIR}})/config/$(echo \"$CONFIGURATION\" | awk -F'-' '{{print tolower($2)}}')/GoogleService-Info.plist",
            );
            name = "[firebase_crashlytics] Crashlytics Upload Symbols";
            outputFileListPaths = (
            );
            outputPaths = (
            );
            runOnlyForDeploymentPostprocessing = 0;
            shellPath = /bin/sh;
            shellScript = "{crashlytics_shell_script}";
        }};
        "#
    );

    let firebase_copy_phase = format!(
        r#"
        /* [firebase] Copy GoogleService-Info.plist to the correct location */ = {{
            isa = PBXShellScriptBuildPhase;
            buildActionMask = 2147483647;
            files = (
            );
            inputFileListPaths = (
            );
            inputPaths = (
            );
            name = "[firebase] Copy GoogleService-Info.plist to the correct location";
            outputFileListPaths = (
            );
            outputPaths = (
            );
            runOnlyForDeploymentPostprocessing = 0;
            shellPath = /bin/sh;
            shellScript = "{copy_shell_script}";
        }};
        "#
    );

    if let Some(pos) = pbxproj_content.find("/* End PBXShellScriptBuildPhase section */") {
        pbxproj_content.insert_str(pos, &firebase_crashlytics_phase);
        pbxproj_content.insert_str(pos, &firebase_copy_phase);
    } else {
        eprintln!("Could not find the PBXShellScriptBuildPhase section.");
        return;
    }

    let mut output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open project.pbxproj for writing");
    output_file
        .write_all(pbxproj_content.as_bytes())
        .expect("Failed to write to project.pbxproj");

    println!("project.pbxproj modified successfully!");
}
