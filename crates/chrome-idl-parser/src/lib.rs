use {
    crate::json::ChromeApi,
    quote::quote,
    std::{
        ffi::OsString,
        fs::{read_dir, FileType},
        io::Write,
        path::Path,
    },
};

pub mod json;
mod utils;

trait ToWasmBindgen {
    fn to_wasm_bindgen(&self) -> proc_macro2::TokenStream;
}

pub fn generate(from: &Path, to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut mods = vec![];

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
                let mut to_path = to.to_path_buf();
                to_path.push(entry.file_name());

                to_path.set_extension("rs");

                mods.push(to_path.file_stem().unwrap().to_str().unwrap().to_string());

                for chrome in v {
                    let mut output = std::fs::File::create(&to_path)?;

                    output.write(format!("{}", chrome.to_wasm_bindgen()).as_bytes())?;
                }
            }
            Err(e) => {
                println!("{:?}: {:?}", e, entry.path());
            }
        }
    }

    let token_stream: proc_macro2::TokenStream = mods
        .into_iter()
        .flat_map(|el| {
            let ident = proc_macro2::Ident::new(&el, proc_macro2::Span::call_site());
            quote!(
                pub mod #ident;
            )
        })
        .collect();

    let mut to_path = to.to_path_buf();

    to_path.push("lib.rs");

    std::fs::write(to_path, &format!("{}", token_stream))?;

    Ok(())
}
