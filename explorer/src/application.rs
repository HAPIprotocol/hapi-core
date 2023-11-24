use {
    anyhow::Result,
    std::net::{SocketAddr, TcpListener},
};

use crate::configuration::Configuration;

pub struct Application {
    pub socket: SocketAddr,
    pub enable_metrics: bool,
}

impl Application {
    pub fn from_configuration(configuration: Configuration) -> Result<Self> {
        let socket = TcpListener::bind(configuration.listener)?.local_addr()?;

        Ok(Self {
            socket,
            enable_metrics: configuration.enable_metrics,
        })
    }

    pub async fn run(self) -> Result<()> {
        self.spawn_server().await?.await?
    }

    pub fn port(&self) -> u16 {
        self.socket.port()
    }
}
