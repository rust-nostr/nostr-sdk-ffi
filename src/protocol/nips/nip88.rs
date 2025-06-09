// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use nostr::nips::nip88;
use uniffi::{Enum, Record};

#[derive(Enum)]
pub enum PollType {
    /// Single choice
    SingleChoice,
    /// Multiple choice
    MultipleChoice,
}

impl From<PollType> for nip88::PollType {
    fn from(value: PollType) -> Self {
        match value {
            PollType::SingleChoice => Self::SingleChoice,
            PollType::MultipleChoice => Self::MultipleChoice,
        }
    }
}

impl From<nip88::PollType> for PollType {
    fn from(value: nip88::PollType) -> Self {
        match value {
            nip88::PollType::SingleChoice => Self::SingleChoice,
            nip88::PollType::MultipleChoice => Self::MultipleChoice,
        }
    }
}

#[derive(Record)]
pub struct PollOption {
    /// Option ID
    pub id: String,
    /// Option label
    pub text: String,
}

impl From<PollOption> for nip88::PollOption {
    fn from(opt: PollOption) -> Self {
        Self {
            id: opt.id,
            text: opt.text,
        }
    }
}

impl From<nip88::PollOption> for PollOption {
    fn from(opt: nip88::PollOption) -> Self {
        Self {
            id: opt.id,
            text: opt.text,
        }
    }
}

// TODO: finish to expose NIP88
// #[derive(Record)]
// pub struct Poll {
//     /// Poll title
//     pub title: String,
//     /// Poll type
//     pub poll_type: PollType,
//     /// Poll options
//     pub options: Vec<PollOption>,
//     /// Relay URLs
//     pub relays: Vec<String>,
//     /// Optionally, when the poll ends
//     pub ends_at: Option<Arc<Timestamp>>,
// }
