use chumsky::Stream;
use logos::{Lexer, Logos, Source, Span};

/// Converts a Logos lexer into a Chumsky stream
pub fn lexer_to_stream<'source, Token, Context>(
    lexer: Lexer<'source, Token>,
    context: Context,
) -> Stream<
    'source,
    Token,
    (Context, Span),
    impl Iterator<Item = (Token, (Context, Span))> + 'source,
>
    where
        Token: Logos<'source> + 'source,
        Context: Clone + 'source
{
    let length = lexer.source().len();
    Stream::from_iter(
        (context.clone(), length..length + 1),
        lexer
            .spanned()
            .map(move |(token, span)| (token, (context.clone(), span))),
    )
}