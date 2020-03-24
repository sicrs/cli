mod command;
mod context;

pub use command::{Command, CommandMeta, Flag, FlagKind};
pub use context::{Context, FlagRes};
use std::process::exit;

impl<T> Command<T> {
    pub fn default() -> Command<T> {
        Command {
            ident: "",
            alias: "",
            directive: Box::new(|_i: T, _c: Context| {
                eprintln!("command not found; try using --help");
                exit(1);
            }),
            flags: Vec::new(),
            meta: CommandMeta::new(),
        }
    }
}

pub struct App<T> {
    cmds: Vec<Command<T>>,
    default: Command<T>,
    pub inner: T,
}

impl<T> App<T> {
    pub fn new(inner: T) -> App<T> {
        App {
            cmds: Vec::new(),
            default: Command::default(),
            inner,
        }
    }

    pub fn register_default(mut self, cmd: Command<T>) -> App<T> {
        self.default = cmd.flag(Flag::new("help", "h", FlagKind::OptFlag, "help"));
        return self;
    }

    pub fn register(mut self, cmd: Command<T>) -> App<T> {
        self.cmds.push(cmd.flag(Flag::new("help", "h", FlagKind::OptFlag, "help")));
        return self;
    }

    pub fn run(self, mut arg: Vec<String>) {
        if arg.len() == 0 {
            // try default
            let mut ctx = Context::new();
            ctx.is_default = true;
            self.default.run(self.inner, ctx);
        } else {
            let cmd: Command<T>;
            let mut ctx = Context::new();

            {
                let mut cmnd: Option<Command<T>> = None;

                for command in self.cmds.into_iter() {
                    if arg[0] == command.ident || arg[0] == command.alias {
                        cmnd = Some(command);
                        break;
                    }
                }

                if let Some(command) = cmnd {
                    // match
                    cmd = command;
                    arg.remove(0);
                } else {
                    // no match
                    cmd = self.default;
                    // default identifier is not matched
                    if arg[0] != cmd.ident && arg[0] != cmd.alias {
                        // run as default
                        ctx.is_default = true;
                    } else {
                        arg.remove(0);
                    }
                }
            }

            // start from index 1
            let mut count = 0;
            loop {
                if arg.len() == 0 {
                    break;
                } else {
                    if count >= arg.len() {
                        break;
                    }
                }

                if arg[count].starts_with("-") {
                    if arg[count].starts_with("--") {
                        for flag in &cmd.flags {
                            if arg[count].contains(flag.ident) {
                                //match
                                if flag.kind == FlagKind::InputFlag {
                                    if count + 1 == arg.len() || arg[count + 1].starts_with("-") {
                                        eprintln!(
                                            "error: no argument found for flag {}",
                                            arg[count]
                                        );
                                        exit(1);
                                    }

                                    if ctx.flagmap.contains_key(flag.ident) {
                                        eprintln!(
                                            "error: the flag --{} has already been set before",
                                            flag.ident,
                                        );
                                        exit(1);
                                    }

                                    ctx.flagmap
                                        .insert(flag.ident, FlagRes::Input(arg[count + 1].clone()));
                                    count += 1;
                                } else {
                                    ctx.flagmap.insert(flag.ident, FlagRes::Opt);
                                }
                                break;
                            }
                        }
                    } else {
                        for flag in &cmd.flags {
                            if arg[count].contains(flag.alias) {
                                if flag.kind == FlagKind::InputFlag {
                                    if arg[count] == format!("-{}", flag.alias) {
                                        if count + 1 == arg.len() || arg[count + 1].starts_with("-")
                                        {
                                            eprintln!("No argument for flag {} found", arg[count]);
                                            exit(1);
                                        }
                                        if ctx.flagmap.contains_key(flag.ident) {
                                            eprintln!(
                                            "error: the flag -{} has already been specified before",
                                            flag.alias
                                        );
                                            exit(1);
                                        }

                                        ctx.flagmap.insert(
                                            flag.ident,
                                            FlagRes::Input(arg[count + 1].clone()),
                                        );
                                        count += 1;
                                        break;
                                    } else {
                                        eprintln!(
                                            "You can't put an input flag alongside an option flag"
                                        );
                                        exit(1);
                                    }
                                } else {
                                    ctx.flagmap.insert(flag.ident, FlagRes::Opt);
                                }
                            }
                        }
                    }
                } else {
                    ctx.arg.push(arg[count].clone());
                }

                count += 1;
            }

            cmd.run(self.inner, ctx);
        }
    }
}

pub fn fallback() {
    eprintln!("try using --help");
    exit(1);
}

#[cfg(test)]
mod tests {
    use super::{App, Command, Context, Flag, FlagKind};
    #[test]
    fn app_test() {
        let app: App<()> = App::new(()).register(
            Command::new("test", "t", |_inner: _, c: Context| {
                assert!(c.is_set("output"));
                assert_eq!(c.get("input").unwrap(), "some_input".to_string());
                assert_eq!(c.arg[0], "another_input".to_string());
            })
            .flag(Flag::new("input", "i", FlagKind::InputFlag, "input"))
            .flag(Flag::new("output", "o", FlagKind::OptFlag, "option")),
        );
        
        let arg: Vec<String> = vec![
            "test".to_string(),
            "-i".to_string(),
            "some_input".to_string(),
            "-o".to_string(),
            "another_input".to_string(),
        ];

        app.run(arg);
    }
}
