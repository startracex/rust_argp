//! rust_argp
//!
//! parse arguments strings or similar strings

use std::fmt::{Display, Formatter, Result};

pub struct Argp {
    /// args will remove one or more matching items from it
    ///
    args: Vec<String>,
    /// Origin does not change
    ///
    origin: Vec<String>,
}

impl Display for Argp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f, "Argp {}\n  args:{:?},\norigin:{:?},\n{}", "{", self.origin, self.args, "}");
    }
}

/// new is alias of Argp::new
///
pub fn new() -> Argp {
    return Argp::new();
}

/// from is alias of Argp::from
///
pub fn from(origin: Vec<String>) -> Argp {
    return Argp::from(origin);
}

impl Argp {
    /// new get from std::env::args() and remove the first element
    ///
    pub fn new() -> Self {
        let mut origin = std::env::args().collect::<Vec<String>>();
        origin.remove(0);
        return Argp::from(origin);
    }

    /// from get from parameter
    ///
    pub fn from(origin: Vec<String>) -> Self {
        Argp {
            args: origin.clone(),
            origin,
        }
    }

    /// bool expects to have a boolean match value
    ///
    /// Remove it from args
    pub fn bool(&mut self, finds: &[&str]) -> bool {
        for (i, arg) in self.args.iter().enumerate() {
            for find in finds {
                if arg == find {
                    self.remove(i, 1);
                    return true;
                }
            }
        }
        return false;
    }

    /// bool_var call bool, if bool returns true, set v to true
    ///
    pub fn bool_var(&mut self, v: &mut bool, s: &[&str]) {
        *v = self.bool(s);
    }

    /// string expects to have a string match value
    ///
    /// The following 3 scenarios will match
    /// * a => 1 `["a", "=", "1"]`
    /// * b => 2 `["b=2"]`
    /// * c => 3 `["c", "3"]`
    ///
    /// Remove them all
    pub fn string(&mut self, finds: &[&str]) -> (String, bool) {
        for find in finds {
            for (i, arg) in self.args.iter().enumerate() {
                if arg == *find && i + 1 < self.args.len() {
                    let value = self.args[i + 1].clone();
                    if value == "=" && i + 2 < self.args.len() {
                        let value = self.args[i + 2].clone();
                        self.remove(i, 3);
                        return (value, true);
                    }
                    self.remove(i, 2);
                    return (value, true);
                }
                if self.args[i].starts_with(&format!("{}=", find)) && arg.len() > find.len() {
                    let value = arg[find.len() + 1..].to_string();
                    self.remove(i, 1);
                    return (value, true);
                }
            }
        }
        (String::new(), false)
    }

    /// string_var call string, if string.1 returns true, set v to string.0
    ///
    pub fn string_var(&mut self, v: &mut String, s: &[&str]) {
        let (value, exist) = self.string(s);
        if exist {
            *v = value;
        }
    }

    /// start expects to have a item that starts with find
    ///
    /// Remove it from args
    pub fn start(&mut self, find: &str) -> (String, bool) {
        for (i, arg) in self.args.to_owned().iter().enumerate() {
            let trim_len = find.len();
            if arg.len() > find.len() && arg[..trim_len] == *find {
                self.remove(i, 1);
                return (arg[trim_len..].to_string(), true);
            }
        }
        return (String::new(), false);
    }

    /// end expects to have a item that ends with find
    ///
    /// Remove it from args
    pub fn end(&mut self, find: &str) -> (String, bool) {
        for (i, arg) in self.args.to_owned().iter().enumerate() {
            let trim_len = arg.len() - find.len();
            if arg.len() > find.len() && arg[..trim_len] == *find {
                self.remove(i, 1);
                return (arg[..trim_len].to_string(), true);
            }
        }
        return (String::new(), false);
    }

    /// before returns content before a attr and its index
    ///
    pub fn before(&self, attr: &str) -> (Vec<String>, isize) {
        for (i, arg) in self.args.iter().enumerate() {
            if arg == attr && i > 0 {
                let value = self.args[..i].to_vec();
                return (value, i as isize);
            }
        }
        return (Vec::new(), -1);
    }

    /// after returns content after a attr and its index
    ///
    pub fn after(&self, attr: &str) -> (Vec<String>, isize) {
        for (i, arg) in self.args.iter().enumerate() {
            if arg == attr && i + 1 < self.args.len() {
                let value = self.args[i + 1..].to_vec();
                return (value, i as isize);
            }
        }
        return (Vec::new(), -1);
    }

    /// attach call after("--")
    ///
    /// Remove anything at and after after("--").1
    pub fn attach(&mut self) -> Vec<String> {
        let (after, at) = self.after("--");
        if at >= 0 {
            self.remove(0, at as usize);
        }
        return after;
    }

    /// remove args since index to index+length
    ///
    pub fn remove(&mut self, index: usize, length: usize) {
        self.args.drain(index..index + length);
    }

    /// short splits the content after the short and splices it with the short
    ///
    pub fn short(&mut self, short: &str) -> &mut Self {
        let short_len = short.len();
        for (i, arg) in self.args.to_owned().iter().enumerate() {
            if arg.len() > short_len && arg[0..short_len] == *short && !arg[short_len..].contains(short) {
                self.remove(i, 1);
                let letters = &arg[short_len..];
                for c in letters.chars() {
                    self.args.push(format!("{}{}", short, c));
                }
            }
        }
        return self;
    }


    /// index_of returns index of find
    ///
    pub fn index_of(&self, find: String) -> isize {
        return self.index_of_func(find, |arg: String, find: String| -> bool{
            return arg == find;
        });
    }

    /// index_of returns index of find if func returns true
    ///
    pub fn index_of_func(&self, find: String, func: fn(String, String) -> bool) -> isize {
        let mut index = -1;
        for arg in &self.args {
            index += 1;
            if func(arg.to_owned(), find.to_owned()) {
                return index;
            }
        }
        return -1;
    }
}

