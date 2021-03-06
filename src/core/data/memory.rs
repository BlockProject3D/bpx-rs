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

use std::io::{Cursor, Read, Result, Seek, SeekFrom, Write};

use crate::{core::SectionData, utils::new_byte_buf};

pub struct InMemorySection
{
    byte_buf: Cursor<Vec<u8>>,
    cur_size: usize
}

impl InMemorySection
{
    pub fn new(initial: usize) -> InMemorySection
    {
        InMemorySection {
            byte_buf: new_byte_buf(initial),
            cur_size: 0
        }
    }
}

impl Read for InMemorySection
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    {
        self.byte_buf.read(buf)
    }
}

impl Write for InMemorySection
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>
    {
        let len = self.byte_buf.write(buf)?;
        if self.byte_buf.position() as usize >= self.cur_size {
            self.cur_size = self.byte_buf.position() as usize;
        }
        Ok(len)
    }

    fn flush(&mut self) -> Result<()>
    {
        self.byte_buf.flush()
    }
}

impl Seek for InMemorySection
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>
    {
        self.byte_buf.seek(pos)
    }
}

impl SectionData for InMemorySection
{
    fn load_in_memory(&mut self) -> Result<Vec<u8>>
    {
        return Ok(self.byte_buf.get_ref().clone());
    }

    fn size(&self) -> usize
    {
        self.cur_size
    }
}
