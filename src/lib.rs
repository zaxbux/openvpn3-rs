mod configuration;
mod error;
mod proxy;
mod session;

use futures_util::future;
use proxy::sessions::{LogStream, SessionManagerEventStream};
use zbus::{fdo::PeerProxy, zvariant::OwnedObjectPath, Connection};

pub use self::configuration::Configuration;
pub use self::error::*;
pub use self::proxy::*;
pub use self::session::Session;

pub type Result<T> = std::result::Result<T, Error>;

/// OpenVPN 3 Client
///
/// Provides convenience methods for creating a new OpenVPN tunnel.
#[derive(Clone, Debug)]
pub struct OpenVPN3<'a> {
    connection: Connection,
    peer_proxy: PeerProxy<'a>,
    sessions_proxy: SessionsProxy<'a>,
    configuration_manager_proxy: ConfigurationProxy<'a>,
}
impl<'a> OpenVPN3<'a> {
    /// Create a new `OpenVPN3` instance.
    pub async fn connect() -> Result<OpenVPN3<'a>> {
        let connection = Connection::system().await?;
        let sessions_proxy = SessionsProxy::new(&connection).await?;
        let peer_proxy = PeerProxy::builder(&connection)
            .destination("net.openvpn.v3.sessions")?
            .path(sessions_proxy.path().to_owned())?
            .build()
            .await?;
        let configuration_manager_proxy = ConfigurationProxy::new(&connection).await?;

        Ok(OpenVPN3 {
            connection,
            peer_proxy,
            sessions_proxy,
            configuration_manager_proxy,
        })
    }

    /// Fetch all available configuration profiles that are available to the user.
    pub async fn configurations(&'a self) -> Result<Vec<Configuration<'a>>> {
        let configs = self
            .configuration_manager_proxy
            .fetch_available_configs()
            .await?;

        futures_util::future::join_all(configs.into_iter().map(|object_path| {
            Configuration::new(
                self.connection.clone(),
                /*&self.sessions_proxy,*/
                /*&self.configuration_manager_proxy,*/
                object_path,
            )
        }))
        .await
        .into_iter()
        .collect::<Result<_>>()
    }

    /// Import a configuration profile.
    pub async fn import<'c>(
        &self,
        name: &str,
        config_str: &str,
        single_use: bool,
        persistent: bool,
    ) -> Result<Configuration<'c>> {
        self.ping().await?;

        let proxy = self
            .configuration_manager_proxy
            .import(name, config_str, single_use, persistent)
            .await?;

        Ok(Configuration::new(
            self.connection.clone(),
            OwnedObjectPath::from(proxy.path().clone()),
        )
        .await?)

        /* Ok(Configuration {
            conn: self.connection.clone(),
            path: OwnedObjectPath::from(proxy.path().clone()),
            proxy,
            sessions_proxy: self.sessions_proxy,
            //_configuration_manager_proxy: &self.configuration_manager_proxy,
        }) */
    }

    /// Fetch all sessions that are available to the user.
    pub async fn sessions(&'a self) -> Result<Vec<Session<'a>>> {
        self.ping().await?;

        let sessions = self.sessions_proxy.fetch_available_sessions().await?;

        futures_util::future::join_all(sessions.into_iter().map(|object_path| {
            Session::new(
                self.connection.clone(),
                /*&self.sessions_proxy,*/ object_path,
            )
        }))
        .await
        .into_iter()
        .collect::<Result<_>>()
    }

    pub async fn interfaces(&'a self) -> Result<Vec<String>> {
        self.ping().await?;

        let interfaces = self.sessions_proxy.fetch_managed_interfaces().await?;
        Ok(interfaces)
    }

    pub async fn net_cfg_manager(&'a self) -> Result<NetCfgProxy<'static>> {
        Ok(NetCfgProxy::new(&self.connection).await?)
    }

    pub async fn event_stream(&self) -> Result<SessionManagerEventStream<'a>> {
        Ok(self.sessions_proxy.receive_session_manager_event().await?)
    }

    pub async fn log_stream(&self) -> Result<LogStream<'a>> {
        Ok(self.sessions_proxy.receive_log().await?)
    }

    async fn ping(&'a self) -> Result<()> {
        let mut attempts = 10;

        while attempts > 0 {
            let results = future::join(self.peer_proxy.ping(), self.sessions_proxy.version()).await;

            if results.0.is_ok() && results.1.is_ok() {
                return Ok(());
            }

            attempts -= 1;
        }

        Ok(())
    }
}
