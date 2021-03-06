// Copyright (c) 2021, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! BPXP utility functions.

use std::{
    fs::{metadata, read_dir, File},
    io::{Read, Seek, Write},
    path::{Path, PathBuf}
};

use crate::{
    package::{
        error::{EosContext, ReadError, WriteError},
        Package
    },
    strings::{get_name_from_dir_entry, get_name_from_path}
};

/// Packs a file or folder in a BPXP with the given virtual name.
///
/// **This function prints some information to standard output as a way
/// to debug data compression issues unless the `debug-log` feature
/// is disabled.**
///
/// # Arguments
///
/// * `package`: the [Package](crate::package::Package) to use.
/// * `vname`: the virtual name for the root source path.
/// * `source`: the source [Path](std::path::Path) to pack.
///
/// returns: Result<(), Error>
///
/// # Errors
///
/// A [WriteError](crate::package::error::WriteError) is returned if some objects could not be packed.
pub fn pack_file_vname<T: Write + Seek>(
    package: &mut Package<T>,
    vname: &str,
    source: &Path
) -> Result<(), WriteError>
{
    let md = metadata(source)?;
    if md.is_file() {
        #[cfg(feature = "debug-log")]
        println!("Writing file {} with {} byte(s)", vname, md.len());
        let mut fle = File::open(source)?;
        package.pack(vname, &mut fle)?;
    } else {
        let entries = read_dir(source)?;
        for rentry in entries {
            let entry = rentry?;
            let mut s = String::from(vname);
            s.push('/');
            s.push_str(&get_name_from_dir_entry(&entry)?);
            pack_file_vname(package, &s, &entry.path())?;
        }
    }
    Ok(())
}

/// Packs a file or folder in a BPXP, automatically computing
/// the virtual name from the source path file name.
///
/// **This function prints some information to standard output as a way
/// to debug data compression issues unless the `debug-log` feature
/// is disabled.**
///
/// # Arguments
///
/// * `package`: the [Package](crate::package::Package) to use.
/// * `source`: the source [Path](std::path::Path) to pack.
///
/// returns: Result<(), Error>
///
/// # Errors
///
/// A [WriteError](crate::package::error::WriteError) is returned if some objects could not be packed.
pub fn pack_file<T: Write + Seek>(package: &mut Package<T>, source: &Path)
    -> Result<(), WriteError>
{
    let str = get_name_from_path(source)?;
    pack_file_vname(package, str, source)
}

/// Unpacks a BPXP.
///
/// **This function prints some information to standard output as a way
/// to debug a broken or incorrectly packed BPXP unless the `debug-log`
/// feature is disabled.**
///
/// # Arguments
///
/// * `package`: the [Package](crate::package::Package) to use.
/// * `target`: the target [Path](std::path::Path) to extract the content to.
///
/// returns: Result<(), Error>
///
/// # Errors
///
/// An [ReadError](crate::package::error::ReadError) is returned if some objects could not be unpacked.
pub fn unpack<T: Read + Seek>(package: &mut Package<T>, target: &Path) -> Result<(), ReadError>
{
    for mut v in package.objects()? {
        let size = v.size();
        let path = v.load_name()?;
        if path.is_empty() {
            return Err(ReadError::BlankString);
        }
        #[cfg(feature = "debug-log")]
        println!("Reading {} with {} byte(s)...", path, size);
        let mut dest = PathBuf::new();
        dest.push(target);
        dest.push(Path::new(path));
        if let Some(v) = dest.parent() {
            std::fs::create_dir_all(v)?;
        }
        let f = File::create(dest)?;
        let s = v.unpack(f)?;
        if size != s {
            return Err(ReadError::Eos(EosContext::Object));
        }
    }
    Ok(())
}
