use std::path::PathBuf;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;

extern crate libellington as le;

// use le::library::library::*;

// use le::library::ellingtondata::BpmInfo;
// use le::library::ellingtondata::EllingtonData;

// use le::input::audiostream::AudioStream;
// use le::analysers::bpmtools::BpmTools;
// use le::shelltools::sox::*;
// use le::shelltools::ffmpeg::*;
use le::library::library::Library;
use le::actions::*;


// #[flame]
// fn process_library(library: Library) -> () {
//     info!("Successfully parsed {} tracks.", library.tracks.len());
//     // Iterate over the tracks.
//     for track in library.tracks {
//         // flame::start("process_track");

//         // Match the tracks that contain ellington data
//         match track.ellington_data() {
//             // If we have ellington data
//             Some(ed) => {
//                 info!("Track: {}", track);
//                 info!("Bpm: {:?}", track.bpm());
//                 info!("Comment: {:#?}", track.comments());
//                 info!("Ed: {:#?}", ed);

//                 let mut call = FfmpegCommand::default(&track.location());
//                 let mut child = call.run();

//                 let cbpm = {
//                     let sox_stream = match &mut child.stdout {
//                         Some(s) => Some(AudioStream::from_stream(s)),
//                         None => None,
//                     }.unwrap();

//                     let calculated_bpm = 
//                         BpmTools::default().analyse(sox_stream);

//                     calculated_bpm
//                 };

//                 child.wait().expect("failed to wait on child");

//                 info!("Calculated ffmpeg bpm: {}", cbpm);

//                 let mut call = SoxCommand::default(&track.location());
//                 let mut child = call.run();

//                 let cbpm = {
//                     let sox_stream = match &mut child.stdout {
//                         Some(s) => Some(AudioStream::from_stream(s)),
//                         None => None,
//                     }.unwrap();

//                     let calculated_bpm = 
//                         BpmTools::default().analyse(sox_stream);

//                     calculated_bpm
//                 };

//                 child.wait().expect("failed to wait on child");

//                 info!("Calculated sox bpm: {}", cbpm);

//                 // build some ellington data 
//                 // let new_data = EllingtonData { 
//                 //     algs: Some (
//                 //         vec![BpmInfo{
//                 //             bpm: cbpm as i64, 
//                 //             alg: "naive".to_string()
//                 //         }]
//                 //     )
//                 // };

//                 // match track.write_data(new_data) {
//                 //     Some(_) => info!("Successfully written data."), 
//                 //     None => info!("Failed to write id3 data for some reason.")
//                 // }
                
//                 info!("===== ===== ===== ===== =====\n");
//             }
//             _ => {
//                 info!("Ignore... {:?}", track.name());
//             }
//         }

//         // flame::end("process_track");
//     }
// }

// #[flame]
fn dispatch(matches: ArgMatches) -> () {

    let library = match (matches.value_of("library"), matches.value_of("directory"), matches.is_present("stream")) { 
        (Some(library_file), _, _ ) => {
            info!("Processing from library: {:?}", library_file);
            Library::from_itunes_xml(library_file)
        }
        (_, Some(directory), _ ) => {
            info!("Reading from directory: {}", directory);
            Library::from_directory_rec(&PathBuf::from(directory))
        }
        (_, _, true) => {
            info!("Reading track file names from stdin.");
            None
        }
        _ => {
            error!("Should not reach here!"); 
            None
        }
    };

    // info!("Got library: {:#?}", library);
    // println!("Titles: \n{:#?}", TrackTitles::run(library.unwrap()));
    // process_library(library.unwrap());
}

fn main() {
    env_logger::init();
    // get the command line arguments to the program
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    info!("Application started");

    dispatch(matches);
}