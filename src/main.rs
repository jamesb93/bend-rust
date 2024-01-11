// External crates
extern crate getopts;
extern crate hound;
extern crate rayon;
extern crate indicatif;

// Standard library
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, BufWriter};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

// External crates
use getopts::Options;
use hound::{WavWriter, WavSpec};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;


fn read_input_file(file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_wav_header(output_path: &str, sample_rate: u32, num_channels: u16, bits_per_sample: u16) -> Result<WavWriter<BufWriter<File>>, io::Error> {
    let spec = WavSpec {
        channels: num_channels,
        sample_rate,
        bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };

    let writer = WavWriter::create(output_path, spec).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(writer)
}

fn write_output_file(mut header: WavWriter<BufWriter<File>>, input_data: &[u8], bits_per_sample: u16) -> Result<(), io::Error> {
    for &data in input_data {
        if bits_per_sample == 8 {
            header.write_sample(data as i8).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        } else {
            header.write_sample((data as i16) << 8).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        }
    }
    header.finalize().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(())
}

fn process_file(input_path: &Path, output_path: &Path, sample_rate: u32, bit_depth: u16) {
    let input_path_str = input_path.to_str().expect("Invalid input file path");
    let output_path_str = output_path.to_str().expect("Invalid output file path");

    if let Ok(input_data) = read_input_file(input_path_str) {
        if let Ok(header) = write_wav_header(output_path_str, sample_rate, 1, bit_depth) {
            if let Ok(()) = write_output_file(header, &input_data, bit_depth) {
            } else {
                eprintln!("Error writing output file: {}.", output_path.display());
            }
        } else {
            eprintln!("Error creating WAV header.");
        }
    } else {
        eprintln!("Error reading input file: {}", input_path.display());
    }
}


fn process_directory(input_path: &Path, output_path: &Path, sample_rate: u32, bit_depth: u16, total_size: Arc<AtomicU64>, limit: u64, pb: &ProgressBar) {
    let entries: Vec<_> = fs::read_dir(input_path).unwrap().collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    entries.into_par_iter().for_each(|entry| {
        let file_size = entry.metadata().unwrap().len();

        if total_size.load(Ordering::SeqCst) + file_size > limit {
            return;
        }

        total_size.fetch_add(file_size, Ordering::SeqCst);
        pb.inc(file_size as u64);

        let input_file_path = entry.path();
        if input_file_path == *output_path {
            return;
        }

        if input_file_path.is_dir() {
            process_directory(&input_file_path, output_path, sample_rate, bit_depth, total_size.clone(), limit, pb);
        } else {
            let mut output_file_path = output_path.join(input_file_path.file_stem().unwrap());
            output_file_path.set_extension("wav");

            process_file(&input_file_path, &output_file_path, sample_rate, bit_depth);
        }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("b", "bit-depth", "Set the bit depth (8 or 16)", "BIT_DEPTH");
    opts.optopt("s", "sample-rate", "Set the sample rate", "SAMPLE_RATE");
    opts.optopt("l", "limit", "Set the limit in MB", "LIMIT");
    opts.optflag("h", "help", "Print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };

    let bit_depth: u16 = matches.opt_str("b").unwrap_or("16".to_string()).parse().unwrap_or(16);
    let sample_rate: u32 = matches.opt_str("s").unwrap_or("44100".to_string()).parse().unwrap_or(44100);
    let limit: u64 = matches.opt_str("l").unwrap_or("100".to_string()).parse().unwrap_or(100) * 1024 * 1024;

    if matches.free.len() < 2 {
        eprintln!("Error: Two arguments are required: input path and output path.");
        return;
    }

    let input_path = Path::new(&matches.free[0]);
    let output_path = Path::new(&matches.free[1]);

    if input_path == output_path {
        eprintln!("Error: Input and output paths cannot be the same.");
        return;
    }

    if input_path.is_dir() && output_path.is_file() || input_path.is_file() && output_path.is_dir() {
        eprintln!("Error: Input and output paths are not both files or directories.");
        return;
    }

    if input_path == output_path {
        eprintln!("Error: Input and output arguments cannot be the same. This prevents dangerous overwriting of files.");
        return;
    }

    let total_size = Arc::new(AtomicU64::new(0));
    
    if input_path.is_dir() {
        if !output_path.exists() {
            println!("Creating output path for you at: {}", output_path.display());
            fs::create_dir_all(output_path).expect("Failed to create output directory");
        }
        let pb = ProgressBar::new(limit);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"));
        process_directory(input_path, output_path, sample_rate, bit_depth, total_size, limit, &pb);
        pb.finish_with_message("Done!");
    } else {
        process_file(input_path, output_path, sample_rate, bit_depth);
        println!("Done processing: {}", input_path.display());
    }
    
}