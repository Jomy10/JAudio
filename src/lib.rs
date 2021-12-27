#![doc(html_logo_url = "path_to_logo", html_favicon_url = "path_to_favicon")]
#![deny(missing_docs)]

//! A low-level package for working with audio files
//!
//! Currently implemented:
//! - [Wave Files](wave_file/WaveFile)

/// For building and reading wave audio files.
///
/// ## Valuable resources
/// [*WAVE PCM soundfile format*. Stanford.edu (Dec 10, 2008). (Wayback machine link)](https://web.archive.org/web/20081210162727/https://ccrma.stanford.edu/CCRMA/Courses/422/projects/WaveFormat/")
pub mod wave_file;