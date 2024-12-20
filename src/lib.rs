use std::collections::{HashMap, HashSet};
use anyhow::Result;

pub struct CliTestSetup {
    cargo_flags: HashSet<String>,
    program: Vec<String>,
    env: HashMap<String, String>,
}

impl CliTestSetup {
    pub fn new() -> CliTestSetup {
        CliTestSetup {
            program: vec!["sh".to_owned(), "-c".to_owned()],
            env: HashMap::<_, _>::new(),
            cargo_flags: HashSet::<_>::new(),
        }
    }

    pub fn run(&self, command: &str) -> Result<CliTestOutput> {
        const CARGO_RUN_STR: &'static str = "cargo run";
        let mut cmd = &mut std::process::Command::new(&self.program[0]);
        for arg in self.program.iter().skip(1) {
            cmd = cmd.arg(arg)
        }
        for env in self.env.iter() {
            cmd = cmd.env(env.0, env.1);
        }

        let cargo_flags = self.cargo_flags.clone();
        let cargo_flags = cargo_flags.into_iter().collect::<Vec<_>>().join(" ");

        let output = cmd.arg(format!("{} {} -- {}", CARGO_RUN_STR, cargo_flags, command)).output()?;

        Ok(CliTestOutput {
            status: output.status,
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }

    pub fn set_program(&mut self, program: &str) -> Result<&mut Self> {
        let p = program.split(' ').collect::<Vec<_>>();
        if p.len() < 1 {
            return Err(anyhow::anyhow!("program can't be empty"));
        }
        self.program = p.iter().map(|s| s.to_string()).collect();
        Ok(self)
    }

    pub fn set_env(&mut self, env: HashMap<String, String>) -> &mut Self {
        self.env = env;
        self
    }

    pub fn with_env(&mut self, name: &str, value: &str) -> &mut Self {
        self.env.insert(name.to_owned(), value.to_owned());
        self
    }

    pub fn set_cargo_flags(&mut self, flag: HashSet<String>) -> &mut Self {
        self.cargo_flags = flag;
        self
    }

    pub fn with_cargo_flag(&mut self, flag: &str) -> &mut Self {
        self.cargo_flags.insert(flag.to_owned());
        self
    }
}

pub struct CliTestOutput {
    pub status: std::process::ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl CliTestOutput {
    pub fn success(&self) -> Result<&Self> {
        if self.status.success() {
            Ok(self)
        } else {
            Err(anyhow::anyhow!(
                "command failed with status: {}\nstderr: {}",
                self.status,
                self.stderr_str(),
            ))
        }
    }

    pub fn stdout_str(&self) -> String {
        String::from_utf8_lossy(&self.stdout).to_string()
    }

    pub fn stderr_str(&self) -> String {
        String::from_utf8_lossy(&self.stderr).to_string()
    }
}
