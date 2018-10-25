use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind, Read};
use std::fs::File;
use std::sync::{Once, ONCE_INIT};

pub static mut INST: Option<Config> = None;
static INST_ONCE: Once = ONCE_INIT;

macro_rules! is_nightly_channel {
    () => {
        option_env!("CFG_RELEASE_CHANNEL")
            .map(|c| c == "nightly")
            .unwrap_or(true)
    };
}

macro_rules! create_config {
    // `name: value type, default value, is stable, description;`
    ($($i:ident: $ty:ty, $def:expr, $stb:expr, $( $dstring:expr ),+);+ $(;)*) => (

        #[derive(Debug, Clone)]
        pub struct Config {
            $($i: (Cell<bool>, bool, $ty, bool)),+
        }

        #[derive(Deserialize, Serialize, Clone)]
        pub struct PartialConfig {
            $(pub $i: Option<$ty>),+
        }

        #[allow(dead_code)]
        impl PartialConfig {
            pub fn to_toml(&self) -> Result<String, String> {
                let cloned = self.clone();

                ::toml::to_string(&cloned)
                    .map_err(|e| format!("Could not output config: {}", e.to_string()))
                //Ok("".to_string())
            }
        }

        pub struct ConfigSetter<'a>(&'a mut Config);

        #[allow(dead_code)]
        impl<'a> ConfigSetter<'a> {
            $( pub fn $i(&mut self, value: $ty) { (self.0).$i.2 = value; } )+
        }

        // Query each option, returns true if the user set the option, false if
        // a default was used.
        pub struct ConfigWasSet<'a>(&'a Config);

        #[allow(dead_code)]
        impl<'a> ConfigWasSet<'a> {
            $(
            pub fn $i(&self) -> bool {
                (self.0).$i.1
            }
            )+
        }

        #[allow(dead_code)]
        impl Config {
            pub fn create_instance( file_path: Option<PathBuf> ) {
                INST_ONCE.call_once(|| {
                    unsafe {
                        if let Ok(result) = load_config(file_path) {
                            INST = Some(result);
                        } else {
                            panic!("load config failed! ");
                        }
                    }
                });
            }

            pub fn instance() -> &'static Self {
                unsafe {
                    assert!(INST.is_some(), "instance is None");
                    INST.as_ref().expect("get instance erro")
                }
            }

            $(
            pub fn $i(&self) -> $ty {
                self.$i.0.set(true);
                self.$i.2.clone()
            }
            )+

            pub fn set<'a>(&'a mut self) -> ConfigSetter<'a> {
                ConfigSetter(self)
            }

            pub fn was_set<'a>(&'a self) -> ConfigWasSet<'a> {
                ConfigWasSet(self)
            }

            pub fn from_toml_path(file_path: &Path) -> Result<Config, Error> {
                let mut file = File::open(&file_path)?;
                let mut toml = String::new();
                file.read_to_string(&mut toml)?;
                println!("toml: {:?}", toml);
                Config::from_toml(&toml)
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))
            }

            pub(crate) fn from_toml(toml: &str) -> Result<Config, String> {
                let parsed: ::toml::Value =
                    toml.parse().map_err(|e| format!("Could not parse TOML: {}", e))?;
                let mut err: String = String::new();
                {
                    let table = parsed
                        .as_table()
                        .ok_or(String::from("Parsed config was not table"))?;
                    for key in table.keys() {
                        if !Config::is_valid_name(key) {
                            let msg = &format!("Warning: Unknown configuration option `{}`\n", key);
                            err.push_str(msg)
                        }
                    }
                }
                match parsed.try_into() {
                    Ok(parsed_config) => {
                        if !err.is_empty() {
                            eprint!("{}", err);
                        }
                        Ok(Config::default().fill_from_parsed_config(parsed_config))
                    }
                    Err(e) => {
                        err.push_str("Error: Decoding config file failed:\n");
                        err.push_str(format!("{}\n", e).as_str());
                        err.push_str("Please check your config file.");
                        Err(err)
                    }
                }
            }

            fn fill_from_parsed_config(mut self, parsed: PartialConfig) -> Config {
            $(
                if let Some(val) = parsed.$i {
                    if self.$i.3 {
                        self.$i.1 = true;
                        self.$i.2 = val;
                    } else {
                        if is_nightly_channel!() {
                            self.$i.1 = true;
                            self.$i.2 = val;
                        } else {
                            eprintln!("Warning: can't set `{} = {:?}`, unstable features are only \
                                       available in nightly channel.", stringify!($i), val);
                        }
                    }
                }
            )+
                self
            }

            pub(crate) fn is_valid_name(name: &str) -> bool {
                match name {
                    $(
                        stringify!($i) => true,
                    )+
                        _ => false,
                }
            }

        }

        // Template for the default configuration
        impl Default for Config {
            fn default() -> Config {
                Config {
                    $(
                        $i: (Cell::new(false), false, $def, $stb),
                    )+
                }
            }
        }
    )
}

create_config!(
       // Comments. macros, and strings
        debug: bool, true, false, "Debug";
        info: bool, true, false, "Info";
        warn: bool, true, false, "Warn";
        error: bool, true, false, "Error";
        fatal: bool, true, false, "Fatal";
    );


/// Load a config by checking the client-supplied options and if appropriate, the
/// file system (including searching the file system for overrides).
pub fn load_config<T: Into<Option<PathBuf>>>( file_path: T) -> Result<Config, Error> {
    if let Some(file_path) = file_path.into() {
        const CONFIG_FILE_NAMES: [&str; 2] = ["Log.toml", "log.toml"];
        for config_file_name in &CONFIG_FILE_NAMES {
            let mut file_path_cloned = file_path.clone();
            file_path_cloned.push(Path::new(config_file_name));
            if file_path_cloned.is_file() {
                return Config::from_toml_path(&file_path_cloned);
            }
        }
    }

    Ok(Config::default())
}

