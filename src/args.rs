use std::env;

pub struct Args {
    inner: Vec<String>,
}

impl Args {
    pub fn new() -> Self {
        Self {
            inner: env::args().collect(),
        }
    }

    pub fn program(&self) -> &str {
        &self.inner[0]
    }

    pub fn has_long(&self, arg: &str) -> bool {
        self.inner.contains(&format!("--{}", arg))
    }

    pub fn has_short(&self, arg: char) -> bool {
        self.inner.contains(&format!("-{}", arg))
    }

    pub fn has(&self, short_arg: char, long_arg: &str) -> bool {
        self.has_short(short_arg) || self.has_long(long_arg)
    }

    pub fn args(&self) -> &[String] {
        &self.inner[1..]
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
