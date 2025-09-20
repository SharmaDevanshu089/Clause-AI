//this is the file that will load envirment variable to load API key
use dotenv::dotenv;

//this will initialise envirment variable
pub fn intitalise() {
    dotenv().ok();
}
//this will check if file envirment file exists
