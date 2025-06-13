// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use nostr::parser::{self, Token};
use uniffi::{Enum, Object, Record};

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

/// Nostr parser options
#[derive(Record)]
pub struct NostrParserOptions {
    /// Parse nostr URIs
    #[uniffi(default = true)]
    pub nostr_uris: bool,
    /// Parse URLs
    #[uniffi(default = true)]
    pub urls: bool,
    /// Parse hashtags
    #[uniffi(default = true)]
    pub hashtags: bool,
    /// Parse text, line breaks and whitespaces
    #[uniffi(default = true)]
    pub text: bool,
}

impl From<NostrParserOptions> for parser::NostrParserOptions {
    fn from(opts: NostrParserOptions) -> Self {
        Self {
            nostr_uris: opts.nostr_uris,
            urls: opts.urls,
            hashtags: opts.hashtags,
            text: opts.text,
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
    #[uniffi::method(default(opts = None))]
    pub fn parse(&self, text: &str, opts: Option<NostrParserOptions>) -> Vec<NostrParserToken> {
        let opts: parser::NostrParserOptions = opts.map(|o| o.into()).unwrap_or_default();
        self.inner
            .parse(text)
            .opts(opts)
            .map(NostrParserToken::from)
            .collect()
    }
}
