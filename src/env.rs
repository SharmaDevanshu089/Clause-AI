use dotenv:;
use std::env;
use std::path::Path;
use std::process::exit;

const ENV: &str = ".env";
const DEBUG: bool = true;
//this will initialise envirment variable
pub fn intitalise() {
    dotenv().ok();
    //i realised that it is better to check this before hand
    let _api = env::var("GEMINI_API_KEY").expect("There is No Api for GEMINI, You can Create One.");
    let _id = env::var("GCP_PROJECT_ID")
        .expect("There is No Project id for GEMINI, You can Create One in .env file");
    if DEBUG {
        text();
    }
}
fn text() {
    if DEBUG {
        let api =
            env::var("GEMINI_API_KEY").expect("There is No Api for GEMINI, You can Create One");
        println!("API Key = {}", api)
    }
}
//this will check if file envirment file exists or not
pub fn conform_env() {
    let path_to_env = Path::new(ENV);
    if DEBUG {
        let path_string = path_to_env.clone().to_string_lossy().into_owned();
        println!("Path to DOTENV is : {}", path_string);
    }
    let returning_var = path_to_env.is_file();
    if !returning_var {
        println!(
            "There is no Envirment Variable found , Please Create one .env file with google's api key; Check ENV_TEMPLATE On how to Do it . Exititng the Program"
        );
        exit(0);
    }
    intitalise();
}
pub fn get_api_key() -> String {
    let api = env::var("GEMINI_API_KEY").expect("There is No Api for GEMINI, You can Create One");
    return api;
}
pub fn get_id() -> String {
    let id = env::var("GCP_PROJECT_ID").expect("There is No Api for GEMINI, You can Create One");
    return id;
}
