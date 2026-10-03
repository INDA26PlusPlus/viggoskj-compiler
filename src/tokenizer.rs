use std::char;

use crate::error::CompilerError;

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
        (
            LexerRule {
                continue_evaluator: number_literal_continue_decition,
                token_type_evaluator: number_literal_token_evaluator,
            },
            TokenState {
                decition: TokenizationDecition::Continue,
                span: (start, start),
            },
        ),
        (
            LexerRule {
                continue_evaluator: text_continue_decition,
                token_type_evaluator: string_token_evaluator,
            },
            TokenState {
                decition: TokenizationDecition::Continue,
                span: (start, start),
            },
        ),
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
                    break;
                }
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

fn text_continue_decition(ctx: &TokenizationContext) -> TokenizationDecition {
    if "abcdefghijklmnopqrstuvwxyz_".contains(ctx.current) {
        if "abcdefghijklmnopqrstuvwxyz_".contains(ctx.next) {
            return TokenizationDecition::Continue;
        } else {
            return TokenizationDecition::Complete;
        }
    }
    return TokenizationDecition::Stop;
}

fn string_token_evaluator(token_string: &str) -> Option<TokenType> {
    return Some(TokenType::Identifier);
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
