//this is the file that will load envirment variable to load API key
use dotenv::dotenv;
//this will load the path
use std::path::Path;
//for exititng a program instead of panicing
use std::process::exit;

const ENV: &str = ".env";
const DEBUG: bool = true;
//this will initialise envirment variable
pub fn intitalise() {
    dotenv().ok();
}
//this will check if file envirment file exists
pub fn check_if_dot_env_exists() -> bool {
    let path_to_env = Path::new(ENV);
    if DEBUG {
        let path_string = path_to_env.clone().to_string_lossy().into_owned();
        println!("Path to DOTENV is : {}", path_string);
    }
    let returning_var = path_to_env.exists();
    if !returning_var {
        println!(
            "There is no Envirment Variable found , Please Create one .env file with google's api key"
        );
    }
    return returning_var;
}
