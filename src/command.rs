use crate::context::Context;

pub struct Command<T> {
    pub ident: &'static str,
    pub alias: &'static str,
    pub directive: Box<dyn Fn(T, Context) + 'static>,
    pub flags: Vec<Flag>,
    pub meta: CommandMeta,
}

impl<U> Command<U> {
    pub fn new<T>(
        ident: &'static str,
        alias: &'static str,
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
            meta: CommandMeta::new(),
        }
    }

    pub fn description(mut self, desc: &'static str) -> Command<U>{
        self.meta.description = desc;
        return self;
    }

    pub fn short_description(mut self, desc: &'static str) -> Command<U> {
        self.meta.short_description = desc;
        return self;
    }

    pub fn usage(mut self, usage: &'static str) -> Command<U> {
        self.meta.usage = usage;
        return self;
    }

    pub fn flag(mut self, f: Flag) -> Command<U> {
        self.flags.push(f);
        return self;
    }

    pub fn run(&self, inner: U, ctx: Context) {
        if ctx.is_set("help") {
            let mut help_mesg: String = format!(
                "{}\n\n
                USAGE:\n
                {}\n",
                self.meta.short_description,
                self.meta.usage,
            );

            let mut options = String::new();
            for flag in self.flags.iter() {
                let flg = if flag.alias != "" {
                    format!("--{} ; -{}", flag.ident, flag.alias)
                } else {
                    flag.ident.to_string()
                };

                options.push_str(format!("{} : {}\n", flg, flag.description).as_str());
            }

            if options.len() != 0 {
                help_mesg.push_str(format!(
                    "OPTIONS:\n{}",
                    options,
                ).as_str());
            }

            println!("{}", help_mesg);
            
        } else {
            (self.directive)(inner, ctx);
        }
    }
}

pub struct CommandMeta {
    description: &'static str,
    short_description: &'static str,
    usage: &'static str,
}

impl CommandMeta {
    pub fn new() -> CommandMeta {
        CommandMeta {
            description: "",
            short_description: "",
            usage: ""
        }
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
    OptFlag,
}

impl Flag {
    pub fn new(
        ident: &'static str,
        alias: &'static str,
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
