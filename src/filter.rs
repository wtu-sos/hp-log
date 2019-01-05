use crate::config::Config;
use crate::color::Color;

#[derive(Copy, Clone)]
pub enum FilterLevel {
    Debug = 1,
    Info  = 2,
    Warn  = 4,
    Error = 8,
    Fatal = 16,
} 

pub struct Filters {
    filter: u8,
}


#[allow(dead_code)]
impl Filters {
    pub fn generate_by_config() -> Self {
        let mut filter = 0u8;
        if Config::instance().debug() {
            filter |= FilterLevel::Debug as u8;
        }

        //println!("info : {}", Config::instance().info());
        if Config::instance().info() {
            filter |= FilterLevel::Info as u8;
        }

        if Config::instance().warn() {
            filter |= FilterLevel::Warn as u8;
        }

        if Config::instance().error() {
            filter |= FilterLevel::Error as u8;
        }

        if Config::instance().fatal() {
            filter |= FilterLevel::Fatal as u8;
        }

        //println!("filter : {}", filter);

        Filters {
            filter,
        }
    }

    pub fn get_filter(&self) -> u8 {
        self.filter
    }

    #[allow(dead_code)]
    pub fn is_pass(&self, level: FilterLevel) -> bool {
        return 0 != (self.filter & level as u8);
    }
}

#[allow(dead_code)]
impl FilterLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            FilterLevel::Debug => "DEBUG",
            FilterLevel::Info  => "INFO",
            FilterLevel::Warn  => "WARN",
            FilterLevel::Error => "ERROR",
            FilterLevel::Fatal => "FATAL",
        } 
    }

    pub fn fg_color(&self) -> Color {
        match self {
            FilterLevel::Debug => Color::Cyan,
            FilterLevel::Info  => Color::Green,
            FilterLevel::Warn  => Color::Yellow,
            FilterLevel::Error => Color::Red,
            FilterLevel::Fatal => Color::Red,
        } 
    }

    pub fn bg_color(&self) -> Color {
        match self {
            FilterLevel::Debug => Color::Black,
            FilterLevel::Info  => Color::Black,
            FilterLevel::Warn  => Color::Black,
            FilterLevel::Error => Color::Black,
            FilterLevel::Fatal => Color::White,
        } 
    }

    pub fn from(digit: u8) -> FilterLevel {
        match digit {
            1  => FilterLevel::Debug,
            2  => FilterLevel::Info,
            4  => FilterLevel::Warn,
            8  => FilterLevel::Error,
            16 => FilterLevel::Fatal,
            _ => unimplemented!(),
        }
    }
}
