// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use nostr::key;
use nostr::nips::nip06::FromMnemonic;

use crate::error::Result;
use crate::protocol::key::Keys;

#[uniffi::export]
impl Keys {
    /// Derive keys from BIP-39 mnemonics (ENGLISH wordlist).
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/06.md>
    #[uniffi::constructor(default(passphrase = None, account = None, typ = None, index = None))]
    pub fn from_mnemonic(
        mnemonic: String,
        passphrase: Option<String>,
        account: Option<u32>,
        typ: Option<u32>,
        index: Option<u32>,
    ) -> Result<Self> {
        let key: key::Keys =
            key::Keys::from_mnemonic_advanced(mnemonic, passphrase, account, typ, index)?;
        Ok(Self::from(key))
    }
}
