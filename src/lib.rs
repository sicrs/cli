mod command;
mod context;

pub use command::{Command, Flag, FlagKind};
pub use context::{Context, FlagRes};
use std::process::exit;

pub struct App<T> {
    cmds: Vec<Command<T>>,
    pub inner: T,
}

impl<T> App<T> {
    pub fn new(inner: T) -> App<T> {
        App {
            cmds: Vec::new(),
            inner,
        }
    }

    pub fn register(mut self, cmd: Command<T>) -> App<T> {
        self.cmds.push(cmd);
        return self;
    }

    pub fn run(self, arg: Vec<String>) {
        if arg.len() == 0 {
            eprintln!("command not specified; try using --help");
            exit(1);
        }

        let cmd: Command<T>;

        {
            let mut cmnd: Option<Command<T>> = None;
            for command in self.cmds.into_iter() {
                if arg[0] == command.ident || arg[0] == command.alias {
                    cmnd = Some(command);
                    break;
                }
            }

            if let Some(command) = cmnd {
                cmd = command;
            } else {
                eprintln!("no command `{}` found", arg[0]);
                exit(1);
            }
        }

        let mut ctx = Context::new();

        // start from index 1
        let mut count = 1;
        loop {
            if count > arg.len() - 1 {
                break;
            }

            if arg[count].starts_with("-") {
                if arg[count].starts_with("--") {
                    for flag in &cmd.flags {
                        if arg[count].contains(flag.alias) {
                            //match
                            if flag.kind == FlagKind::InputFlag {
                                if count + 1 == arg.len() || arg[count + 1].starts_with("-") {
                                    eprintln!("error: no argument found for flag {}", arg[count]);
                                    exit(1);
                                }

                                if ctx.flagmap.contains_key(flag.ident) {
                                    eprintln!(
                                        "error: the flag --{} has already been set before",
                                        flag.alias,
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
                        if arg[count].contains(flag.ident) {
                            if flag.kind == FlagKind::InputFlag {
                                if arg[count] == format!("-{}", flag.ident) {
                                    if count + 1 == arg.len() || arg[count + 1].starts_with("-") {
                                        eprintln!("No argument for flag {} found", arg[count]);
                                        exit(1);
                                    }
                                    if ctx.flagmap.contains_key(flag.ident) {
                                        eprintln!(
                                            "error: the flag -{} has already been specified before",
                                            flag.ident
                                        );
                                        exit(1);
                                    }

                                    ctx.flagmap
                                        .insert(flag.ident, FlagRes::Input(arg[count + 1].clone()));
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

        (cmd.directive)(self.inner, ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::{App, Command, Context, FlagKind, Flag};
    #[test]
    fn app_test() {
        let app: App<()> = App::new(())
            .register(
                Command::new(
                    "test", "t", "test command", 
                |_inner: _, c: Context| {
                    assert!(c.is_set("o"));
                    assert_eq!(c.get("i").unwrap(), "some_input".to_string());
                    assert_eq!(c.arg[0], "another_input".to_string());
                })
                .flag(Flag::new("i", "input", FlagKind::InputFlag, "input"))
                .flag(Flag::new("o", "option", FlagKind::OptFlag, "option"))
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