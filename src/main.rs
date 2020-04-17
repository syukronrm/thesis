mod config;
mod ikdsr;
mod prelude;
mod queries;
mod src;
mod types;

use prelude::*;

fn main() {
    let conf: AppConfig = Default::default();

    println!("{}", "main()");
    println!("{:?}", conf);
}
