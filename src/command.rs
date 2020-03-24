use crate::context::Context;

pub struct Command<T> {
    pub ident: &'static str,
    pub alias: &'static str,
    pub description: &'static str,
    pub directive: Box<dyn Fn(T, Context) + 'static>,
    pub flags: Vec<Flag>,
}

impl<U> Command<U> {
    pub fn new<T>(ident: &'static str, alias: &'static str, description: &'static str, directive: T) -> Command<U> 
    where
        T: Fn(U, Context) + 'static,
    {
        Command {
            ident,
            alias,
            directive: Box::new(directive),
            description,
            flags: Vec::new(),
        }
    }

    pub fn flag(mut self, f: Flag) -> Command<U> {
        self.flags.push(f);
        return self;
    }
}

pub struct Flag {
    pub alias: &'static str,
    pub description: &'static str,
    pub ident: &'static str,
    pub kind: FlagKind,
}

#[derive(PartialEq)]
pub enum FlagKind {
    InputFlag,
    OptFlag
}

impl Flag {
    pub fn new(ident: &'static str, alias: &'static str, kind: FlagKind, description: &'static str) -> Flag {
        Flag {
            alias,
            description,
            ident,
            kind,
        }
    }
}