pub struct Command {
    pub last_result: Option<String>,
    last_call: std::time::Instant,
    command: &'static str,
    args: Vec<&'static str>,
    refresh_delay: std::time::Duration,
}

impl Command {
    pub async fn new(
        command: &'static str,
        args: Vec<&'static str>,
        call_interval: f32,
    ) -> Result<Self, tokio::io::Error> {
        let mut s = Self {
            command,
            args,
            refresh_delay: std::time::Duration::from_millis(
                (call_interval * 1000f32).round() as u64
            ),
            last_call: std::time::Instant::now(),
            last_result: None,
        };
        s.call().await?;
        Ok(s)
    }
    pub async fn update(&mut self) -> Result<(), tokio::io::Error> {
        if self.last_call.elapsed() > self.refresh_delay {
            self.call().await?;
        }
        Ok(())
    }
    async fn call(&mut self) -> Result<(), tokio::io::Error> {
        use tokio::process::Command;
        self.last_call = std::time::Instant::now();
        let out = Command::new(self.command).args(&self.args).output().await?;
        self.last_result = match String::from_utf8(out.stdout) {
            Ok(v) => Some(v.trim().to_string()),
            Err(_) => {
                let err_msg = format!("{} command returned invalid utf8", self.command);
                eprintln!(
                    "{}\n\tcommand:{}\n\targs:{:?}",
                    err_msg, self.command, self.args
                );
                Some(err_msg)
            }
        };
        Ok(())
    }
}
