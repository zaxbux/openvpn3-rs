use zbus::{
    zvariant::{OwnedObjectPath, OwnedValue},
    CacheProperties, Connection,
};

use crate::{session::Session, ConfigurationProxy, Result, SessionsProxy};

#[derive(Clone, Debug)]
pub struct Configuration<'a> {
    pub(crate) conn: Connection,
    pub(crate) path: OwnedObjectPath,
    pub(crate) proxy: ConfigurationProxy<'a>,
    //pub(crate) sessions_proxy: &'a SessionsProxy<'a>,
    pub(crate) sessions_proxy: SessionsProxy<'a>,
    //pub(crate) _configuration_manager_proxy: &'a ConfigurationManagerProxy<'a>,
}

impl<'a> Configuration<'a> {
    const DBUS_INTERFACE: &'static str = "net.openvpn.v3.configuration";

    pub(crate) async fn new(
        conn: Connection,
        /*sessions_proxy: &'a SessionsProxy<'_>,*/
        /*configuration_manager_proxy: &'a ConfigurationManagerProxy<'_>,*/
        configuration_path: OwnedObjectPath,
    ) -> Result<Configuration<'a>> {
        let sessions_proxy = SessionsProxy::new(&conn).await?;
        let proxy = ConfigurationProxy::builder(&conn)
            .destination(Self::DBUS_INTERFACE)?
            .path(configuration_path.clone())?
            .cache_properties(CacheProperties::No)
            .build()
            .await?;

        Ok(Self {
            conn,
            path: configuration_path,
            proxy,
            sessions_proxy,
            //_configuration_manager_proxy: configuration_manager_proxy,
        })
    }

    pub async fn new_tunnel<'c>(&self) -> Result<Session<'c>> {
        let proxy = self.sessions_proxy.new_tunnel(&self.path).await?;
        Ok(Session::new(
            self.conn.clone(),
            OwnedObjectPath::from(proxy.path().clone()),
        )
        .await?)
        /* Ok(Session {
            //_conn: self.conn.clone(),
            //_path: OwnedObjectPath::from(proxy.path().clone()),
            proxy,
            //_sessions_proxy: &self.sessions_proxy,
        }) */
    }

    pub async fn fetch(&'a self) -> Result<String> {
        Ok(self.proxy.fetch().await?)
    }

    pub async fn json(&'a self) -> Result<serde_json::Value> {
        Ok(serde_json::from_str(
            self.proxy.fetch_json().await?.as_str(),
        )?)
    }

    pub async fn remove(&'a self) -> Result<()> {
        Ok(self.proxy.remove().await?)
    }

    pub async fn get_property<T>(&'a self, property_name: &str) -> Result<T>
    where
        T: TryFrom<OwnedValue>,
        T::Error: Into<zbus::Error>,
    {
        Ok(self.proxy.get_property(property_name).await?)
    }
}
