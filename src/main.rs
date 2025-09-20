mod env;

const DEBUG: bool = true;

fn main() {
    env::check_if_dot_env_exists();
}
