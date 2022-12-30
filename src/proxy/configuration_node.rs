//! # DBus interface proxy for: `net.openvpn.v3.configuration`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `net.openvpn.v3.configuration.node.xml`.

use zbus::{dbus_proxy, fdo};

/// Configuration Object
#[dbus_proxy(
    interface = "net.openvpn.v3.configuration",
    default_service = "net.openvpn.v3.configuration",
    default_path = "/net/openvpn/v3/configuration"
)]
trait ConfigurationNode {
    /// AccessGrant method
    ///
    /// By default, only the user ID (UID) who imported the configuration have access to it. This method used to grant other users access to the configuration.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID to the user account which is granted access.
    fn access_grant(&self, uid: u32) -> fdo::Result<()>;

    /// AccessRevoke method
    ///
    /// This revokes access to a configuration object for a specific user. Please note that the owner (the user which imported the configuration) cannot have its access revoked.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID to the user account which gets the access revoked.
    fn access_revoke(&self, uid: u32) -> fdo::Result<()>;

    /// Fetch method
    ///
    /// This method will return a string of the stored configuration profile as it is stored. This should be contain the same information which was imported. It will not necessarily be an identical copy of what was imported, as it has been processed during the import.
    ///
    /// # Returns
    /// The configuration file as a plain string blob.
    fn fetch(&self) -> zbus::Result<String>;

    /// FetchJSON method
    ///
    /// This is a variant of Fetch, which returns the configuration profile formatted as a JSON string blob. The intention of this is for user front-ends to have a simple API to retrieve the complete configuration profile in a format which can easily be parsed and presented in a user interface.
    ///
    /// # Returns
    ///
    /// The configuration file as a JSON formatted string blob.
    #[dbus_proxy(name = "FetchJSON")]
    fn fetch_json(&self) -> zbus::Result<String>;

    /// Remove method
    ///
    /// Removes a VPN profile from the configuration manager. If the configuration is persistent, it will be removed from the disk as well. This method takes no arguments and does not return anything on success. If an error occurs, a D-Bus error is returned.
    fn remove(&self) -> fdo::Result<()>;

    /// Seal method
    ///
    /// This method makes the configuration read-only. That means it can no longer be manipulated, nor removed.
    fn seal(&self) -> fdo::Result<()>;

    /// SetOption method
    ///
    /// This method allows manipulation of a stored configuration. This is targeted at user front-ends to be able to easily manipulate imported configuration files.
    ///
    /// ** WARNING: ** This method is currently not implemented!
    ///
    /// # Arguments
    ///
    /// * `option` - String containing the name of the option to be modified.
    /// * `value` - String containing the new value of the option.
    fn set_option(&self, option: &str, value: &str) -> fdo::Result<()>;

    /// SetOverride method
    ///
    /// Modifies an override parameter in the configuration profile.
    ///
    /// # Arguments
    ///
    /// * `name` - Property name to modify.
    /// * `value` - The new value the property should have. The data type ov must match the data type of the property in the D-Bus object.
    fn set_override(&self, name: &str, value: &zbus::zvariant::Value<'_>) -> fdo::Result<()>;

    /// UnsetOverride method
    ///
    /// Unset an override setting.
    ///
    /// # Arguments
    ///
    /// * `name` - Override property name to remove.
    fn unset_override(&self, name: &str) -> fdo::Result<()>;

    /// An array of UID values granted access.
    #[dbus_proxy(property, name = "acl")]
    fn acl(&self) -> zbus::Result<Vec<u32>>;

    /// If set to true, the VPN tunnel will make use of the kernel accelerated Data Channel Offload (DCO) feature (requires kernel support).
    #[dbus_proxy(property, name = "dco")]
    fn dco(&self) -> zbus::Result<bool>;
    fn set_dco(&self, value: bool) -> fdo::Result<()>;

    /// Unix Epoch timestamp of the import time.
    #[dbus_proxy(property, name = "import_timestamp")]
    fn import_timestamp(&self) -> zbus::Result<u64>;

    /// Unix Epoch timestamp of the last time it Fetch was called.
    ///
    /// It will track/count `Fetch` usage only if the calling user is `openvpn`.
    #[dbus_proxy(property, name = "last_used_timestamp")]
    fn last_used_timestamp(&self) -> zbus::Result<u64>;

    /// If set to true, only the owner and openvpn user can retrieve the configuration file. Other users granted access can only use this profile to start a new tunnel.
    #[dbus_proxy(property, name = "locked_down")]
    fn locked_down(&self) -> zbus::Result<bool>;
    fn set_locked_down(&self, value: bool) -> fdo::Result<()>;

    /// Contains the user friendly name of the configuration profile
    #[dbus_proxy(property, name = "name")]
    fn name(&self) -> zbus::Result<String>;
    fn set_name(&self, value: &str) -> fdo::Result<()>;

    /// Contains all the override settings enabled. This is stored as a key/value based dictionary, where value can be any arbitrary data type.
    #[dbus_proxy(property, name = "overrides")]
    fn overrides(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// The UID value of the user which did the import.
    #[dbus_proxy(property, name = "owner")]
    fn owner(&self) -> zbus::Result<u32>;

    /// If set to true, this configuration will be saved to disk by the configuration manager. The location of the file storage is managed by the configuration manager itself and the configuration manager will load persistent profiles each time it starts.
    #[dbus_proxy(property, name = "persistent")]
    fn persistent(&self) -> zbus::Result<bool>;

    /// If set to true, access control is disabled. But only owner may change this property, modify the ACL or delete the configuration.
    #[dbus_proxy(property, name = "public_access")]
    fn public_access(&self) -> zbus::Result<bool>;
    fn set_public_access(&self, value: bool) -> fdo::Result<()>;

    /// If set to true, the configuration have been sealed and can no longer be modified.
    #[dbus_proxy(property, name = "readonly")]
    fn readonly(&self) -> zbus::Result<bool>;

    /// If set to true, this configuration profile will be automatically removed after the first `Fetch` call. This is intended to be used by command line clients providing a similar user experience as the OpenVPN 2.x versions provides.
    #[dbus_proxy(property, name = "single_use")]
    fn single_use(&self) -> zbus::Result<bool>;

    /// If set to true, another user granted access to this profile will transfer the VPN session ownership back to the profile owner at start up.
    #[dbus_proxy(property, name = "transfer_owner_session")]
    fn transfer_owner_session(&self) -> zbus::Result<bool>;
    fn set_transfer_owner_session(&self, value: bool) -> fdo::Result<()>;

    /// Number of times Fetch has been called.
    ///
    ///  It will track/count `Fetch` usage only if the calling user is `openvpn`.
    #[dbus_proxy(property, name = "used_count")]
    fn used_count(&self) -> zbus::Result<u32>;

    /// Contains an indication if the configuration profile is considered functional for a VPN session.
    #[dbus_proxy(property, name = "valid")]
    fn valid(&self) -> zbus::Result<bool>;
}
