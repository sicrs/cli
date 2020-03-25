use crate::context::Context;

pub struct Command<T> {
    pub ident: &'static str,
    pub alias: Option<&'static str>,
    pub directive: Box<dyn Fn(T, Context) + 'static>,
    pub flags: Vec<Flag>,
    pub helptext: &'static str,
}

impl<U> Command<U> {
    pub fn new<T>(
        ident: &'static str,
        alias: Option<&'static str>,
        directive: T,
    ) -> Command<U>
    where
        T: Fn(U, Context) + 'static,
    {
        Command {
            ident,
            alias,
            directive: Box::new(directive),
            flags: Vec::new(),
            helptext: "s"
        }
    }

    pub fn flag(mut self, f: Flag) -> Command<U> {
        self.flags.push(f);
        return self;
    }

    pub fn set_help(mut self, ht: &'static str) -> Command<U> {
        self.helptext = ht;
        return self;
    }

    pub fn run(&self, inner: U, ctx: Context) {
        if ctx.is_set("help") {
            println!("{}", self.helptext)
        } else {
            (self.directive)(inner, ctx);
        }
    }
}

pub struct Flag {
    pub alias: Option<&'static str>,
    pub description: &'static str,
    pub ident: &'static str,
    pub kind: FlagKind,
}

#[derive(PartialEq)]
pub enum FlagKind {
    InputFlag,
    OptFlag,
}

impl Flag {
    pub fn new(
        ident: &'static str,
        alias: Option<&'static str>,
        kind: FlagKind,
        description: &'static str,
    ) -> Flag {
        Flag {
            alias,
            description,
            ident,
            kind,
        }
    }
}
