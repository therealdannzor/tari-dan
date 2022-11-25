// Copyright 2021. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use tari_comms::types::CommsPublicKey;
use tari_utilities::ByteArray;

pub trait NodeAddressable: Eq + Hash + Clone + Debug + Send + Sync + Display {
    fn zero() -> Self;
    fn as_bytes(&self) -> &[u8];
}

impl NodeAddressable for String {
    fn zero() -> Self {
        "".to_string()
    }

    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl NodeAddressable for &str {
    fn zero() -> Self {
        ""
    }

    fn as_bytes(&self) -> &[u8] {
        str::as_bytes(self)
    }

    // fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
    //     Some(std::str::from_utf8(bytes).ok()?)
    // }
}

impl NodeAddressable for CommsPublicKey {
    fn zero() -> Self {
        CommsPublicKey::default()
    }

    fn as_bytes(&self) -> &[u8] {
        <CommsPublicKey as ByteArray>::as_bytes(self)
    }

    // fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
    //     Some(CommsPublicKey::from_bytes(bytes).ok()?)
    // }
}
