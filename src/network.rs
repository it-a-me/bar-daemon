use async_trait::async_trait;
use tokio::{
    fs, io,
    time::{Duration, Instant},
};
pub struct Network {
    byte_path: &'static str,
    bytes: u64,
    last_bytes: u64,
    last_time: Instant,
    interval: Duration,
}
impl Network {
    pub async fn new(byte_path: &'static str) -> Result<Self, io::Error> {
        let bytes = fs::read_to_string(byte_path)
            .await?
            .trim()
            .parse()
            .expect(&format!("{} is not a valid int", byte_path));
            let last_time= Instant::now();
        Ok(Self {
            byte_path,
            last_bytes: bytes,
            bytes,
            interval:last_time.elapsed(),
            last_time,
        })
    }
}

#[async_trait]
impl super::BarQuery for Network {
    async fn result(&mut self) -> Result<String, io::Error> {
        let bps = (self.bytes-self.last_bytes)as u128 *1000/self.interval.as_millis();
        Ok(match bps {
            b @ 0..=2000 => format!("{}B/s", b),
            k @ 2001..=2000000 => format!("{}KiB/s", k / 1000),
            m @ 2000001.. => format!("{}MiB/s", m / 1000000),
        })
    }
    async fn update(&mut self) -> Result<(), io::Error> {
        self.last_bytes = self.bytes;
        self.bytes = fs::read_to_string(self.byte_path)
            .await?
            .trim()
            .parse()
            .expect(&format!("{} is not a valid int", self.byte_path));
        self.interval = self.last_time.elapsed();
        self.last_time = Instant::now();
        Ok(())
    }
}
