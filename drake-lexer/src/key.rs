#[cfg(test)]
mod tests;

use somen::prelude::*;

use crate::utils::escaped_char;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key {
    Normal(String),
    Local(String),
    Builtin(String),
}

pub fn key<'a, I>() -> impl Parser<I, Output = Key> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        normal_key().map(Key::Normal),
        local_key().map(Key::Local),
        builtin_key().map(Key::Builtin),
    ))
    .expect("key")
}

pub fn normal_key<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((bare_key(), raw_key())).expect("normal key")
}

pub fn local_key<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    token('_').prefix(normal_key()).expect("local key")
}

pub fn builtin_key<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    token('@').prefix(normal_key()).expect("built-in key")
}

pub fn bare_key<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    (
        is(char::is_ascii_alphabetic).expect("[A-Za-z]").once(),
        is(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
            .expect("[0-9_A-Za-z]")
            .repeat(..),
    )
        .collect()
        .expect("bare key")
}

pub fn raw_key<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    escaped_char('}')
        .expect("character")
        .repeat(..)
        .collect()
        .between(tag("${"), token('}'))
        .expect("raw key")
}