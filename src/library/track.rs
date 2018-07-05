use library::ellingtondata::EllingtonData;
use library::format::{Mp3, UnknownFile};
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TrackImpl {
    pub location: PathBuf,
    // metadata. Should this be handled separately?
    // these are from the "file", as in, read, from the file metadata
    pub name: String,
    pub bpm: Option<i64>,              // we might not have a bpm value
    pub comments: Option<Vec<String>>, // or comments!
}

pub trait Track: fmt::Display + Debug {
    fn location(self: &Self) -> PathBuf;
    fn name(self: &Self) -> Option<String>;
    fn bpm(self: &Self) -> Option<i64>;
    fn comments(self: &Self) -> Option<Vec<String>>;
    fn ellington_data(self: &Self) -> Option<Vec<EllingtonData>>;
    fn write_data(self: &Self, new_data: EllingtonData) -> Option<()>; 
    fn from_file_impl(path: &PathBuf) -> Option<Box<Track + 'static>>
    where
        Self: Sized;
}

impl Track {
    pub fn from_file(location: &PathBuf) -> Option<Box<Track + 'static>> {
        match location.extension().and_then(OsStr::to_str) {
            Some("mp3") => Mp3::from_file_impl(location),
            Some("mp4") => UnknownFile::from_file_impl(location),
            Some("m4a") => UnknownFile::from_file_impl(location),
            Some("flac") => UnknownFile::from_file_impl(location),
            _ => UnknownFile::from_file_impl(location),
        }
    }
}