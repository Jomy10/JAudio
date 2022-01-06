# JAudio

A low-level interface for working with audio files.

Note that this is a 0.x release and a currently only wave files are supported. in the future there will be support for other audioformats as well as converting fiels within the same format.

## Usage
Add the following to your `Cargo.toml` file:
```toml
JAudio = "0.1.1"
```

### Wave files
Currently, only wave files are implemented.

The following example read bytes from a file, then inserts these bytes in a `WaveFile` struct and finally saves the file
again.
```rust
use jaudio::wave_file::*;
use std::fs;

fn main() {
    // Reading a file to bytes and creating a `WaveFile` object
    // The file has 2 channels, a sample rate of 44100Hz and 16 bits per sample
    let mut bytes: Vec<u8> = WaveFile::file_to_data("audio.wav").unwrap();
    let mut wave = WaveFile::new(AudioFormat::PCM, 2, 44100, 16);
    
    // adding the audio from the file we read to wave
    wave.add_bytes(&mut bytes);
    
    // The path we want to save the file to
    let path = "file.wav";
    fs::write(path, wave.to_bytes()).unwrap();
}
```

## Contributions
Contributions ar very welcome, as I will probably not get around to completing all audio file formats.
Optimizations to code are also very welcome.


## Other info
This is a continuation of [Jonas' Utils](https://github.com/Jomy10/jonas-utils). Specifically, the [audio utils](https://github.com/Jomy10/jonas-utils/tree/master/src/main/java/be/jonaseveraert/util/audio).
