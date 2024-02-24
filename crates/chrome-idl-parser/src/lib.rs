use {
    crate::json::ChromeApi,
    std::{
        ffi::OsString,
        fs::{read_dir, FileType},
        path::Path,
    },
};

pub mod json;

pub fn generate(from: &Path, to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut i = 1;
    for item in read_dir(&from)? {
        let entry = item?;
        if entry
            .file_name()
            .into_string()
            .unwrap()
            .ends_with("private.json")
            || entry.file_name().into_string().unwrap().starts_with("_")
        {
            continue;
        }

        if let Ok(file_type) = entry.file_type() {
            if !file_type.is_file() {
                continue;
            }
        }

        if !entry.file_name().into_string().unwrap().ends_with(".json") {
            continue;
        }

        let buf = std::fs::read_to_string(&entry.path())?;

        let buf = buf
            .split('\n')
            .filter(|el| !el.starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");

        match deser_hjson::from_str::<Vec<ChromeApi>>(&buf) {
            Ok(v) => {
                println!("{i}");
                i += 1;
            }
            Err(e) => {
                println!("{:?}: {:?}", e, entry.path());
            }
        }
    }

    Ok(())
}
