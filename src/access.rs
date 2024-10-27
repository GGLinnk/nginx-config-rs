use std::net::IpAddr;

use combine::choice;
use combine::easy::Error;
use combine::error::StreamError;
use combine::Parser;

use ast::{Item, Source};
use helpers::{ident, semi, string};
use tokenizer::{Token, TokenStream};

fn parse_source<'a>(val: Token<'a>) -> Result<Source, Error<Token<'a>, Token<'a>>> {
    let value = val.value;
    if value == "all" {
        return Ok(Source::All);
    } else if value == "unix:" {
        return Ok(Source::Unix);
    }
    let mut pair = value.splitn(2, '/');
    let addr = pair.next().unwrap().parse::<IpAddr>()?;
    if let Some(net) = pair.next() {
        let subnet = net
            .parse::<u8>()
            .map_err(|e| Error::unexpected_message(format!("invalid subnet: {}", e)))?;
        return Ok(Source::Network(addr, subnet));
    } else {
        return Ok(Source::Ip(addr));
    }
}

fn allow<'a>() -> impl Parser<Output = Item, Input = TokenStream<'a>> {
    ident("allow")
        .with(string())
        .and_then(parse_source)
        .skip(semi())
        .map(Item::Allow)
}

fn deny<'a>() -> impl Parser<Output = Item, Input = TokenStream<'a>> {
    ident("deny")
        .with(string())
        .and_then(parse_source)
        .skip(semi())
        .map(Item::Deny)
}

pub fn directives<'a>() -> impl Parser<Output = Item, Input = TokenStream<'a>> {
    choice((allow(), deny()))
}
