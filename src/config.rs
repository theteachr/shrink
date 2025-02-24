use url::Url;

pub struct Config {
    pub port: u16,
    pub server_url: Url,
}

impl Default for Config {
    fn default() -> Self {
        let port = 3000;
        let mut base: Url = "http://localhost".parse().unwrap();
        let _ = base.set_port(Some(port));

        Self {
            port,
            server_url: base,
        }
    }
}

fn read<R>(name: &'static str, f: impl Fn(String) -> Option<R>) -> Option<R> {
    std::env::var(name).ok().and_then(f)
}

impl Config {
    pub fn from_env() -> Option<Self> {
        let config = Config {
            port: read("PORT", |v| v.parse().ok())?,
            server_url: read("SERVER_URL", |url| url.parse().ok())?,
        };

        Some(config)
    }
}
