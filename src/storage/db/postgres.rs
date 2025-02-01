use axum::http::Uri;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::Storage;

type StoreResponse = Result<(), &'static str>;
type LoadResponse = Result<Uri, &'static str>;

enum Request {
    Store(Uri, String),
    Load(String),
}

pub struct Postgres {
    tx: Sender<Request>,
    rx_store: Arc<Mutex<Receiver<StoreResponse>>>,
    rx_load: Arc<Mutex<Receiver<LoadResponse>>>,
}

impl Postgres {
    pub fn connect(config: &str) -> Result<Self, &'static str> {
        // Create a simple streaming channel
        let (tx_store, rx_store) = channel();
        let (tx_load, rx_load) = channel();
        let (tx, rx) = channel();
        let config = config.parse().map_err(|_| "bad config")?;

        thread::spawn(move || {
            let manager = PostgresConnectionManager::new(config, NoTls);
            let mut pool = Pool::new(manager).expect("failed to create pool");

            pool.get()
                .expect("failed to get a worker")
                .batch_execute(include_str!("scripts/postgres/schema.sql"))
                .expect("valid schema");

            loop {
                let action: Request = rx.recv().expect("recv error");

                match action {
                    Request::Store(uri, code) => {
                        tx_store.send(pool.store(uri, &code)).expect("store error")
                    }
                    Request::Load(code) => tx_load.send(pool.load(code)).expect("load error"),
                };
            }
        });

        Ok(Self {
            tx,
            rx_store: Arc::new(Mutex::new(rx_store)),
            rx_load: Arc::new(Mutex::new(rx_load)),
        })
    }
}

impl Storage for Pool<PostgresConnectionManager<NoTls>> {
    fn store(&mut self, uri: axum::http::Uri, code: &str) -> std::result::Result<(), &'static str> {
        self.get()
            .map_err(|_| "failed to get a worker")?
            .execute(
                include_str!("scripts/postgres/insert.sql"),
                &[&code, &uri.to_string()],
            )
            .map_err(|_| "could not insert into sqlite")?;

        Ok(())
    }

    fn load(&self, code: String) -> std::result::Result<axum::http::Uri, &'static str> {
        let mut conn = self.get().map_err(|_| "failed to get a worker")?;

        conn.query(include_str!("scripts/postgres/select.sql"), &[&code])
            .map_err(|_| "failed to query postgres")?
            .iter()
            .filter_map(|row| row.get::<usize, String>(0).parse::<Uri>().ok())
            .next()
            .ok_or("could not find uri")
    }
}

impl Default for Postgres {
    fn default() -> Self {
        match Postgres::connect("host=localhost user=postgres password=secret") {
            Ok(client) => client,
            Err(e) => panic!("{}", e),
        }
    }
}

impl Storage for Postgres {
    fn store(&mut self, uri: Uri, code: &str) -> Result<(), &'static str> {
        self.tx
            .send(Request::Store(uri, code.to_owned()))
            .map_err(|_| "failed to send store request")?;

        self.rx_store
            .lock()
            .unwrap()
            .recv()
            .map_err(|_| "failed to receive store response")?
    }

    fn load(&self, code: String) -> Result<Uri, &'static str> {
        self.tx
            .send(Request::Load(code))
            .map_err(|_| "failed to send load request")?;

        self.rx_load
            .lock()
            .unwrap()
            .recv()
            .map_err(|_| "failed to receive load response")?
    }
}
