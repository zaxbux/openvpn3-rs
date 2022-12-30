//! Provides an interface to communicate with the OpenVPN 3 configuration D-Bus API.

use super::Session;

use crate::{ConfigurationNodeProxy, Result, SessionsProxy};

use zbus::{
    zvariant::{OwnedObjectPath, OwnedValue},
    CacheProperties, Connection,
};

/// OpenVPN 3 Configuration
#[derive(Clone, Debug)]
pub struct Configuration<'a> {
    pub(crate) connection: Connection,
    pub(crate) path: OwnedObjectPath,
    pub(crate) proxy: ConfigurationNodeProxy<'a>,
    pub(crate) sessions_proxy: SessionsProxy<'a>,
}

impl<'a> Configuration<'a> {
    const DBUS_INTERFACE: &'static str = "net.openvpn.v3.configuration";

    /// Constructs a new [Configuration] that represents a single OpenVPN 3 VPN configuration profile through the D-Bus API.
    ///
    /// # Arguments
    ///
    /// * `connection` - D-Bus [Connection] instance.
    /// * `path` - D-Bus [OwnedObjectPath] to the configuration profile.
    pub(crate) async fn new(
        connection: Connection,
        path: OwnedObjectPath,
    ) -> Result<Configuration<'a>> {
        let sessions_proxy = SessionsProxy::new(&connection).await?;
        let proxy = ConfigurationNodeProxy::builder(&connection)
            .destination(Self::DBUS_INTERFACE)?
            .path(path.clone())?
            .cache_properties(CacheProperties::No)
            .build()
            .await?;

        Ok(Self {
            connection,
            path,
            proxy,
            sessions_proxy,
        })
    }

    /// Start a new VPN backend client process for this VPN configuration profile.
    pub async fn new_tunnel<'c>(&self) -> Result<Session<'c>> {
        let proxy = self.sessions_proxy.new_tunnel(&self.path).await?;
        Ok(Session::new(
            self.connection.clone(),
            OwnedObjectPath::from(proxy.path().clone()),
        )
        .await?)
    }

    /// Fetch the configuration as a string blob.
    pub async fn fetch(&'a self) -> Result<String> {
        Ok(self.proxy.fetch().await?)
    }

    /// Fetch the configuration as a JSON value.
    pub async fn json(&'a self) -> Result<serde_json::Value> {
        Ok(serde_json::from_str(
            self.proxy.fetch_json().await?.as_str(),
        )?)
    }

    /// Removes this VPN configuration profile.
    pub async fn remove(&'a self) -> Result<()> {
        Ok(self.proxy.remove().await?)
    }

    /// Get a property value from the underlying D-Bus proxy.
    pub async fn get_property<T>(&'a self, property_name: &str) -> Result<T>
    where
        T: TryFrom<OwnedValue>,
        T::Error: Into<zbus::Error>,
    {
        Ok(self.proxy.get_property(property_name).await?)
    }
}
