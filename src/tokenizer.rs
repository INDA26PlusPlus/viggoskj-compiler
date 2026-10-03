use std::char;

use crate::error::CompilerError;

macro_rules! create_char_decition {
    ($name:ident, $char:literal) => {
        fn $name(ctx: &TokenizationContext) -> TokenizationDecition {
            if ctx.current == $char {
                TokenizationDecition::Complete
            } else {
                TokenizationDecition::Stop
            }
        }
    };
}

macro_rules! create_constant_choise {
    ($name:ident, $token_type:expr) => {
        fn $name(_: &str) -> Option<TokenType> {
            Some($token_type)
        }
    };
}

macro_rules! create_enclosing_decition {
    ($name:ident, $opened:literal, $closed:literal) => {
        fn $name(ctx: &TokenizationContext) -> TokenizationDecition {
            if ctx.current == $opened || ctx.current == $closed {
                TokenizationDecition::Complete
            } else {
                TokenizationDecition::Stop
            }
        }
    };
}

macro_rules! create_enclosing_choise {
    ($name:ident, $enclosing_type:expr, $open:literal, $closed:literal) => {
        fn $name(token: &str) -> Option<TokenType> {
            Some(TokenType::Enclosing {
                enclosing_type: $enclosing_type,
                open: match token {
                    $open => true,
                    $closed => false,
                    _ => return None,
                },
            })
        }
    };
}

macro_rules! create_word_decition {
    ($name:ident, $word:literal) => {
        fn $name(ctx: &TokenizationContext) -> TokenizationDecition {
            if $word.contains(ctx.current) {
                if $word.contains(ctx.next) {
                    TokenizationDecition::Continue
                } else {
                    TokenizationDecition::Complete
                }
            } else {
                TokenizationDecition::Stop
            }
        }
    };
}

macro_rules! create_word_choise {
    ($name:ident, $word:literal, $token_type:expr) => {
        fn $name(token: &str) -> Option<TokenType> {
            if token == $word {
                Some($token_type)
            } else {
                None
            }
        }
    };
}

// +
create_char_decition!(plus_decition_evaluator, '+');
create_constant_choise!(
    plus_token_type_evaluator,
    TokenType::Operator {
        operator: Operator::Plus
    }
);

// if
create_word_decition!(if_decition_evaluator, "if");
create_word_choise!(
    if_token_type_evaluator,
    "if",
    TokenType::FlowControll {
        flow_controll_type: FlowControllType::If
    }
);

// return
create_word_decition!(return_decition_evaluator, "return");
create_word_choise!(
    return_token_type_evaluator,
    "return",
    TokenType::FlowControll {
        flow_controll_type: FlowControllType::Return
    }
);

// ;
create_char_decition!(semicolon_decition_evaluator, ';');
create_constant_choise!(
    semicolon_token_type_evaluator,
    TokenType::FlowControll {
        flow_controll_type: FlowControllType::EndLine
    }
);

// {}
create_enclosing_decition!(curly_bracket_decition_evaluator, '{', '}');
create_enclosing_choise!(
    curly_bracket_token_type_evaluator,
    EnclosingType::CurlyBracket,
    "{",
    "}"
);

// ()
create_enclosing_decition!(bracket_decition_evaluator, '(', ')');
create_enclosing_choise!(
    bracket_token_type_evaluator,
    EnclosingType::Bracket,
    "(",
    ")"
);

// ==
create_word_decition!(equals_equals_decition_evaluator, "==");
create_word_choise!(
    equals_equals_token_type_evaluator,
    "==",
    TokenType::Operator {
        operator: Operator::EqualEqual
    }
);

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub span: (usize, usize),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Type,
    Enclosing {
        enclosing_type: EnclosingType,
        open: bool,
    },
    Identifier,
    Literal,
    Operator {
        operator: Operator,
    },
    FlowControll {
        flow_controll_type: FlowControllType,
    },
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Equal,
    EqualEqual,
    ForwardSlash,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnclosingType {
    Bracket,
    CurlyBracket,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FlowControllType {
    If,
    Return,
    EndLine,
}

type ContinueEvaluator = fn(&TokenizationContext) -> TokenizationDecition;
type TokenTypeEvaluator = fn(&str) -> Option<TokenType>;

struct LexerRule {
    continue_evaluator: ContinueEvaluator,
    token_type_evaluator: TokenTypeEvaluator,
}

fn reset_rules(start: usize) -> Vec<(LexerRule, TokenState)> {
    vec![
        generate_rule(
            return_decition_evaluator,
            return_token_type_evaluator,
            start,
        ),
        generate_rule(
            if_decition_evaluator,
            if_token_type_evaluator,
            start,
        ),
        generate_rule(
            semicolon_decition_evaluator,
            semicolon_token_type_evaluator,
            start,
        ),
        generate_rule(
            equals_equals_decition_evaluator,
            equals_equals_token_type_evaluator,
            start,
        ),
        generate_rule(
            bracket_decition_evaluator,
            bracket_token_type_evaluator,
            start,
        ),
        generate_rule(
            curly_bracket_decition_evaluator,
            curly_bracket_token_type_evaluator,
            start,
        ),
        generate_rule(plus_decition_evaluator, plus_token_type_evaluator, start),
        generate_rule(
            number_literal_continue_decition,
            number_literal_token_evaluator,
            start,
        ),
        generate_rule(identifier_continue_decition, literal_token_evaluator, start),
    ]
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, CompilerError> {
    let chars = Vec::from_iter(source.chars());

    let mut tokens = Vec::new();

    // instead do double buffers later
    let mut rules = reset_rules(0);

    for char_index in 0..(chars.len() - 1) {
        let context = TokenizationContext {
            current: chars[char_index],
            next: chars[char_index + 1],
        };

        for i in 0..rules.len() {
            let (rule, state) = &rules[i];

            if state.decition != TokenizationDecition::Continue {
                continue;
            };

            rules[i].1 = TokenState {
                decition: (rule.continue_evaluator)(&context),
                span: (state.span.0, char_index + 1),
            };
        }
        if rules
            .iter()
            .find(|(_, state)| state.decition == TokenizationDecition::Continue)
            .is_none()
        {
            // all completed, find highest priority parser

            let mut completed = false;

            for rule in rules
                .iter()
                .filter(|(_, state)| state.decition == TokenizationDecition::Complete)
            {
                if let Some(token_type) =
                    (rule.0.token_type_evaluator)(&source[rule.1.span.0..rule.1.span.1])
                {
                    tokens.push(Token {
                        token_type: token_type,
                        span: rule.1.span,
                    });
                    completed = true;
                    break;
                }
            }

            if !completed && source[rules[0].1.span.0..(char_index + 1)].trim().len() != 0 {
                tokens.push(Token {
                    span: (rules[0].1.span.0, char_index + 1),
                    token_type: TokenType::Unknown,
                });
            }

            rules = reset_rules(char_index + 1);
        }
    }

    return Ok(tokens);
}

pub fn token_string(token: &Token, source: &str) -> String {
    source[token.span.0..token.span.1].to_string()
}

struct TokenizationContext {
    current: char,
    next: char,
}

#[derive(Debug)]
struct TokenState {
    span: (usize, usize),
    decition: TokenizationDecition,
}

#[derive(PartialEq, Debug)]
enum TokenizationDecition {
    Continue,
    Stop,
    Complete,
}

fn identifier_continue_decition(ctx: &TokenizationContext) -> TokenizationDecition {
    if "abcdefghijklmnopqrstuvwxyz_".contains(ctx.current) {
        if "abcdefghijklmnopqrstuvwxyz_".contains(ctx.next) {
            return TokenizationDecition::Continue;
        } else {
            return TokenizationDecition::Complete;
        }
    }
    return TokenizationDecition::Stop;
}

fn literal_token_evaluator(token_string: &str) -> Option<TokenType> {
    return Some(TokenType::Identifier);
}

fn number_literal_continue_decition(ctx: &TokenizationContext) -> TokenizationDecition {
    if "1234567890xs".contains(ctx.current) {
        if "1234567890xs".contains(ctx.next) {
            return TokenizationDecition::Continue;
        } else {
            return TokenizationDecition::Complete;
        }
    }
    return TokenizationDecition::Stop;
}

fn number_literal_token_evaluator(token_string: &str) -> Option<TokenType> {
    let (first, second) = token_string.split_once("s")?;

    let mut first_combination = parse_number_literal_component(first)?;
    let mut second_combination = parse_number_literal_component(second)?;

    if first_combination.0 == 69 {
        (first_combination, second_combination) = (second_combination, first_combination);
    };

    if first_combination.0 != 67 || second_combination.0 != 69 {
        return None;
    } else {
        return Some(TokenType::Literal);
    }
}

fn parse_number_literal_component(component: &str) -> Option<(u32, u32)> {
    let (first, second) = match component.split_once("x") {
        Some(v) => v,
        None => return None,
    };

    let first_num = first.parse::<u32>().ok()?;

    let second_num = second.parse::<u32>().ok()?;

    Some((first_num, second_num))
}

fn generate_rule(
    continue_evaluator: ContinueEvaluator,
    token_type_evaluator: TokenTypeEvaluator,
    start: usize,
) -> (LexerRule, TokenState) {
    (
        LexerRule {
            continue_evaluator,
            token_type_evaluator,
        },
        TokenState {
            decition: TokenizationDecition::Continue,
            span: (start, start),
        },
    )
}
