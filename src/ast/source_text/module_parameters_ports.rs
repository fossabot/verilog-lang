//! A.1.3 Module parameters and ports
use crate::ast::*;

/// list_of_port_declarations ::= ( [ { attribute_instance } ansi_port_declaration { , { attribute_instance } ansi_port_declaration } ] )
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Ports {
    pub ports: Vec<(Attributes, Port)>,
}

impl Parse for Ports {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Ports::default();
        if parser.probe_err(&[Token::LParen]) {
            parser.advance();
            loop {
                let attrs = if parser.probe(&[Token::LParen]) {
                    Attributes::parse(parser).unwrap_or_default()
                } else {
                    Attributes::default()
                };
                if let Some(port) = Port::parse(parser) {
                    res.ports.push((attrs, port));
                }
                if parser.probe(&[Token::Comma]) {
                    parser.advance();
                    continue;
                } else if parser.probe_err(&[Token::Comma, Token::RParen]) {
                    parser.advance();
                    break;
                } else {
                    return None;
                }
            }
        }
        Some(res)
    }
}

/// ansi_port_declaration ::= [ net_port_header ] port_identifier
/// net_port_header ::= [ port_direction ] net_port_type
/// net_port_type ::= [ net_type ] data_type_or_implicit
/// data_type_or_implicit ::= data_type
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Port {
    pub direction: Option<PortDirection>,
    pub port_type: Option<NetType>,
    pub data_type: Option<DataType>,
    pub identifier: Identifier,
}

impl Parse for Port {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        let mut res = Port::default();
        if parser.probe(&[Token::Input, Token::Output, Token::InOut, Token::Ref]) {
            res.direction = PortDirection::parse(parser);
        }
        if parser.probe(&[
            Token::Supply0,
            Token::Supply1,
            Token::Tri,
            Token::TriAnd,
            Token::TriOr,
            Token::TriReg,
            Token::Tri0,
            Token::Tri1,
            Token::Uwire,
            Token::Wire,
            Token::Wand,
            Token::Wor,
        ]) {
            res.port_type = NetType::parse(parser);
        }
        if parser.probe(&[Token::Bit, Token::Logic, Token::Reg]) {
            res.data_type = DataType::parse(parser);
        }

        if parser.probe_err(&[Token::Identifier]) {
            if let Some(identifier) = Identifier::parse(parser) {
                res.identifier = identifier;
                return Some(res);
            }
        }

        None
    }
}

/// port_direction ::= input | output | inout | ref
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum PortDirection {
    Input,
    Output,
    InOut,
    Ref,
}

impl Parse for PortDirection {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if let Some(token) = parser.peek() {
            let res = match token.token {
                Token::Input => Some(PortDirection::Input),
                Token::Output => Some(PortDirection::Output),
                Token::InOut => Some(PortDirection::InOut),
                Token::Ref => Some(PortDirection::Ref),
                _ => None,
            };
            if res.is_some() {
                parser.advance();
            }
            res
        } else {
            None
        }
    }
}

/// net_type ::= supply0 | supply1 | tri | triand | trior | trireg | tri0 | tri1 | uwire| wire | wand | wor
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum NetType {
    Wire,
}

impl Parse for NetType {
    fn parse(parser: &mut Parser<'_>) -> Option<Self> {
        if let Some(token) = parser.peek() {
            let res = match token.token {
                Token::Wire => Some(NetType::Wire),
                _ => None,
            };
            if res.is_some() {
                parser.advance();
            }
            res
        } else {
            None
        }
    }
}