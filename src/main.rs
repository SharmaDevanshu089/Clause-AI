mod env;
mod server;

#[macro_use] extern crate rocket;

const DEBUG: bool = true;

fn main() {
    //Important Conforms the Env
    env::conform_env();
}
