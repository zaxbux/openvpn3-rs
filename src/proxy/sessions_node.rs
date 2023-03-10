//! # DBus interface proxy for: `net.openvpn.v3.sessions`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `net.openvpn.v3.sessions.node.xml`.

use self::constants::*;
use crate::log::constants::{LogGroup, LogLevel};
use zbus::{dbus_proxy, fdo};

/// Session Object
///
/// [OpenVPN Documentation](https://github.com/OpenVPN/openvpn3-linux/blob/master/docs/dbus/dbus-service-net.openvpn.v3.sessions.md)
#[dbus_proxy(
    interface = "net.openvpn.v3.sessions",
    default_service = "net.openvpn.v3.sessions",
    default_path = "/net/openvpn/v3/sessions"
)]
pub trait SessionsNode {
    /// AccessGrant method
    ///
    /// By default, only the user ID (UID) who created the session have access to it. This method used to grant other users access to this session.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID to the user account which is granted access;
    fn access_grant(&self, uid: u32) -> zbus::Result<()>;

    /// AccessRevoke method
    ///
    /// This revokes access to a session object for a specific user. Please note that the owner (the user which initiated this session) cannot have its access revoked.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID to the user account which gets the access revoked.
    fn access_revoke(&self, uid: u32) -> zbus::Result<()>;

    /// Connect method
    ///
    /// This method starts the connection process. This requires that all credentials needed before contacting the server have been provided. It is always a good idea to first call the `net.openvpn.v3.sessions.Ready` method first to ensure the backend is ready to connect.
    fn connect(&self) -> zbus::Result<()>;

    /// Disconnect method
    ///
    /// Disconnects a VPN connection. This will shutdown and stop the VPN backend process and the session object will be removed.
    fn disconnect(&self) -> zbus::Result<()>;

    /// LogForward method
    ///
    /// This enables log forwarding from the session to the currently connected D-Bus client. The forwarding itself is sent by the `net.openvpn.v3.log` service.
    ///
    /// # Arguments
    ///
    /// * `enable` - Enables or disables the log forwarding.
    fn log_forward(&self, enable: bool) -> zbus::Result<()>;

    /// Pause method
    ///
    /// This method pauses an on-going VPN connection.
    ///
    /// # Arguments
    ///
    /// * `reason` - A string used for log purposes primarily, describing why the tunnel was paused.
    fn pause(&self, reason: &str) -> zbus::Result<()>;

    /// Ready method
    ///
    /// This method is to check if the backend VPN client is ready to connect. If it is ready, it will return immediately. If it is not, it will return with a D-Bus error exception providing the reason it is not ready. Most commonly it needs some input from the user, such as user credentials or some challenge token not already provided in the configuration.
    fn ready(&self) -> zbus::Result<()>;

    /// Restart method
    ///
    /// Completely disconnects and reconnects an active VPN connection.
    fn restart(&self) -> zbus::Result<()>;

    /// Resume method
    ///
    /// Resumes a paused VPN connection.
    fn resume(&self) -> zbus::Result<()>;

    /// UserInputProvide method
    ///
    /// This method is used to return information from the front-end application to the backend service.
    ///
    /// # Arguments
    ///
    /// * `type_` - [ClientAttentionType] reference to query for.
    /// * `group` - [ClientAttentionGroup] reference to query for.
    /// * `id` - Request ID to query for, provided by `UserInputQueueCheck()`.
    /// * `value` - The frontend's response to the backend.
    fn user_input_provide(
        &self,
        type_: ClientAttentionType,
        group: ClientAttentionGroup,
        id: u32,
        value: &str,
    ) -> zbus::Result<()>;

    /// UserInputQueueCheck method
    ///
    /// This is used to get the proper index values of information requests the backend has asked for and which is not yet satisfied. The index list is tied to a specific ([ClientAttentionType], [ClientAttentionGroup]) tuple.
    ///
    /// # Arguments
    ///
    /// * `type_` - [ClientAttentionType] reference to query for.
    /// * `group` - [ClientAttentionGroup] reference to query for.
    ///
    /// # Returns
    ///
    /// An array of indexes which needs to be provided.
    fn user_input_queue_check(
        &self,
        type_: ClientAttentionType,
        group: ClientAttentionGroup,
    ) -> zbus::Result<Vec<u32>>;

    /// UserInputQueueFetch method
    ///
    /// This method returns details about a specific information request from the backend process.
    ///
    /// # Arguments
    ///
    /// * `type_` - [ClientAttentionType] reference to query for.
    /// * `group` - [ClientAttentionGroup] reference to query for.
    /// * `id` - Request ID to query for, provided by `UserInputQueueCheck()`.
    ///
    /// # Returns
    ///
    /// A tuple consisting of:
    ///
    /// 0. `type` - [ClientAttentionType] reference.
    /// 1. `group` - [ClientAttentionGroup] reference.
    /// 2. `id` - Request ID.
    /// 3. `name` - An internal variable name.
    /// 4. `description` - A description to present to the front-end user.
    /// 5. `hidden_input` - If true, the user's input should be masked/hidden.
    fn user_input_queue_fetch(
        &self,
        type_: ClientAttentionType,
        group: ClientAttentionGroup,
        id: u32,
    ) -> zbus::Result<(
        ClientAttentionType,
        ClientAttentionGroup,
        u32,
        String,
        String,
        bool,
    )>;

    /// UserInputQueueGetTypeGroup method
    ///
    /// This will return information about various [ClientAttentionType] and [ClientAttentionGroup] tuples which contains requests for the front-end application. This information is used when checking the request queue via `UserInputQueueCheck()`.
    ///
    /// # Returns
    ///
    /// An array of tuples of ([ClientAttentionType], [ClientAttentionGroup]).
    fn user_input_queue_get_type_group(&self)
        -> zbus::Result<Vec<result::UserInputQueueTypeGroup>>;

    /// AttentionRequired signal
    ///
    /// This signal is issued whenever the backend needs information, most commonly from the front-end user interface. This can be used to get user credentials or do PKCS#11/SmartCard operations, etc.
    ///
    /// See [AttentionRequiredArgs].
    #[dbus_proxy(signal)]
    fn attention_required(
        &self,
        type_: ClientAttentionType,
        group: ClientAttentionGroup,
        message: &str,
    ) -> zbus::Result<()>;

    /// Log signal
    ///
    /// See [LogArgs].
    #[dbus_proxy(signal, name = "Log")]
    fn log(&self, group: LogGroup, level: LogLevel, message: &str) -> zbus::Result<()>;

    /// StatusChange signal
    ///
    /// This signal is issued each time specific events occurs. They can both be from the session object itself or [StatusChange] messages proxied from the VPN client backend process.
    ///
    /// See [StatusChangeArgs].
    #[dbus_proxy(signal)]
    fn status_change(
        &self,
        code_major: StatusMajor,
        code_minor: StatusMinor,
        message: &str,
    ) -> zbus::Result<()>;

    /// acl property
    ///
    /// An array of UID values granted access.
    #[dbus_proxy(property, name = "acl")]
    fn acl(&self) -> zbus::Result<Vec<u32>>;

    /// backend_pid property
    ///
    /// Process ID of the VPN backend client process.
    #[dbus_proxy(property, name = "backend_pid")]
    fn backend_pid(&self) -> zbus::Result<u32>;

    /// config_name property
    ///
    /// Name of the configuration profile when the session was started.
    #[dbus_proxy(property, name = "config_name")]
    fn config_name(&self) -> zbus::Result<String>;

    /// config_path property
    ///
    /// D-Bus object path to the configuration profile used.
    #[dbus_proxy(property, name = "config_path")]
    fn config_path(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// dco property
    ///
    /// Kernel based Data Channel Offload flag. Must be modified before calling `Connect()` to override the current setting.
    #[dbus_proxy(property, name = "dco")]
    fn dco(&self) -> zbus::Result<bool>;
    fn set_dco(&self, value: bool) -> fdo::Result<()>;

    /// device_name property
    ///
    /// Virtual network interface name used by this session.
    #[dbus_proxy(property, name = "device_name")]
    fn device_name(&self) -> zbus::Result<String>;

    /// device_path property
    ///
    /// D-Bus object path to the `net.openvpn.v3.netcfg` device object related to this session.
    #[dbus_proxy(property, name = "device_path")]
    fn device_path(&self) -> zbus::Result<String>;

    /// last_log property
    ///
    /// Contains the last Log signal proxied from the backend process.
    #[dbus_proxy(property, name = "last_log")]
    fn last_log(&self) -> zbus::Result<result::Log>;

    /// log_forwards property
    ///
    /// Log Proxy/forward object paths used by `net.openvpn.v3.log` to configure the forwarding.
    #[dbus_proxy(property, name = "log_forwards")]
    fn log_forwards(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// log_verbosity property
    ///
    /// Defines the minimum log level Log signals should have to be sent.
    #[dbus_proxy(property, name = "log_verbosity")]
    fn log_verbosity(&self) -> zbus::Result<LogLevel>;
    fn set_log_verbosity(&self, value: LogLevel) -> fdo::Result<()>;

    /// owner property
    ///
    /// The UID value of the user which did the import.
    #[dbus_proxy(property, name = "owner")]
    fn owner(&self) -> zbus::Result<u32>;

    /// public_access property
    ///
    /// If set to true, access control is disabled. Only owner may change this property, modify the ACL or delete the configuration.
    #[dbus_proxy(property, name = "public_access")]
    fn public_access(&self) -> zbus::Result<bool>;
    fn set_public_access(&self, value: bool) -> fdo::Result<()>;

    /// restrict_log_access property
    ///
    /// If set to true, only the session owner can modify `receive_log_events` and `log_verbosity`, otherwise all granted users can access the log settings.
    #[dbus_proxy(property, name = "restrict_log_access")]
    fn restrict_log_access(&self) -> zbus::Result<bool>;
    fn set_restrict_log_access(&self, value: bool) -> fdo::Result<()>;

    /// session_created property
    ///
    /// Unix Epoc timestamp of when the session was created.
    #[dbus_proxy(property, name = "session_created")]
    fn session_created(&self) -> zbus::Result<u64>;

    /// session_name property
    ///
    /// Name of the VPN session, named by the the OpenVPN 3 Core library on successful connect.
    #[dbus_proxy(property, name = "session_name")]
    fn session_name(&self) -> zbus::Result<String>;

    /// statistics property
    ///
    /// Contains tunnel statistics.
    #[dbus_proxy(property, name = "statistics")]
    fn statistics(&self) -> zbus::Result<result::Statistics>;

    /// status property
    ///
    /// Contains the last processed StatusChange signal.
    #[dbus_proxy(property, name = "status")]
    fn status(&self) -> zbus::Result<result::Status>;
}

pub mod result {
    use crate::log::constants::{LogCategory, LogGroup};

    use super::constants::*;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use zbus::zvariant::{OwnedValue, Type};

    pub type Statistics = std::collections::HashMap<String, i64>;
    pub type UserInputQueueTypeGroup = (ClientAttentionType, ClientAttentionGroup);

    /// Essentially a saved [super::Log] signal
    #[derive(Debug, Type, Serialize, Deserialize, PartialEq)]
    pub struct Log {
        pub group: LogGroup,
        pub category: LogCategory,
        pub message: String,
    }

    impl TryFrom<(u8, u8, String)> for Log {
        type Error = <u8 as TryFrom<OwnedValue>>::Error;

        fn try_from(v: (u8, u8, String)) -> std::result::Result<Self, Self::Error> {
            Ok(Self {
                group: unsafe { std::mem::transmute(<u8>::try_from(v.0)?) },
                category: unsafe { std::mem::transmute(<u8>::try_from(v.1)?) },
                message: v.2,
            })
        }
    }

    impl TryFrom<OwnedValue> for Log {
        type Error = zbus::Error;

        fn try_from(v: OwnedValue) -> std::result::Result<Self, Self::Error> {
            let v: (u8, u8, String) = v.try_into()?;
            Ok(Log::try_from(v)?)
        }
    }

    assert_impl_all!(Log: Send, Sync, Unpin);

    /// Essentially a saved [super::StatusChange] signal
    #[derive(Debug, Type, Serialize, Deserialize, PartialEq)]
    pub struct Status {
        /// Major status group classification.
        pub code_major: StatusMajor,
        /// Minor status category classification within the status group.
        pub code_minor: StatusMinor,
        /// An optional string containing a more descriptive message of the signal.
        pub status_message: String,
    }

    impl TryFrom<(u32, u32, String)> for Status {
        type Error = <u32 as TryFrom<OwnedValue>>::Error;

        fn try_from(v: (u32, u32, String)) -> std::result::Result<Self, Self::Error> {
            Ok(Self {
                code_major: unsafe { std::mem::transmute(<u32>::try_from(v.0)?) },
                code_minor: unsafe { std::mem::transmute(<u32>::try_from(v.1)?) },
                status_message: v.2,
            })
        }
    }

    impl TryFrom<OwnedValue> for Status {
        type Error = zbus::Error;

        fn try_from(v: OwnedValue) -> std::result::Result<Self, Self::Error> {
            let v: (u32, u32, String) = v.try_into()?;
            Ok(Status::try_from(v)?)
        }
    }

    assert_impl_all!(Status: Send, Sync, Unpin);
}

pub mod constants {
    use std::fmt;

    use serde_repr::{Deserialize_repr, Serialize_repr};
    use static_assertions::assert_impl_all;
    use zbus::zvariant::{OwnedValue, Type};

    /// Client Attention Type
    ///
    /// Source: openvpn3-linux/src/dbus/constants.hpp
    #[repr(u32)]
    #[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]

    pub enum ClientAttentionType {
        /// This is an invalid value, used for initialization only
        Unset,
        /// User credentials are requested
        Credentials,
        /// PKCS#11/Smart card operation
        PKCS11,
        /// Access permission
        AccessPerm,
    }

    assert_impl_all!(ClientAttentionType: Send, Sync, Unpin);

    impl fmt::Display for ClientAttentionType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Unset => write!(f, "(unset)",),
                Self::Credentials => write!(f, "User Credentials",),
                Self::PKCS11 => write!(f, "PKCS#11 operation",),
                Self::AccessPerm => write!(f, "Requesting access permission"),
            }
        }
    }

    /// Client Attention Group
    ///
    /// Source: openvpn3-linux/src/dbus/constants.hpp
    #[repr(u32)]
    #[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]
    pub enum ClientAttentionGroup {
        /// This is an invalid value, used for initialization only
        Unset,
        /// Classic username/password authentication
        UserPassword,
        /// Credentials for authenticating to the HTTP proxy
        HttpProxyCreds,
        /// Passphrase for the user's private key
        PkPassphrase,
        /// Static challenge/response authentication, typically acquired before a connection starts
        ChallengeStatic,
        /// Dynamic challenge/response authentication, requested by the VPN server
        ChallengeDynamic,
        /// PKCS#11 signature operation
        PKCS11Sign,
        /// PKCS#11 decrypt operation
        PKCS11Decrypt,
        /// URL for web authentication
        OpenUrl,
    }

    assert_impl_all!(ClientAttentionGroup: Send, Sync, Unpin);

    impl fmt::Display for ClientAttentionGroup {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Unset => write!(f, "(unset)"),
                Self::UserPassword => write!(f, "Username/password authentication"),
                Self::HttpProxyCreds => write!(f, "HTTP proxy credentials"),
                Self::PkPassphrase => write!(f, "Private key passphrase"),
                Self::ChallengeStatic => write!(f, "Static challenge"),
                Self::ChallengeDynamic => write!(f, "Dynamic challenge"),
                Self::PKCS11Sign => write!(f, "PKCS#11 sign operation"),
                Self::PKCS11Decrypt => write!(f, "PKCS#11 decrypt operation"),
                Self::OpenUrl => write!(f, "Web authentication"),
            }
        }
    }

    /// Status Major
    ///
    /// Source: openvpn3-linux/src/dbus/constants.hpp
    #[repr(u32)]
    #[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]
    pub enum StatusMajor {
        /// Invalid status major code, used for initialization
        UNSET = 0,
        /// Status is related to configuration
        CONFIG = 1,
        /// Status is related to an OpenVPN connection
        CONNECTION = 2,
        /// Status is related to an on-going session
        SESSION = 3,
        /// Status is related to Smart Card/PKCS#11 operations
        PKCS11 = 4,
        /// Status is related to process management
        PROCESS = 5,
    }

    assert_impl_all!(StatusMajor: Send, Sync, Unpin);

    impl TryFrom<OwnedValue> for StatusMajor {
        type Error = <u32 as TryFrom<OwnedValue>>::Error;

        fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
            // safe because StatusMajor has repr u32
            Ok(unsafe { std::mem::transmute(<u32>::try_from(v)?) })
        }
    }

    impl fmt::Display for StatusMajor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                StatusMajor::UNSET => write!(f, "(unset)"),
                StatusMajor::CONFIG => write!(f, "Configuration"),
                StatusMajor::CONNECTION => write!(f, "Connection"),
                StatusMajor::SESSION => write!(f, "Session"),
                StatusMajor::PKCS11 => write!(f, "PKCS#11"),
                StatusMajor::PROCESS => write!(f, "Process"),
            }
        }
    }

    /// Status Minor
    ///
    /// Source: openvpn3-linux/src/dbus/constants.hpp
    #[repr(u32)]
    #[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]
    pub enum StatusMinor {
        ///  An invalid result code, used for initialization
        Unset,

        ///  Failed parsing configuration
        CfgError,
        ///  Configuration file parsed successfully
        CfgOk,
        ///  Some embedded (inline) files are missing
        CfgInlineMissing,
        ///  Require input from user
        CfgRequireUser,

        ///  Client connection initialized, ready to connect
        ConnInit,
        ///  Client started connecting
        ConnConnecting,
        ///  Client have connected successfully
        ConnConnected,
        ///  Client started disconnect process
        ConnDisconnecting,
        ///  Client completed disconnecting
        ConnDisconnected,
        ///  Client connection failed, disconnected
        ConnFailed,
        ///  Client authentication failed, disconnected
        ConnAuthFailed,
        ///  Client needed to reconnect
        ConnReconnecting,
        ///  Client started to pause the connection
        ConnPausing,
        ///  Client connection is paused
        ConnPaused,
        ///  Client connection is resuming
        ConnResuming,
        ///  Client connection process have completed and exited successfully
        ConnDone,

        ///  New session object created
        SessNew,
        ///  Backend session object have completed its task
        SessBackendCompleted,
        ///  Session object removed
        SessRemoved,
        ///  User/password authentication needed
        SessAuthUserpass,
        ///  Challenge/response authentication needed
        SessAuthChallenge,
        ///  Authentication needed via external URL
        SessAuthUrl,

        ///  PKCS#11 sign operation required
        PKCS11Sign,
        ///  PKCS#11 encryption operation required
        PKCS11Encrypt,
        ///  PKCS#11 decryption operation required
        PKCS11Decrypt,
        ///  PKCS#11 verification operation required
        PKCS11Verify,

        ///  Successfully started a new process
        ProcStarted,
        ///  A process of ours stopped as expected
        ProcStopped,
        ///  A process of ours stopped unexpectedly
        ProcKilled,
    }

    assert_impl_all!(StatusMinor: Send, Sync, Unpin);

    impl TryFrom<OwnedValue> for StatusMinor {
        type Error = <u32 as TryFrom<OwnedValue>>::Error;

        fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
            // safe because StatusMinor has repr u32
            Ok(unsafe { std::mem::transmute(<u32>::try_from(v)?) })
        }
    }

    impl fmt::Display for StatusMinor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                StatusMinor::Unset => write!(f, "(unset)"),
                StatusMinor::CfgError => write!(f, "Configuration error"),
                StatusMinor::CfgOk => write!(f, "Configuration OK"),
                StatusMinor::CfgInlineMissing => write!(f, "Configuration missing inline data"),
                StatusMinor::CfgRequireUser => write!(f, "Configuration requires user input"),
                StatusMinor::ConnInit => write!(f, "Client initialized"),
                StatusMinor::ConnConnecting => write!(f, "Client connecting"),
                StatusMinor::ConnConnected => write!(f, "Client connected"),
                StatusMinor::ConnDisconnecting => write!(f, "Client disconnecting"),
                StatusMinor::ConnDisconnected => write!(f, "Client disconnected"),
                StatusMinor::ConnFailed => write!(f, "Client connection failed"),
                StatusMinor::ConnAuthFailed => write!(f, "Client authentication failed"),
                StatusMinor::ConnReconnecting => write!(f, "Client reconnect"),
                StatusMinor::ConnPausing => write!(f, "Client pausing connection"),
                StatusMinor::ConnPaused => write!(f, "Client connection paused"),
                StatusMinor::ConnResuming => write!(f, "Client connection resuming"),
                StatusMinor::ConnDone => write!(f, "Client process exited"),
                StatusMinor::SessNew => write!(f, "New session created"),
                StatusMinor::SessBackendCompleted => write!(f, "Backend Session Object completed"),
                StatusMinor::SessRemoved => write!(f, "Session deleted"),
                StatusMinor::SessAuthUserpass => write!(f, "User/password authentication"),
                StatusMinor::SessAuthChallenge => write!(f, "Challenge/response authentication"),
                StatusMinor::SessAuthUrl => write!(f, "URL authentication"),
                StatusMinor::PKCS11Sign => write!(f, "PKCS#11 Sign"),
                StatusMinor::PKCS11Encrypt => write!(f, "PKCS#11 Encrypt"),
                StatusMinor::PKCS11Decrypt => write!(f, "PKCS#11 Decrypt"),
                StatusMinor::PKCS11Verify => write!(f, "PKCS#11 Verify"),
                StatusMinor::ProcStarted => write!(f, "Process started"),
                StatusMinor::ProcStopped => write!(f, "Process stopped"),
                StatusMinor::ProcKilled => write!(f, "Process killed"),
            }
        }
    }
}
