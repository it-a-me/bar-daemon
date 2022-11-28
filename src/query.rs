use async_trait::async_trait;
use tokio::io;


pub struct Command {
    command: &'static str,
    args: Vec<&'static str>,
}

impl Command {
    pub fn new(command: &'static str, args: Vec<&'static str>) -> Self {
        Self { command, args }
    }
    pub async fn call(&self) -> Result<String, io::Error> {
        Ok(String::from_utf8(
            tokio::process::Command::new(self.command)
                .args(&self.args)
                .output()
                .await?
                .stdout,
        )
        .unwrap_or(format!("{} returned invalid utf8", self.command))
        .trim()
        .to_string())
    }
}

#[async_trait]
impl super::BarQuery for Command {
    async fn result(&mut self) -> Result<String, io::Error> {
        self.call().await
    }
    async fn update(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
