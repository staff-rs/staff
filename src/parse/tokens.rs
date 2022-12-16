#[derive(Debug)]
pub enum Group {
    Chord,
    Block,
}

#[derive(Debug)]
pub enum Token<'a> {
    Start(Group),
    End(Group),
    Command(&'a str),
    Literal(&'a str),
    LineBreak,
}

pub struct Tokens<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokens<'a> {
    fn parse_literal(&mut self) -> Option<&'a str> {
        let start = self.pos;
        loop {
            match self.input.chars().nth(self.pos) {
                Some(' ') => {
                    self.pos += 1;
                    break if self.pos - 1 > start {
                        Some(&self.input[start..self.pos - 1])
                    } else {
                        None
                    };
                }
                Some('>') | Some('\n') => {
                    break if self.pos > start {
                        Some(&self.input[start..self.pos])
                    } else {
                        None
                    }
                }
                None => {
                    break if self.pos > start {
                        Some(&self.input[start..self.pos])
                    } else {
                        None
                    }
                }
                Some(_) => self.pos += 1,
            }
        }
    }
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.input.chars().nth(self.pos) {
                Some('\\') => {
                    self.pos += 1;
                    if let Some(literal) = self.parse_literal() {
                        break Some(Token::Command(literal));
                    } else {
                        todo!()
                    }
                }
                Some('{') => {
                    self.pos += 1;
                    break Some(Token::Start(Group::Block));
                }
                Some('}') => {
                    self.pos += 1;
                    break Some(Token::End(Group::Block));
                }
                Some('<') => {
                    self.pos += 1;
                    break Some(Token::Start(Group::Chord));
                }
                Some('>') => {
                    self.pos += 1;
                    break Some(Token::End(Group::Chord));
                }
                Some('\n') => {
                    self.pos += 1;
                    break Some(Token::LineBreak);
                }
                Some(_c) => {
                    if let Some(literal) = self.parse_literal() {
                        break Some(Token::Literal(literal));
                    }
                }
                None => break None,
            }
        }
    }
}
