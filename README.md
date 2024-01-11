# bend

bend is a small command-line application for 'bending' data into audio. It used to be written in Nim and can be found [here](https://github.com/jamesb93/bend-nim), though the Rust version here supercedes the old Nim one.

This project stems from my own use of [SoX](http://sox.sourceforge.net) to _bend_ raw data into audio with the command:

`sox -r 44100 -b 8 -c 1 -e unsigned-integer input.raw output.wav`

I wanted to write a small application that could implement this functionality but without the overhead of having to install SoX, a tool suited to performing a number of other DSP tasks that I didn't need. This was also a fun project to learn more about the structure of WAVE audio as well as experimenting with the Rust language.

[Francesco Cameli](github.com/vitreo12) is a significant contributor, particularly in optimising the code to be fast (from 100ms to less than 5ms!). I'd like to also thank him for his patience and guidance on all things Nim and Rust

# Build The Tool Yourself

To build `bend` yourself you need a valid `rust` toolchain greater than version 1.75.0. A good way to install the `rust` compiler is by using `rustup`, which can be found at the [rustup website](https://rustup.rs/).

Once you have `rustc` installed (the `rust` compiler), you can `cd` to this repository and cast the following spell:

`cargo  install --path .`

You now have the `bend` binary in your path which can be invoked. See the Usage section.

# Usage

`bend` can process individual files, or process directories of files.

## Examples

### Convert a single file
`bend ~/posts.json ~/audio_output.wav`

### Convert a whole directory up to 100MB (default)
`bend ~/Documents ~/batch_output`

### Convert a whole directory up to 1GB
`bend ~/Documents ~/batch_output -l 1000`

### Convert a whole directory up to 2GB with a bit depth of 16 at 16000Hz
`bend ~/Documents ~/batch_output -l 2000 -s 16000`


## Help and issues

If you have any issues or questions please raise one on the github!
