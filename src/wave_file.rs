use std::error::Error;
use std::fs;

/// The possible audio formats for [WaveFile](wave_file/WaveFile)
pub enum AudioFormat {
    /// PCM format
    PCM
}

impl AudioFormat {
    fn get_val(&self) -> u32 {
        match self {
            PCM => 1
        }
    }
}

/// Stores data resembling a WAVE file.
///
/// # Parameters
/// - `audio_format`: audio format of the wav file.
/// - `num_channels`: the number of channels the file will have (mono or stereo)
/// - `sample_rate`: The sample rate of tha wave file in Hz (e.g. 22050, 44100, ...)
/// - `bits_per_sample`: The amount of bits per sample. If 16 bits, the audio sample will contain 2
/// bytes per channel. (e.g. 8, 16, ...). Important to take into account when adding bytes to the WaveFile!
/// - `byte_rate`
/// - `block_align`
/// - `chunks`: contains all audio data
///
/// # Examples
/// ```rust
/// # use jaudio::wave_file::*;
/// # use std::fs;
/// #
/// # fn main() {
///  // The file we are reading here has 2 channels, a sample rate of 44100Hz and 16 bits per sample
///  let mut bytes = WaveFile::file_to_data("audio.wav").unwrap();
///  let mut wave = WaveFile::new(AudioFormat::PCM, 2, 44100, 16);
///
///  // adding the audio from the file we read to wave
///  wave.add_bytes(&mut bytes);
///
///  // The path we want to save the file to
///  let path = "file.wav";
///  fs::write(path, wave.to_bytes()).unwrap();
/// # }
/// ```
pub struct WaveFile {
    audio_format: u32,
    num_channels: u32,
    sample_rate: u32,
    bits_per_sample: u32,
    // Non-public
    byte_rate: u32,
    block_align: u32,
    audio_byte_data: Vec<u8>, // Vector of bytes
}

// New
impl WaveFile {
    /// ## Parameters
    /// - `audio_format`: audio format of the wav file.
    /// - `num_channels`: the number of channels the file will have (mono or stereo)
    /// - `sample_rate`: The sample rate of tha wave file in Hz (e.g. 22050, 44100, ...)
    /// - `bits_per_sample`: The amount of bits per sample. If 16 bits, the audio sample will contain 2
    /// bytes per channel. (e.g. 8, 16, ...). Important to take into account when adding bytes to the WaveFile!
    pub fn new(
        audio_format: AudioFormat,
        num_channels: u32,
        sample_rate: u32,
        bits_per_sample: u32
    ) -> WaveFile
    {
        // Subchunck 1 calculations
        let byte_rate = sample_rate * num_channels * (bits_per_sample / 8);
        let block_align = num_channels * (bits_per_sample / 8);
        
        let audio_format: u32 = audio_format.get_val();
        
        // Return new WaveFile
        WaveFile{ audio_format, num_channels, sample_rate, bits_per_sample, byte_rate, block_align, audio_byte_data: Vec::new() }
    }
}

// Functions on instance
impl WaveFile {
    /// Adds audio data to `WaveFile` from bytes.
    ///
    /// See also [WaveFormat](https://web.archive.org/web/20081210162727/https://ccrma.stanford.edu/CCRMA/Courses/422/projects/WaveFormat/)
    /// for the structure of the "Data" part of a wave file.
    ///
    /// # Panics
    /// If the given `bytes` do not conform to the sample size in bytes. So, if it is not
    /// divisible by `block_align`.
    ///
    /// # Parameters
    /// - bytes: will be moved to `audio_byte_data` of `WaveFile`, leaving `bytes` empty.
    pub fn add_bytes(&mut self, bytes: &mut Vec<u8>) {
        // Ex. if each sample is 2 bytes long -> don't allow add_bytes methodif the amount of bytes is not % by 2
        if bytes.len() as u32 % self.block_align != 0 {
            panic!("Trying to add a chunck that does not fit evenly; this would cause un-aligned blocks.");
        }
        
        self.audio_byte_data.append(bytes);
    }
    
    /// Returns the audio data
    pub fn bytes(&mut self) -> &mut Vec<u8> {
        &mut self.audio_byte_data
    }
    
    /// Returns the block align, can be used to check if the bytes passed to `to_bytes` is divisible
    /// by `block_align`.
    pub fn block_align(&self) -> u32 {
        self.block_align
    }
    
    /// A byte representation of the `WaveFile`.
    ///
    /// Can be used to write to a file.
    pub fn to_bytes(&self) -> Vec<u8> {
        let subchunk1_size: u32 = 16; // If longer than 16 -> support ExtraPrams field (but necessary?)
        let chunk_id: String = String::from("RIFF");
        let format: String = String::from("WAVE");
        // Sub chunk 1 (fmt)
        let subchunk1_id: String = String::from("fmt ");
        // Stores: suchunk1_size; audio_format, num_channels, sample_rate, byte_rate, block_align, bits_per_sample
    
        // Sub Chunk 2 (data)
        let subchunk2_id: String = String::from("data");
        // stores: subchunk2_size
        
        // Subchunk 2 calculations
        let num_bytes_in_data: u32 = self.audio_byte_data.len() as u32;
        
        let num_samples = num_bytes_in_data / (2 * self.num_channels);
    
        let subchunk2_size = num_samples * self.num_channels * (self.bits_per_sample / 8);
        
        // chunk calculation
        let chunk_size = 4 + (8 + subchunk1_size) + (8 + subchunk2_size);
        
        // Convert to bytes //
        // Chunk descriptor
        let chunk_id: &[u8] = chunk_id.as_bytes();
        let chunk_size = chunk_size.to_le_bytes(); // in little endian (le)
        let format = format.as_bytes();
        
        // fmt subchunk
        // TODO: something more efficient for 2 byte long arrays
        let subchunk1_id = subchunk1_id.as_bytes();
        let subchunk1_size = subchunk1_size.to_le_bytes(); // this has 4 bytes
        let mut i = 0;
        let audio_format: [u8; 2] = self.audio_format.to_le_bytes().into_iter().filter(|v| {
            if i < 2 { i += 1; true } else { false }
        }).collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(); // this has 2
        let mut i = 0;
        let num_channels: [u8; 2] = self.num_channels.to_le_bytes().into_iter().filter(|v| {
            if i < 2 { i += 1; true } else { false }
        })
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap();
        let sample_rate = self.sample_rate.to_le_bytes();
        let byte_rate = self.byte_rate.to_le_bytes();
        let mut i = 0;
        let block_align: [u8; 2] = self.block_align.to_le_bytes().into_iter().filter(|v| {
            if i < 2 { i += 1; true } else { false }
        })
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap();
        let mut i = 0;
        let bits_per_sample: [u8; 2] = self.bits_per_sample.to_le_bytes().into_iter().filter(|v| {
            if i < 2 { i += 1; true } else { false }
        })
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap();
        
        // data subchunk
        let subchunk2_id = subchunk2_id.as_bytes();
        let subchunk2_size = subchunk2_size.to_le_bytes();
        // data = self.audio_byte_data
        
        let mut data = Vec::new();
        // head
        data.extend_from_slice(chunk_id);
        data.extend(chunk_size);
        data.extend(format);
        // subchunk 1
        data.extend(subchunk1_id);
        data.extend(subchunk1_size);
        data.extend(audio_format);
        data.extend(num_channels);
        data.extend(sample_rate);
        data.extend(byte_rate);
        data.extend(block_align);
        data.extend(bits_per_sample);
        
        // subchunk 2
        data.extend(subchunk2_id);
        data.extend(subchunk2_size);
        data.extend(&self.audio_byte_data);
        
        data
    }
}

// Static functions
impl WaveFile {
    /// Returns only the data part of a wave file.
    ///
    /// This method can only read PCM format (for the moment)
    ///
    /// # Errors (from [fs::read()](std::fs::read))
    /// This function will return an error if path does not already exist.
    /// Other errors may also be returned according to [OpenOptions::open](std::fs::OpenOptions::open).
    ///
    /// It will also return an error if it encounters while reading an error of a kind other than
    /// [io::ErrorKind::Interrupted](std::io::error::ErrorKind::Interrupted).
    pub fn file_to_data(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let file_content = fs::read(file_path)?;
        Ok(file_content[44..].to_vec())
    }
}