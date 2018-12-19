# hp-log
high performance multi-thread log implement in rust

## Usage

``` toml
[dependencies]

``` rust
#[macro_use]
extern crate hp_log;
use hp_log::*;

fn main() {
	Logger::load_config(PathBuf::from("./"));
	info!("main running");
	debug!("main running");
	warn!("main running");
	error!("main running");
	fatal!("main running");
	Logger::close();
}

