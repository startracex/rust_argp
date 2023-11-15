use std::fmt::{Display, Formatter, Result};

pub struct Argp {
    args: Vec<String>,
    origin: Vec<String>,
}

impl Display for Argp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Argp{}\n  args:{:?}\norigin:{:?}\n{}", "{", self.origin, self.args, "}")
    }
}

pub fn new() -> Argp {
    return Argp::new();
}

pub fn from(origin: Vec<String>) -> Argp {
    return Argp::from(origin);
}

impl Argp {
    pub fn new() -> Self {
        let mut origin = std::env::args().collect::<Vec<String>>();
        origin.remove(0);
        return Argp::from(origin);
    }

    pub fn from(origin: Vec<String>) -> Self {
        Argp {
            args: origin.clone(),
            origin,
        }
    }

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

    pub fn bool_var(&mut self, v: &mut bool, s: &[&str]) {
        *v = self.bool(s);
    }

    pub fn string(&mut self, finds: &[&str]) -> (String, bool) {
        for find in finds {
            for (i, arg) in self.args.iter().enumerate() {
                if arg == *find && i + 1 < self.args.len() {
                    let value = self.args[i + 1].clone();
                    if value == "=" && i + 2 < self.args.len() {
                        let value = self.args[i + 2].clone();
                        self.remove(i, 3);
                        return (value, true);
                    } else {
                        self.remove(i, 2);
                        return (value, true);
                    }
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

    pub fn string_var(&mut self, v: &mut String, s: &[&str]) {
        let (value, exist) = self.string(s);
        if exist {
            *v = value;
        }
    }

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

    pub fn before(&self, attr: &str) -> (Vec<String>, isize) {
        for (i, arg) in self.args.iter().enumerate() {
            if arg == attr && i > 0 {
                let value = self.args[..i].to_vec();
                return (value, i as isize);
            }
        }
        return (Vec::new(), -1);
    }

    pub fn after(&self, attr: &str) -> (Vec<String>, isize) {
        for (i, arg) in self.args.iter().enumerate() {
            if arg == attr && i + 1 < self.args.len() {
                let value = self.args[i + 1..].to_vec();
                return (value, i as isize);
            }
        }
        return (Vec::new(), -1);
    }

    pub fn attach(&mut self) -> Vec<String> {
        let (after, at) = self.after("--");
        if at >= 0 {
            self.remove(0, at as usize);
        }
        return after;
    }

    pub fn remove(&mut self, index: usize, length: usize) {
        self.args.drain(index..index + length);
    }

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


    pub fn index_of(&self, find: String) -> isize {
        return self.index_of_func(find, |arg: String, find: String| -> bool{
            return arg == find;
        });
    }

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

