// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use nostr::parser::{self, Token};
use uniffi::{Enum, Object};

use crate::protocol::nips::nip21::Nip21Enum;

#[derive(Enum)]
pub enum NostrParserToken {
    /// Nostr URI
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/21.md>
    Nostr(Nip21Enum),
    /// Url
    Url(String),
    /// Hashtag
    Hashtag(String),
    /// Other text
    ///
    /// Spaces at the beginning or end of a text are parsed as [`Token::Whitespace`].
    Text(String),
    /// Line break
    LineBreak,
    /// A whitespace
    Whitespace,
}

impl<'a> From<Token<'a>> for NostrParserToken {
    fn from(token: Token<'a>) -> Self {
        match token {
            Token::Nostr(uri) => Self::Nostr(uri.into()),
            Token::Url(url) => Self::Url(url.to_string()),
            Token::Hashtag(hashtag) => Self::Hashtag(hashtag.to_string()),
            Token::Text(text) => Self::Text(text.to_string()),
            Token::LineBreak => Self::LineBreak,
            Token::Whitespace => Self::Whitespace,
        }
    }
}

/// Nostr parser
#[derive(Object)]
pub struct NostrParser {
    inner: parser::NostrParser,
}

#[uniffi::export]
impl NostrParser {
    /// Construct a new nostr parser
    ///
    /// It's suggested to construct this once and reuse it, to avoid regex re-compilation.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: parser::NostrParser::new(),
        }
    }

    /// Parse text into tokens
    pub fn parse(&self, text: &str) -> Vec<NostrParserToken> {
        self.inner.parse(text).map(NostrParserToken::from).collect()
    }
}
