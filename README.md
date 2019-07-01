# hp-log
high performance multi-thread log implement in rust

## Usage

use the newest version from crate.io (recommand)
``` toml
[dependencies]
hp-log = "0.3~"
```
use the newest version by git
``` toml
[dependencies]
hp-log = {git = "https://github.com/wtu-sos/hp-log.git"}
```
## Config File
``` toml
# log level filter
# OFF  ERROR  WARN  INFO  DEBUG  TRACE 
global_max_level = "TRACE"
[console_conf]
switch = true  # Whether the output is on the console
debug = true
info = true 
warn = true 
error = true
fatal = true

[file_conf]
switch = true  # Whether to output in the log file 
debug = true
info = true 
warn = true 
error = true
fatal = true

file_log_dir = "/tmp/log/" 
file_temp_buf = "1048576"

```

## Generated Code Example
``` rust
#![feature(rustc_private)]

extern crate hp_log;
#[macro_use]
extern crate log;

fn main() {
    hp_log::init("./".to_string());
    
    trace!("main running ````````````````````");
    info!("main running info");
    debug!("main running .........................");
    warn!("main running ****************");
    error!("main running +++++++++++++++++++++++++");

    hp_log::close();

}
```

## License
`hp-log` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2018 wtu-sos
