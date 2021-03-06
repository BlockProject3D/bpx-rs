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

//! The BPX decoder.

use std::{
    collections::BTreeMap,
    io,
    io::{Read, Seek, Write}
};

use crate::{
    core::{
        compression::{
            Checksum,
            Crc32Checksum,
            Inflater,
            WeakChecksum,
            XzCompressionMethod,
            ZlibCompressionMethod
        },
        data::AutoSectionData,
        error::ReadError,
        header::{
            MainHeader,
            SectionHeader,
            Struct,
            FLAG_CHECK_CRC32,
            FLAG_CHECK_WEAK,
            FLAG_COMPRESS_XZ,
            FLAG_COMPRESS_ZLIB
        },
        section::{SectionEntry, SectionEntry1},
        DEFAULT_COMPRESSION_THRESHOLD
    },
    utils::ReadFill
};

const READ_BLOCK_SIZE: usize = 8192;

pub fn read_section_header_table<T: Read>(
    mut backend: &mut T,
    main_header: &MainHeader,
    checksum: u32
) -> Result<(u32, BTreeMap<u32, SectionEntry>), ReadError>
{
    let mut sections = BTreeMap::new();
    let mut final_checksum = checksum;
    let mut hdl: u32 = 0;

    for i in 0..main_header.section_num {
        let (checksum, header) = SectionHeader::read(&mut backend)?;
        final_checksum += checksum;
        sections.insert(
            hdl,
            SectionEntry {
                header,
                data: None,
                modified: false,
                index: i,
                entry1: SectionEntry1 {
                    flags: header.flags,
                    threshold: DEFAULT_COMPRESSION_THRESHOLD
                }
            }
        );
        hdl += 1;
    }
    if final_checksum != main_header.chksum {
        return Err(ReadError::Checksum(final_checksum, main_header.chksum));
    }
    Ok((hdl, sections))
}

pub fn load_section1<T: io::Read + io::Seek>(
    file: &mut T,
    section: &SectionHeader
) -> Result<AutoSectionData, ReadError>
{
    let mut data = AutoSectionData::new_with_size(section.size)?;
    data.seek(io::SeekFrom::Start(0))?;
    if section.flags & FLAG_CHECK_WEAK != 0 {
        let mut chksum = WeakChecksum::new();
        //TODO: Check
        load_section_checked(file, section, &mut data, &mut chksum)?;
        let v = chksum.finish();
        if v != section.chksum {
            return Err(ReadError::Checksum(v, section.chksum));
        }
    } else if section.flags & FLAG_CHECK_CRC32 != 0 {
        let mut chksum = Crc32Checksum::new();
        //TODO: Check
        load_section_checked(file, section, &mut data, &mut chksum)?;
        let v = chksum.finish();
        if v != section.chksum {
            return Err(ReadError::Checksum(v, section.chksum));
        }
    } else {
        let mut chksum = WeakChecksum::new();
        //TODO: Check
        load_section_checked(file, section, &mut data, &mut chksum)?;
    }
    data.seek(io::SeekFrom::Start(0))?;
    Ok(data)
}

fn load_section_checked<TBackend: io::Read + io::Seek, TWrite: Write, TChecksum: Checksum>(
    file: &mut TBackend,
    section: &SectionHeader,
    out: TWrite,
    chksum: &mut TChecksum
) -> Result<(), ReadError>
{
    if section.flags & FLAG_COMPRESS_XZ != 0 {
        load_section_compressed::<XzCompressionMethod, _, _, _>(file, section, out, chksum)?;
    } else if section.flags & FLAG_COMPRESS_ZLIB != 0 {
        load_section_compressed::<ZlibCompressionMethod, _, _, _>(file, section, out, chksum)?;
    } else {
        load_section_uncompressed(file, section, out, chksum)?;
    }
    Ok(())
}

fn load_section_uncompressed<TBackend: io::Read + io::Seek, TWrite: Write, TChecksum: Checksum>(
    bpx: &mut TBackend,
    header: &SectionHeader,
    mut output: TWrite,
    chksum: &mut TChecksum
) -> io::Result<()>
{
    let mut idata: [u8; READ_BLOCK_SIZE] = [0; READ_BLOCK_SIZE];
    let mut count: usize = 0;
    let mut remaining: usize = header.size as usize;

    bpx.seek(io::SeekFrom::Start(header.pointer))?;
    while count < header.size as usize {
        let res = bpx.read_fill(&mut idata[0..std::cmp::min(READ_BLOCK_SIZE, remaining)])?;
        output.write_all(&idata[0..res])?;
        chksum.push(&idata[0..res]);
        count += res;
        remaining -= res;
    }
    Ok(())
}

fn load_section_compressed<
    TMethod: Inflater,
    TBackend: io::Read + io::Seek,
    TWrite: Write,
    TChecksum: Checksum
>(
    bpx: &mut TBackend,
    header: &SectionHeader,
    output: TWrite,
    chksum: &mut TChecksum
) -> Result<(), ReadError>
{
    bpx.seek(io::SeekFrom::Start(header.pointer))?;
    XzCompressionMethod::inflate(bpx, output, header.csize as usize, chksum)?;
    Ok(())
}
