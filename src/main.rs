mod config;
mod prelude;
mod src;
mod types;

use prelude::*;

fn main() {
    let conf: AppConfig = Default::default();

    println!("{}", "main()");
    println!("{:?}", conf);
}
