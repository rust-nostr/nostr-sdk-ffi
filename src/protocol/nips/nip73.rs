// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use nostr::Url;
use nostr::nips::nip73;
use uniffi::Enum;

use crate::error::NostrSdkError;

/// NIP-73 kinds
#[derive(Enum)]
pub enum Nip73Kind {
    /// URLs kind "web"
    Url,
    /// Books kind "isbn"
    Book,
    /// Geohashes kind "geo"
    Geohash,
    /// Movies kind "isan"
    Movie,
    /// Papers kind "doi"
    Paper,
    /// Hashtags kind "#"
    Hashtag,
    /// Podcast feeds kind "podcast:guid"
    PodcastFeed,
    /// Podcast episodes kind "podcast:item:guid"
    PodcastEpisode,
    /// Podcast publishers kind "podcast:publisher:guid"
    PodcastPublisher,
    /// Blockchain transaction kind "<blockchain>:tx"
    BlockchainTransaction { tx: String },
    /// Blockchain address kind "<blockchain>:address"
    BlockchainAddress { address: String },
}

impl From<nip73::Nip73Kind> for Nip73Kind {
    fn from(value: nip73::Nip73Kind) -> Self {
        match value {
            nip73::Nip73Kind::Url => Self::Url,
            nip73::Nip73Kind::Book => Self::Book,
            nip73::Nip73Kind::Geohashe => Self::Geohash,
            nip73::Nip73Kind::Movie => Self::Movie,
            nip73::Nip73Kind::Paper => Self::Paper,
            nip73::Nip73Kind::Hashtag => Self::Hashtag,
            nip73::Nip73Kind::PodcastFeed => Self::PodcastFeed,
            nip73::Nip73Kind::PodcastEpisode => Self::PodcastEpisode,
            nip73::Nip73Kind::PodcastPublisher => Self::PodcastPublisher,
            nip73::Nip73Kind::BlockchainTransaction(tx) => Self::BlockchainTransaction { tx },
            nip73::Nip73Kind::BlockchainAddress(address) => Self::BlockchainAddress { address },
        }
    }
}

impl From<Nip73Kind> for nip73::Nip73Kind {
    fn from(value: Nip73Kind) -> Self {
        match value {
            Nip73Kind::Url => Self::Url,
            Nip73Kind::Book => Self::Book,
            Nip73Kind::Geohash => Self::Geohashe,
            Nip73Kind::Movie => Self::Movie,
            Nip73Kind::Paper => Self::Paper,
            Nip73Kind::Hashtag => Self::Hashtag,
            Nip73Kind::PodcastFeed => Self::PodcastFeed,
            Nip73Kind::PodcastEpisode => Self::PodcastEpisode,
            Nip73Kind::PodcastPublisher => Self::PodcastPublisher,
            Nip73Kind::BlockchainTransaction { tx } => Self::BlockchainTransaction(tx),
            Nip73Kind::BlockchainAddress { address } => Self::BlockchainAddress(address),
        }
    }
}

/// External Content ID
#[derive(Enum)]
pub enum ExternalContentId {
    /// URL
    Url(String),
    /// Hashtag
    Hashtag(String),
    /// Geohash
    Geohash(String),
    /// Book
    Book(String),
    /// Podcast Feed
    PodcastFeed(String),
    /// Podcast Episode
    PodcastEpisode(String),
    /// Podcast Publisher
    PodcastPublisher(String),
    /// Movie
    Movie(String),
    /// Paper
    Paper(String),
    /// Blockchain Transaction
    BlockchainTransaction {
        /// The blockchain name (e.g., "bitcoin", "ethereum")
        chain: String,
        /// A lower case hex transaction id
        transaction_hash: String,
        /// The chain id if one is required
        chain_id: Option<String>,
    },
    /// Blockchain Address
    BlockchainAddress {
        /// The blockchain name (e.g., "bitcoin", "ethereum")
        chain: String,
        /// The on-chain address
        address: String,
        /// The chain id if one is required
        chain_id: Option<String>,
    },
}

impl From<nip73::ExternalContentId> for ExternalContentId {
    fn from(content: nip73::ExternalContentId) -> Self {
        match content {
            nip73::ExternalContentId::Url(url) => Self::Url(url.to_string()),
            nip73::ExternalContentId::Hashtag(val) => Self::Hashtag(val),
            nip73::ExternalContentId::Geohash(val) => Self::Geohash(val),
            nip73::ExternalContentId::Book(val) => Self::Book(val),
            nip73::ExternalContentId::PodcastFeed(val) => Self::PodcastFeed(val),
            nip73::ExternalContentId::PodcastEpisode(val) => Self::PodcastEpisode(val),
            nip73::ExternalContentId::PodcastPublisher(val) => Self::PodcastPublisher(val),
            nip73::ExternalContentId::Movie(val) => Self::Movie(val),
            nip73::ExternalContentId::Paper(val) => Self::Paper(val),
            nip73::ExternalContentId::BlockchainTransaction {
                chain,
                transaction_hash,
                chain_id,
            } => Self::BlockchainTransaction {
                chain,
                transaction_hash,
                chain_id,
            },
            nip73::ExternalContentId::BlockchainAddress {
                chain,
                address,
                chain_id,
            } => Self::BlockchainAddress {
                chain,
                address,
                chain_id,
            },
        }
    }
}

impl TryFrom<ExternalContentId> for nip73::ExternalContentId {
    type Error = NostrSdkError;

    fn try_from(content: ExternalContentId) -> Result<Self, Self::Error> {
        Ok(match content {
            ExternalContentId::Url(url) => Self::Url(Url::parse(&url)?),
            ExternalContentId::Hashtag(val) => Self::Hashtag(val),
            ExternalContentId::Geohash(val) => Self::Geohash(val),
            ExternalContentId::Book(val) => Self::Book(val),
            ExternalContentId::PodcastFeed(val) => Self::PodcastFeed(val),
            ExternalContentId::PodcastEpisode(val) => Self::PodcastEpisode(val),
            ExternalContentId::PodcastPublisher(val) => Self::PodcastPublisher(val),
            ExternalContentId::Movie(val) => Self::Movie(val),
            ExternalContentId::Paper(val) => Self::Paper(val),
            ExternalContentId::BlockchainTransaction {
                chain,
                transaction_hash,
                chain_id,
            } => Self::BlockchainTransaction {
                chain,
                transaction_hash,
                chain_id,
            },
            ExternalContentId::BlockchainAddress {
                chain,
                address,
                chain_id,
            } => Self::BlockchainAddress {
                chain,
                address,
                chain_id,
            },
        })
    }
}
