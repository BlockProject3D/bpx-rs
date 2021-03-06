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

#![warn(missing_docs)]

//! This library is the official implementation for the [BPX](https://gitlab.com/bp3d/bpx/bpx/-/blob/rev2/BPX_Format.pdf) container format.

pub mod core;
mod garraylen;
pub mod macros;
pub mod utils;

#[cfg(feature = "table")]
pub mod table;

#[cfg(feature = "sd")]
pub mod sd;

#[cfg(feature = "strings")]
pub mod strings;

#[cfg(feature = "package")]
pub mod package;

#[cfg(feature = "shader")]
pub mod shader;

/// Represents a pointer to a section.
///
/// *Allows indirect access to a given section instead of sharing mutable references in user code.*
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Handle(u32);

impl Handle
{
    /// Constructs a Handle from a raw u32.
    ///
    /// # Arguments
    ///
    /// * `raw`: the raw key.
    ///
    /// returns: Handle
    ///
    /// # Safety
    ///
    /// You must ensure the raw key is a valid key. Failure to do so could panic bpx::core::Container.
    pub unsafe fn from_raw(raw: u32) -> Self
    {
        Self(raw)
    }

    /// Extracts the raw key from this Handle.
    pub fn into_raw(self) -> u32
    {
        self.0
    }
}
