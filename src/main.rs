use std::fs::File;
use std::io::{self, Read, BufWriter};
use hound::{WavWriter, WavSpec};

fn read_input_file(file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_wav_header(output_path: &str, sample_rate: u32, num_channels: u16) -> Result<WavWriter<BufWriter<File>>, io::Error> {
    let spec = WavSpec {
        channels: num_channels,
        sample_rate,
        bits_per_sample: 8,
        sample_format: hound::SampleFormat::Int,
    };

    let writer = WavWriter::create(output_path, spec).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(writer)
}

fn write_output_file(mut header: WavWriter<BufWriter<File>>, input_data: &[u8]) -> Result<(), io::Error> {
    for &data in input_data {
        header.write_sample(data as i8).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    }
    header.finalize().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(())
}

fn main() {
    let input_path = "/opt/local/var/macports/build/_opt_local_var_macports_sources_rsync.macports.org_macports_release_tarballs_ports_lang_rust/rust/work/rustc-1.66.0-src/build/aarch64-apple-darwin/llvm/lib/libLLVMAArch64AsmParser.a";
    let output_path = "/Users/jby/output.wav";

    if let Ok(input_data) = read_input_file(input_path) {
        if let Ok(header) = write_wav_header(output_path, 44100, 1) {
            if let Ok(()) = write_output_file(header, &input_data) {
                println!("File conversion successful!");
            } else {
                eprintln!("Error writing output file.");
            }
        } else {
            eprintln!("Error creating WAV header.");
        }
    } else {
        eprintln!("Error reading input file.");
    }
}