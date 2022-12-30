//! Provides an interface to communicate with the OpenVPN 3 sessions D-Bus API.

use crate::{
    proxy::sessions_node::{AttentionRequiredStream, LogStream, StatusChangeStream},
    sessions_node::{
        constants::{ClientAttentionGroup, ClientAttentionType},
        result::{Statistics, Status, UserInputQueueTypeGroup},
    },
    Error, Result, SessionsNodeProxy,
};

use zbus::{
    zvariant::{ObjectPath, OwnedObjectPath, OwnedValue},
    CacheProperties, Connection,
};

/// OpenVPN 3 Session
#[derive(Clone, Debug)]
pub struct Session<'a> {
    pub(crate) proxy: SessionsNodeProxy<'a>,
}

impl<'a> Session<'a> {
    const DBUS_INTERFACE: &'static str = "net.openvpn.v3.sessions";

    /// Constructs a new [Session] that represents a single OpenVPN 3 VPN session through the D-Bus API.
    pub(crate) async fn new(
        conn: Connection,
        session_path: OwnedObjectPath,
    ) -> Result<Session<'a>> {
        let proxy = SessionsNodeProxy::builder(&conn)
            .destination(Self::DBUS_INTERFACE)?
            .path(session_path.clone())?
            .cache_properties(CacheProperties::No)
            .build()
            .await?;

        Ok(Self { proxy })
    }

    /// Get a reference to the underlying proxy's object path.
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

    /// Start the connection process.
    pub async fn connect(&'a self) -> Result<()> {
        Ok(self.proxy.connect().await?)
    }

    /// Pause an active connection.
    pub async fn pause(&'a self, reason: &str) -> Result<()> {
        Ok(self.proxy.pause(reason).await?)
    }

    /// Resume a paused connection.
    pub async fn resume(&'a self) -> Result<()> {
        Ok(self.proxy.resume().await?)
    }

    /// Disconnect and reconnect.
    pub async fn restart(&'a self) -> Result<()> {
        Ok(self.proxy.restart().await?)
    }

    /// Disconnect and remove the VPN session.
    pub async fn disconnect(&'a self) -> Result<()> {
        Ok(self.proxy.disconnect().await?)
    }

    /// Get the last processed [StatusChange] signal.
    pub async fn status(&'a self) -> Result<Status> {
        Ok(self.proxy.status().await?)
    }

    /// Get tunnel statistics.
    pub async fn statistics(&'a self) -> Result<Statistics> {
        Ok(self.proxy.statistics().await?)
    }

    /// Get a property value from the underlying D-Bus proxy.
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

    /// Fetch a [UserInputSlot] which represents a request for user input and which can be used to provide input to the backend.
    ///
    /// # Arguments
    ///
    /// * `qtype` - Queue type of the user input slot.
    /// * `qgroup` - Queue group of the user input slot.
    /// * `qid` - Queue ID of the user input slot to retrieve.
    ///
    /// # Returns
    ///
    /// A [UserInputSlot] which provides information on what input to query for.
    pub async fn user_input_queue_fetch(
        &'a self,
        qtype: ClientAttentionType,
        qgroup: ClientAttentionGroup,
        qid: u32,
    ) -> Result<UserInputSlot<'a>> {
        Ok(UserInputSlot::new(&self.proxy, qtype, qgroup, qid).await?)
    }

    /// Fetch all required user inputs.
    ///
    /// # Returns
    ///
    /// An array of [UserInputSlot] instances which represent single requests for input and can be used to provide input to the backend.
    pub async fn fetch_user_input_slots(&'a self) -> Result<Vec<UserInputSlot>> {
        let mut slots = Vec::new();

        for (qtype, qgroup) in self.user_input_queue_get_type_group().await? {
            for qid in self.user_input_queue_check(qtype, qgroup).await? {
                slots.push(UserInputSlot::new(&self.proxy, qtype, qgroup, qid).await?)
            }
        }

        Ok(slots)
    }

    /// Get a [AttentionRequiredStream] for this VPN session.
    pub async fn attention_required_stream(&self) -> Result<AttentionRequiredStream<'a>> {
        Ok(self.proxy.receive_attention_required().await?)
    }

    /// Get a [StatusChangeStream] for this VPN session.
    pub async fn status_change_stream(&self) -> Result<StatusChangeStream<'a>> {
        Ok(self.proxy.receive_status_change().await?)
    }

    /// Get a [LogStream] for this VPN session.
    ///
    /// This should be called after the backend process is ready
    pub async fn log_stream(&self) -> Result<LogStream<'a>> {
        self.proxy.log_forward(true).await?;
        Ok(self.proxy.receive_log().await?)
    }
}

/// User Input Slot
///
/// Represents a single request for user input by the backend VPN process.
pub struct UserInputSlot<'a> {
    proxy: &'a SessionsNodeProxy<'a>,
    qtype: ClientAttentionType,
    qgroup: ClientAttentionGroup,
    qid: u32,
    variable_name: String,
    label: String,
    mask: bool,
}

impl<'a> UserInputSlot<'a> {
    /// Construct a [UserInputSlot]
    ///
    /// # Arguments
    ///
    /// * `proxy` - [SessionsNodeProxy] object for the related VPN session.
    /// * `qtype` - [ClientAttentionType] of the request.
    /// * `qgroup` - [ClientAttentionGroup] of the request.
    /// * `qid` - Unique ID for this request.
    pub async fn new(
        proxy: &'a SessionsNodeProxy<'_>,
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

    /// Provide input to the backend for this request.
    ///
    /// # Arguments
    ///
    /// * `value` - Input value.
    pub async fn provide_input(&'a self, value: &str) -> Result<()> {
        Ok(self
            .proxy
            .user_input_provide(self.qtype, self.qgroup, self.qid, value)
            .await?)
    }

    /// A tuple consisting of this request's [ClientAttentionType] and [ClientAttentionGroup].
    pub fn type_group(&'a self) -> UserInputQueueTypeGroup {
        (self.qtype, self.qgroup)
    }

    /// Internal variable name.
    pub fn variable_name(&'a self) -> &str {
        &self.variable_name
    }

    /// A description to present to the user.
    pub fn label(&'a self) -> &str {
        &self.label
    }

    /// Should the user's input be masked/hidden?
    pub fn input_mask(&'a self) -> bool {
        self.mask
    }
}
