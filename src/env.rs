//this is the file that will load envirment variable to load API key
use dotenv::dotenv;
//this will load the path
use std::path::Path;
//for exititng a program instead of panicing
use std::process::exit;
//for testing only
use std::env;

const ENV: &str = ".env";
const DEBUG: bool = true;
//this will initialise envirment variable
pub fn intitalise() {
    dotenv().ok();
    //i realised that it is better to check this before hand
    let _api = env::var("GEMINI_API_KEY").expect("There is No Api for GEMINI, You can Create One");
    if DEBUG {
        text();
    }
}
fn text() {
    if DEBUG {
        println!("API Key = {}", _api)
    }
}
//this will check if file envirment file exists
pub fn conform_env() {
    let path_to_env = Path::new(ENV);
    if DEBUG {
        let path_string = path_to_env.clone().to_string_lossy().into_owned();
        println!("Path to DOTENV is : {}", path_string);
    }
    let returning_var = path_to_env.is_file();
    if !returning_var {
        println!(
            "There is no Envirment Variable found , Please Create one .env file with google's api key; Check Github On how to Do it . Exititng the Program"
        );
        exit(0);
    }
    intitalise();
}
