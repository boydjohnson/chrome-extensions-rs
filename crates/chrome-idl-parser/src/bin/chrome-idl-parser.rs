use {
    chrome_idl_parser::json::ChromeApi,
    std::io::{stdin, stdout},
};

fn main() {
    let api: Vec<ChromeApi> = serde_json::from_reader(stdin()).unwrap();
    serde_json::to_writer(stdout(), &api).unwrap();
}
