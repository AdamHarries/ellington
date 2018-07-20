extern crate byteorder;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

// extern crate slog; 
// extern crate slog_term; 
// extern crate slog_scope; 
// extern crate flame;
extern crate histogram;
extern crate id3;
extern crate itertools;
extern crate memmap;
extern crate percent_encoding;
extern crate plist;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate walkdir;

#[macro_use]
extern crate serde_derive;

extern crate regex;
#[macro_use]
extern crate lazy_static;

extern crate taglib;

pub mod analysers;
pub mod input; 
pub mod library; 
pub mod shelltools;

pub fn trueish() -> bool{ 
    true 
}

#[cfg(test)]
mod tests {
    #[test]
    fn tautology_internal() {
        assert!(super::trueish());
    }
}
