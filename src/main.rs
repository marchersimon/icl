use std::{fs::File, env};
use std::io::Read;
use clap::{arg, Command};
use log::{debug, error};


pub struct Options {
    infile: String,
    debug: bool,
}

pub struct MIDIFile {
    buffer: Vec<u8>,
    pos: usize,
    file_format: FileFormat,
    number_of_track_chunks: u16,
    division: i16,
}

enum FileFormat {
    SingleTrack,
    MultipleTrack,
    MultipleSong,
}

impl MIDIFile {
    
    fn read_header(&mut self) -> Result<(), String> {
        // Identifier
        let identifier = self.get_string(4)?;
        if identifier != "MThd" {
            return Err(
                format!("Wrong identifier for header chunk: Expected \"MThd\" but got \"{}\"", identifier)
                .to_string());
        }
        
        // Header Length
        let header_lenght = self.get_dword()?;
        if header_lenght != 6 {
            return Err(
                format!("Wrong header chunk length: Expected 0x06 but got {:#06x}", header_lenght)
                .to_string());
        }
        
        // File format
        match self.get_word()? {
            0 => {
                debug!("Single Track File Format");
                self.file_format = FileFormat::SingleTrack;
            },
            1 => {
                debug!("Multiple Track File Format");
                self.file_format = FileFormat::MultipleTrack;
            },
            2 => {
                debug!("Multiple Song File Format");
                self.file_format = FileFormat::MultipleSong;
            },
            file_format @ _ => {
                return Err(
                    format!("Invalid file format: {}", file_format)
                    .to_string());
            }
        }
        
        // Number of Track Chunks
        self.number_of_track_chunks = self.get_word()?;
        if self.number_of_track_chunks == 0 {
            return Err("MIDI File must have at least one track chunk".to_string());
        }
        
        // Division
        self.division = self.get_word()? as i16;
        if self.division > 0 {
            debug!("Division given in ticks per beat");
        } else if self.division < 0 {
            debug!("Division given in SMPTE format");
        } else {
            return Err("Division cannot be zero".to_string());
        }
        Ok(())
    }

    fn new(buffer: Vec<u8>) -> Result<MIDIFile, String> {
        let mut file = MIDIFile {
            buffer,
            pos: 0,
            file_format: FileFormat::SingleTrack,
            number_of_track_chunks: 0,
            division: 0,
        };

        match file.read_header() {
            Ok(()) => Ok(file), 
            Err(str) => Err(str),
        }
    }
    
    fn get_byte(&mut self) -> Result<u8, String> {
        if self.pos == self.buffer.len() {
            return Err("File ended unexpectedly".to_string());
        }
        let byte = self.buffer[self.pos];
        self.pos += 1;
        Ok(byte)
    }
    
    fn get_string(&mut self, len: usize) -> Result<String, String> {
        let mut str = String::new();
        for _i in 0..len {
            str.push_str(&(self.get_byte()? as char).to_string());
        }
        Ok(str)
    }

    fn get_word(&mut self) -> Result<u16, String> {
        Ok((self.get_byte()? as u16) << 8  |
           (self.get_byte()? as u16))
    }

    fn get_dword(&mut self) -> Result<u32, String> {
        Ok((self.get_byte()? as u32) << 24 |
           (self.get_byte()? as u32) << 16 |
           (self.get_byte()? as u32) << 8  |
           (self.get_byte()? as u32))
    }
}

pub fn cli_parse() -> Options {
    let matches = Command::new("tinymid")
        .version("0.1")
        .about("A command line MIDI viewer")
        .arg(arg!(<infile> "File to read"))
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .get_matches();

    let opts = Options{infile: matches.value_of("infile").unwrap().to_string(), debug: matches.is_present("debug")};

    opts
}

pub fn read_file(name: String) -> Result<Vec<u8>, std::io::Error>{
    let mut file = File::open(&name)?;
    let metadata = std::fs::metadata(&name)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer)?;
    Ok(buffer)
}

fn main() {

    
    let opts = cli_parse();    
    
    if opts.debug {
        env::set_var("RUST_LOG", "debug");
    }

    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    
    let buffer: Vec<u8>;
    match read_file(opts.infile) {
        Ok(buff) => buffer = buff,
        Err(err) => {
            error!("{}", err.to_string());
            std::process::exit(1);
        },
    };

    let _mid1;
    match MIDIFile::new(buffer) {
        Ok(mid) => _mid1 = mid,
        Err(str) => {
            error!("{}", str);
            std::process::exit(1);
        },
    }
}