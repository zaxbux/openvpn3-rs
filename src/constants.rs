use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use static_assertions::assert_impl_all;
use std::fmt;
use zbus::zvariant::{OwnedValue, Type};

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

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]
pub enum StatusMinor {
    ///  An invalid result code, used for initialization
    UNSET,

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
    Pkcs11Sign,
    ///  PKCS#11 encryption operation required
    Pkcs11Encrypt,
    ///  PKCS#11 decryption operation required
    Pkcs11Decrypt,
    ///  PKCS#11 verification operation required
    Pkcs11Verify,

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
            StatusMinor::UNSET => write!(f, "(unset)"),
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
            StatusMinor::Pkcs11Sign => write!(f, "PKCS#11 Sign"),
            StatusMinor::Pkcs11Encrypt => write!(f, "PKCS#11 Encrypt"),
            StatusMinor::Pkcs11Decrypt => write!(f, "PKCS#11 Decrypt"),
            StatusMinor::Pkcs11Verify => write!(f, "PKCS#11 Verify"),
            StatusMinor::ProcStarted => write!(f, "Process started"),
            StatusMinor::ProcStopped => write!(f, "Process stopped"),
            StatusMinor::ProcKilled => write!(f, "Process killed"),
        }
    }
}

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]

pub enum ClientAttentionType {
    /// This is an invalid value, used for initialization only
    Unset = 0,
    /// User credentials are requested
    Credentials = 1,
    /// PKCS#11/Smart card operation
    Pkcs11 = 2,
}

assert_impl_all!(ClientAttentionType: Send, Sync, Unpin);

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]
pub enum ClientAttentionGroup {
    /// This is an invalid value, used for initialization only
    Unset = 0,
    /// Classic username/password authentication
    UserPassword = 1,
    /// Credentials for authenticating to the HTTP proxy
    HttpProxyCreds = 2,
    /// Passphrase for the user's private key
    PkPassphrase = 3,
    /// Static challenge/response authentication, typically acquired before a connection starts
    ChallengeStatic = 4,
    /// Dynamic challenge/response authentication, requested by the VPN server
    ChallengeDynamic = 5,
    /// PKCS#11 signature operation
    Pkcs11Sign = 6,
    /// PKCS#11 decrypt operation
    Pkcs11Decrypt = 7,
    /// URL for web authentication
    OpenUrl = 8,
}

assert_impl_all!(ClientAttentionGroup: Send, Sync, Unpin);

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]
pub enum LogGroup {
    /// Default - should not be used in code, but is here to detect errors
    UNDEFINED,
    /// Master process (main openvpn-manager)
    MASTERPROC,
    /// Configuration Manager process (child of openvpn-manager
    CONFIGMGR,
    /// Session manager process (child of openvpn-manager)
    SESSIONMGR,
    /// Backend starter process (openvpn3-service-backendstart)
    BACKENDSTART,
    /// Logger process (child of openvpn-manager)
    LOGGER,
    /// Session process (openvpn-service-client)
    BACKENDPROC,
    /// OpenVPN 3 Core tunnel object in the session process
    CLIENT,
    /// Network Configuration service (openvpn3-service-netcfg
    NETCFG,
    /// External services integrating with openvpn3-service-logger
    EXTSERVICE,
}

impl fmt::Display for LogGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogGroup::UNDEFINED => write!(f, "[[UNDEFINED]]"),
            LogGroup::MASTERPROC => write!(f, "Master Process"),
            LogGroup::CONFIGMGR => write!(f, "Config Manager"),
            LogGroup::SESSIONMGR => write!(f, "Session Manager"),
            LogGroup::BACKENDSTART => write!(f, "Backend Starter"),
            LogGroup::LOGGER => write!(f, "Logger"),
            LogGroup::BACKENDPROC => write!(f, "Backend Session Process"),
            LogGroup::CLIENT => write!(f, "Client"),
            LogGroup::NETCFG => write!(f, "Network Configuration"),
            LogGroup::EXTSERVICE => write!(f, "External Service"),
        }
    }
}

assert_impl_all!(LogGroup: Send, Sync, Unpin);

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]
pub enum LogCategory {
    /// Undefined/not set
    UNDEFINED,
    /// Debug messages
    DEBUG,
    /// Even more details
    VERB2,
    /// More details
    VERB1,
    /// Informational messages
    INFO,
    /// Warnings - important issues which might need attentio
    WARN,
    /// Errors - These must be fixed for successful operation
    ERROR,
    /// Critical - These requires users attention
    CRIT,
    /// Fatal errors - The current operation is going to stop
    FATAL,
}

assert_impl_all!(LogCategory: Send, Sync, Unpin);

impl fmt::Display for LogCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogCategory::UNDEFINED => write!(f, "[[UNDEFINED]]"),
            LogCategory::DEBUG => write!(f, "DEBUG"),
            LogCategory::VERB2 => write!(f, "VERB2"),
            LogCategory::VERB1 => write!(f, "VERB1"),
            LogCategory::INFO => write!(f, "INFO"),
            LogCategory::WARN => write!(f, "WARNING"),
            LogCategory::ERROR => write!(f, "-- ERROR --"),
            LogCategory::CRIT => write!(f, "!! CRITICAL !!"),
            LogCategory::FATAL => write!(f, "**!! FATAL !!**"),
        }
    }
}

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]
pub enum LogLevel {
    /// Log level 0 - Only FATAL and Critical messages are logged
    FATAL,
    /// Log level 1 - includes log level 0 + Error messages
    ERROR,
    /// Log level 2 - includes log level 1 + Warning messages
    WARNING,
    /// Log level 3 - includes log level 2 + informational messages
    INFO,
    /// Log level 4 - includes log level 3 + Verb 1 messages
    VERB1,
    /// Log level 5 - includes log level 4 + Verb 2 messages
    VERB2,
    /// Log level 6 - includes log level 5 + Debug messages (everything)
    DEBUG,
}

assert_impl_all!(LogLevel: Send, Sync, Unpin);

impl TryFrom<OwnedValue> for LogLevel {
    type Error = <u32 as TryFrom<OwnedValue>>::Error;

    fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
        Ok(unsafe { std::mem::transmute(<u32>::try_from(v)?) })
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::FATAL => write!(f, "FATAL"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::WARNING => write!(f, "WARNING"),
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::VERB1 => write!(f, "VERB1"),
            LogLevel::VERB2 => write!(f, "VERB2"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
        }
    }
}

#[repr(u16)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Type, Debug, PartialEq, Eq)]
pub enum EventType {
    /// Should not be used, identifies an uninitialized object or an error.
    Unset = 0,
    /// A new VPN session was created. It might not yet be started.
    Created = 1,
    /// An existing session object was destroyed, the session was disconnected.
    Destroyed = 2,
}

assert_impl_all!(EventType: Send, Sync, Unpin);

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unset => write!(f, "Unset!"),
            Self::Created => write!(f, "Created"),
            Self::Destroyed => write!(f, "Destroyed"),
        }
    }
}

/// Flags used in the CheckAuthorization() method.
#[bitflags]
#[repr(u32)]
#[derive(Type, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum NetworkChangeEventFilterFlags {
    ///	A new virtual interface has been added on the system
    DeviceAdded = 0x001,
    ///	A virtual interface has been removed from the system
    DeviceRemoved = 0x002,
    ///	An IP address has been added to a virtual interface
    IpaddrAdded = 0x004,
    ///	An IP address has been removed from the virtual interface
    IpaddrRemoved = 0x008,
    ///	A route has been added to the routing table, related to this interface
    RouteAdded = 0x010,
    ///	A route has been remove from the routing table, related to this interface
    RouteRemoved = 0x020,
    ///	A route has been excluded from the routing table, related to this interface
    RouteExcluded = 0x040,
    ///	A DNS server has been added to the DNS configuration
    DnsServerAdded = 0x080,
    ///	A DNS server has been removed from the DNS configuration
    DnsServerRemoved = 0x100,
    ///	A DNS search domain has been added to the DNS configuration
    DnsSearchAdded = 0x200,
    ///	A DNS search domain has been removed from the DNS configuration
    DnsSearchRemoved = 0x400,
}

assert_impl_all!(NetworkChangeEventFilterFlags: Send, Sync, Unpin);
