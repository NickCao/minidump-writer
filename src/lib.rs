use std::error;
use std::result;

type Error = Box<dyn error::Error + std::marker::Send + std::marker::Sync>;
pub type Result<T> = result::Result<T, Error>;

pub mod cpu_set;
pub mod linux_ptrace_dumper;