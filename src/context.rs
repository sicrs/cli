use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
    pub arg: Vec<String>,
    pub flagmap: HashMap<&'static str, FlagRes>,
    pub is_default: bool,
}

#[derive(Debug)]
pub enum FlagRes {
    Input(String),
    Opt,
}

impl Context {
    pub fn new() -> Context {
        Context {
            arg: Vec::new(),
            flagmap: HashMap::new(),
            is_default: false,
        }
    }

    pub fn is_set(&self, ident: &str) -> bool {
        return self.flagmap.contains_key(ident);
    }

    pub fn get(&self, ident: &str) -> Option<String> {
        if let Some(value) = self.flagmap.get(ident) {
            if let FlagRes::Input(content) = value {
                return Some(content.to_string());
            } else {
                return None;
            }
        }

        return None;
    }

    pub fn push(&mut self, k: &'static str,f: FlagRes) -> Option<FlagRes> {
        return self.flagmap.insert(k, f);
    }
}