use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use std::iter::Peekable;

use super::Error;

pub struct Prop {
    pub name: Ident,
    pub value: Option<Vec<TokenTree>>,
}

impl ToTokens for Prop {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        tokens.extend(quote! {: });
        if let Some(value) = &self.value {
            value.iter().for_each(|value| value.to_tokens(tokens));
        } else {
            tokens.extend(quote! {true});
        }
        tokens.extend(quote! {,});
    }
}

fn parse_prop(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Prop, Error> {
    let name = match input.next() {
        Some(TokenTree::Ident(ident)) => ident,
        token => return Err(Error::ExpectedPropertyName(token.unwrap())),
    };

    match input.peek() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
            input.next();
        }
        _ => return Ok(Prop { name, value: None }),
    };

    let mut value = vec![match input.peek() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
            let punct = punct.clone();

            input.next();

            return Err(Error::ExpectedPropertyValue(TokenTree::Punct(punct)));
        }
        Some(TokenTree::Punct(punct)) if punct.as_char() == '!' => TokenTree::Ident(name.clone()),
        Some(_) => input.next().unwrap(),
        None => todo!("Unexpectedly reached end of macro input."),
    }];

    match input.peek() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '!' => {
            value.push(input.next().unwrap());
            value.push(input.next().unwrap());
        }
        _ => {}
    };

    Ok(Prop {
        name,
        value: Some(value),
    })
}

fn parse_dots(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), Error> {
    for _ in 0..3 {
        match input.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {}
            token => return Err(Error::ExpectedDot(token.unwrap())),
        }
    }

    Ok(())
}

pub struct TagOpen {
    pub is_builtin: bool,
    pub path: Path,
    pub props: Vec<Prop>,
    pub is_default: bool,
    pub is_self_closing: bool,
}

pub struct TagClose {
    pub is_builtin: bool,
    pub path: Path,
}

pub struct Colon2([Punct; 2]);

impl ToTokens for Colon2 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0[0].to_tokens(tokens);
        self.0[1].to_tokens(tokens);
    }
}

pub struct GenericArguments {
    pub left_token: Punct,
    pub right_token: Punct,
    pub colon2_token: Colon2,
    pub args: Vec<Path>,
    pub commas: Vec<Punct>,
}

impl ToTokens for GenericArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.colon2_token.to_tokens(tokens);
        self.left_token.to_tokens(tokens);

        let mut commas = self.commas.iter();

        for arg in &self.args {
            arg.to_tokens(tokens);

            if let Some(comma) = commas.next() {
                comma.to_tokens(tokens);
            }
        }

        self.right_token.to_tokens(tokens);
    }
}

pub enum PathArguments {
    None,
    AngleBracketed(GenericArguments),
    Parenthesized(GenericArguments),
}

impl ToTokens for PathArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PathArguments::AngleBracketed(args) | PathArguments::Parenthesized(args) => {
                args.to_tokens(tokens)
            }
            PathArguments::None => {}
        }
    }
}

pub struct PathSegment {
    pub ident: Ident,
    pub arguments: PathArguments,
}

impl ToTokens for PathSegment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.arguments.to_tokens(tokens);
    }
}

pub struct Path {
    pub parts: Vec<PathSegment>,
    pub puncts: Vec<Colon2>,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stream = self.to_token_stream();
        stream.fmt(f)
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut puncts = self.puncts.iter();

        for part in &self.parts {
            part.to_tokens(tokens);

            if let Some(punct) = puncts.next() {
                punct.to_tokens(tokens);
            }
        }
    }
}

fn parse_args(
    input: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<(Vec<Path>, Vec<Punct>), Error> {
    let mut paths = vec![];
    let mut commas = vec![];

    loop {
        paths.push(parse_path(input)?);

        match input.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {
                let punct = punct.clone();
                let _ = input.next();
                commas.push(punct)
            }
            _ => break,
        }
    }

    Ok((paths, commas))
}

fn parse_path(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Path, Error> {
    let mut parts = vec![];
    let mut puncts = vec![];

    'segments: loop {
        let ident = match input.next() {
            Some(TokenTree::Ident(ident)) => ident,
            token => return Err(Error::ExpectedIdentifier(token.unwrap())),
        };

        let mut arguments = PathArguments::None;

        for i in 0..2 {
            let colon1 = match input.peek() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => {
                    let punct = punct.clone();
                    input.next();
                    punct
                }
                _ => {
                    parts.push(PathSegment { ident, arguments });
                    break 'segments;
                }
            };

            let colon2 = match input.next() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => punct,
                token => return Err(Error::ExpectedColon(token.unwrap())),
            };

            if i != 0 {
                puncts.push(Colon2([colon1, colon2]));
                break;
            }

            match input.peek() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == '<' => {
                    let colon2_token = Colon2([colon1, colon2]);
                    let left_token = match input.next() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '<' => punct,
                        token => return Err(Error::ExpectedOpeningAngle(token.unwrap())),
                    };
                    let (args, commas) = parse_args(input)?;
                    let right_token = match input.next() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => punct,
                        token => return Err(Error::ExpectedClosingAngle(token.unwrap())),
                    };
                    arguments = PathArguments::AngleBracketed(GenericArguments {
                        colon2_token,
                        left_token,
                        args,
                        commas,
                        right_token,
                    });
                }
                Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                    let group = match input.next() {
                        Some(TokenTree::Group(group)) => group,
                        _ => unreachable!(),
                    };
                    let colon2_token = Colon2([colon1, colon2]);
                    let mut left_token = Punct::new('(', Spacing::Joint);
                    left_token.set_span(group.span_open());
                    let (args, commas) = parse_args(&mut group.stream().into_iter().peekable())?;
                    let mut right_token = Punct::new(')', Spacing::Joint);
                    right_token.set_span(group.span_close());
                    arguments = PathArguments::Parenthesized(GenericArguments {
                        colon2_token,
                        left_token,
                        args,
                        commas,
                        right_token,
                    })
                }
                _ => {
                    puncts.push(Colon2([colon1, colon2]));
                    break;
                }
            }
        }

        parts.push(PathSegment { ident, arguments })
    }

    Ok(Path { parts, puncts })
}

pub fn parse_token(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Token, Error> {
    match input.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '<' => {}
        Some(TokenTree::Literal(literal)) => return Ok(Token::Literal(literal)),
        Some(TokenTree::Group(group)) => return Ok(Token::Group(group)),
        token => return Err(Error::ExpectedOpeningAngle(token.unwrap())),
    }

    let mut is_builtin = false;
    let mut is_closing = false;

    loop {
        // We start by matching the builtin operator.
        match input.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '~' && !is_builtin => {
                is_builtin = true;
                input.next();
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '/' && !is_closing => {
                is_closing = true;
                input.next();
            }
            Some(TokenTree::Ident(_)) => break,
            token => return Err(Error::ExpectedPreamble(token.cloned().unwrap())),
        }
    }

    let path = parse_path(input)?;

    if is_closing {
        match input.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                return Ok(Token::TagClose(TagClose { is_builtin, path }))
            }
            token => return Err(Error::ExpectedClosingAngle(token.unwrap())),
        }
    }

    let mut props = vec![];

    let mut is_default = false;

    let mut is_self_closing = false;

    loop {
        match input.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {
                parse_dots(input)?;

                is_default = true;

                match input.next() {
                    Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => break,
                    Some(TokenTree::Punct(punct)) if punct.as_char() == '/' => {
                        is_self_closing = true;

                        match input.next() {
                            Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => break,
                            token => return Err(Error::ExpectedClosingAngle(token.unwrap())),
                        }
                    }
                    token => return Err(Error::ExpectedClosingAngle(token.unwrap())),
                }
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '/' => {
                is_self_closing = true;

                input.next();

                match input.next() {
                    Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => break,
                    token => return Err(Error::ExpectedClosingAngle(token.unwrap())),
                }
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                input.next();
                break;
            }
            _ => props.push(parse_prop(input)?),
        }
    }

    Ok(Token::TagOpen(TagOpen {
        is_builtin,
        path,
        props,
        is_default,
        is_self_closing,
    }))
}

pub fn parse(input: TokenStream) -> Result<Vec<Token>, Error> {
    let mut input = input.into_iter().peekable();
    let mut tokens = vec![];

    while let Some(_) = input.peek() {
        tokens.push(parse_token(&mut input)?);
    }

    Ok(tokens)
}

pub enum Token {
    TagOpen(TagOpen),
    TagClose(TagClose),
    Literal(Literal),
    Group(Group),
}
