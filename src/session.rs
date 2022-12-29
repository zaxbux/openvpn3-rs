use std::collections::HashMap;

use crate::{
    constants::{ClientAttentionGroup, ClientAttentionType},
    proxy::session::{AttentionRequiredStream, LogStream, Status, StatusChangeStream},
    Error, Result, SessionProxy,
};

use zbus::{
    zvariant::{ObjectPath, OwnedObjectPath, OwnedValue},
    CacheProperties, Connection,
};

pub type UserInputQueueTypeGroup = (ClientAttentionType, ClientAttentionGroup);
pub type Statistics = HashMap<String, i64>;

#[derive(Clone, Debug)]
pub struct Session<'a> {
    //pub(crate) _conn: Connection,
    //pub(crate) _path: OwnedObjectPath,
    pub(crate) proxy: SessionProxy<'a>,
    //pub(crate) _sessions_proxy: &'a SessionsProxy<'a>,
}

impl<'a> Session<'a> {
    const DBUS_INTERFACE: &'static str = "net.openvpn.v3.sessions";

    pub(crate) async fn new(
        conn: Connection,
        //sessions_proxy: &'a SessionsProxy<'_>,
        session_path: OwnedObjectPath,
    ) -> Result<Session<'a>> {
        let proxy = SessionProxy::builder(&conn)
            .destination(Self::DBUS_INTERFACE)?
            .path(session_path.clone())?
            .cache_properties(CacheProperties::No)
            .build()
            .await?;

        Ok(Self {
            //_conn: conn,
            //_path: session_path,
            proxy,
            //_sessions_proxy: sessions_proxy,
        })
    }

    pub fn path(&'a self) -> &ObjectPath {
        self.proxy.path()
    }

    pub async fn ready(&'a self) -> Result<()> {
        Ok(self.proxy.ready().await.map_err(|err| {
            let err_str = err.to_string();

            if err_str.contains("net.openvpn.v3.sessions.error: Backend VPN process is not ready") {
                Error::BackendNotReady
            } else if err_str.contains("net.openvpn.v3.error.ready: Missing user credentials") {
                Error::MissingUserCredentials
            } else {
                Error::Zbus(err)
            }
        })?)
    }

    pub async fn connect(&'a self) -> Result<()> {
        Ok(self.proxy.connect().await?)
    }

    pub async fn pause(&'a self, reason: &str) -> Result<()> {
        Ok(self.proxy.pause(reason).await?)
    }

    pub async fn resume(&'a self) -> Result<()> {
        Ok(self.proxy.resume().await?)
    }

    pub async fn restart(&'a self) -> Result<()> {
        Ok(self.proxy.restart().await?)
    }

    pub async fn disconnect(&'a self) -> Result<()> {
        Ok(self.proxy.disconnect().await?)
    }

    pub async fn status(&'a self) -> Result<Status> {
        Ok(self.proxy.status().await?)
    }

    pub async fn statistics(&'a self) -> Result<Statistics> {
        Ok(self.proxy.statistics().await?)
    }

    pub async fn get_property<T>(&'a self, property_name: &str) -> Result<T>
    where
        T: TryFrom<OwnedValue>,
        T::Error: Into<zbus::Error>,
    {
        Ok(self.proxy.get_property(property_name).await?)
    }

    pub async fn user_input_queue_get_type_group(&'a self) -> Result<Vec<UserInputQueueTypeGroup>> {
        Ok(self.proxy.user_input_queue_get_type_group().await?)
    }

    pub async fn user_input_queue_check(
        &'a self,
        qtype: ClientAttentionType,
        qgroup: ClientAttentionGroup,
    ) -> Result<Vec<u32>> {
        Ok(self.proxy.user_input_queue_check(qtype, qgroup).await?)
    }

    pub async fn user_input_queue_fetch(
        &'a self,
        qtype: ClientAttentionType,
        qgroup: ClientAttentionGroup,
        qid: u32,
    ) -> Result<UserInputSlot<'a>> {
        Ok(UserInputSlot::new(&self.proxy, qtype, qgroup, qid).await?)
    }

    pub async fn fetch_user_input_slots(&'a self) -> Result<Vec<UserInputSlot>> {
        let mut slots = Vec::new();

        for (qtype, qgroup) in self.user_input_queue_get_type_group().await? {
            for qid in self.user_input_queue_check(qtype, qgroup).await? {
                slots.push(UserInputSlot::new(&self.proxy, qtype, qgroup, qid).await?)
            }
        }

        Ok(slots)
    }

    pub async fn attention_required_stream(&self) -> Result<AttentionRequiredStream<'a>> {
        Ok(self.proxy.receive_attention_required().await?)
    }

    pub async fn status_change_stream(&self) -> Result<StatusChangeStream<'a>> {
        Ok(self.proxy.receive_status_change().await?)
    }

    /// Get the log stream
    ///
    /// This should be called after the backend process is ready
    pub async fn log_stream(&self) -> Result<LogStream<'a>> {
        self.proxy.log_forward(true).await?;
        Ok(self.proxy.receive_log().await?)
    }
}

pub struct UserInputSlot<'a> {
    proxy: &'a SessionProxy<'a>,
    qtype: ClientAttentionType,
    qgroup: ClientAttentionGroup,
    qid: u32,
    variable_name: String,
    label: String,
    mask: bool,
}

impl<'a> UserInputSlot<'a> {
    pub async fn new(
        proxy: &'a SessionProxy<'_>,
        qtype: ClientAttentionType,
        qgroup: ClientAttentionGroup,
        qid: u32,
    ) -> Result<UserInputSlot<'a>> {
        let qslot = proxy.user_input_queue_fetch(qtype, qgroup, qid).await?;

        // Sanity check
        if qtype != qslot.0 || qgroup != qslot.1 || qid != qslot.2 {
            return Err(Error::UserInputSlotMismatch);
        }

        Ok(Self {
            proxy,
            qtype,
            qgroup,
            qid,
            variable_name: qslot.3,
            label: qslot.4,
            mask: qslot.5,
        })
    }

    pub async fn provide_input(&'a self, value: &str) -> Result<()> {
        Ok(self
            .proxy
            .user_input_provide(self.qtype, self.qgroup, self.qid, value)
            .await?)
    }

    pub fn type_group(&'a self) -> UserInputQueueTypeGroup {
        (self.qtype, self.qgroup)
    }

    pub fn variable_name(&'a self) -> &str {
        &self.variable_name
    }

    pub fn label(&'a self) -> &str {
        &self.label
    }

    pub fn input_mask(&'a self) -> bool {
        self.mask
    }
}
