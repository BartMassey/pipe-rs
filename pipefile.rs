// Copyright © 2016 Bart Massey
// This work is made available under the "MIT License".
// Please see the file COPYING in this distribution for
// license terms.

//! Create a UNIX pipe with read and write endpoints.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate libc;

use std::fs::File;
use std::io::{Result, Error};
use libc::c_int;
use libc::pipe as raw_pipe;
use std::os::unix::io::FromRawFd;

/// A pipe has two ends and no middle.
pub struct PipeFile {
    /// Read from this end of the pipe.
    pub read_end: File,
    /// Write to this end of the pipe.
    pub write_end: File
}


/// Make a new pipe.
///
/// # Examples
///
/// ```
/// use std::io::{Write, BufReader, BufRead};
/// 
/// let p = pipefile::pipe().expect("couldn't create pipe");
/// // Start the write.
/// let mut writer = p.write_end;
/// let write_thread = std::thread::spawn(move || {
///     writer.write("hello world".as_bytes())
///           .expect("couldn't write message");
/// });
/// // Do the read.
/// let mut reader = BufReader::new(&p.read_end);
/// let mut message = String::new();
/// reader.read_line(&mut message)
///       .expect("couldn't read message");
/// // Clean up and check.
/// write_thread.join()
///       .expect("couldn't finish writer");
/// assert_eq!(message, "hello world");
/// ```
pub fn pipe() -> Result<PipeFile> {
    let mut fds: [c_int; 2] = [0; 2];
    match unsafe { raw_pipe((&mut fds).as_mut_ptr()) } {
        0 => Ok(PipeFile {
                    read_end: unsafe { FromRawFd::from_raw_fd(fds[0]) },
                    write_end: unsafe { FromRawFd::from_raw_fd(fds[1]) }
             }),
        _ => Err(Error::last_os_error())
    }
}
