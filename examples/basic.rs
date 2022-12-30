//! This example uses the asynchronous proxy instances.
//!
//! The client configuration must be passed as a string blob, including any certificates/keys inline (PEM format). A basic username/password authentication handler is also included.
//!

use async_std::task;
use openvpn3_rs::NetCfgProxy;

static CONFIG_STR: &str = "
client
dev tun
persist-tun
persist-key
proto udp
remote my-server 1194
resolv-retry infinite
<ca>
-----BEGIN CERTIFICATE-----
-----END CERTIFICATE-----
</ca>
<cert>
-----BEGIN CERTIFICATE-----
-----END CERTIFICATE-----
</cert>
<key>
-----BEGIN PRIVATE KEY-----
-----END PRIVATE KEY-----
</key>
<tls-auth>
-----BEGIN OpenVPN Static key V1-----
-----END OpenVPN Static key V1-----
</tls-auth>
key-direction 1
remote-cert-tls server
auth-user-pass
pull
";

fn main() {
    task::block_on(async {
        let connection = zbus::Connection::system().await.unwrap();
        let config_manager = openvpn3_rs::ConfigurationProxy::new(&connection)
            .await
            .unwrap();
        let config = config_manager
            .import("My VPN", CONFIG_STR, true, false)
            .await
            .unwrap();

        let sessions_manager = openvpn3_rs::SessionsProxy::new(&connection).await.unwrap();
        let session = sessions_manager.new_tunnel(&config.path()).await.unwrap();

        let mut ready = false;
        while !ready {
            // If the session is ready, the `ready()` method will return Ok(), otherwise an error will be returned with more details.
            if let Err(err) = session.ready().await {
                let err_str = err.to_string();
                if err_str.contains("Missing user credentials") {
                    // This loop queries the session for which credentials are needed. This example covers username/password authentication.

                    let ui_type_group = session.user_input_queue_get_type_group().await.unwrap();

                    for (ca_type, ca_group) in ui_type_group {
                        let ui_queue_ids = session
                            .user_input_queue_check(ca_type, ca_group)
                            .await
                            .unwrap();

                        for id in ui_queue_ids {
                            let (ca_type, ca_group, id, name, _description, _hidden_input) =
                                session
                                    .user_input_queue_fetch(ca_type, ca_group, id)
                                    .await
                                    .unwrap();

                            if name == "username" {
                                session
                                    .user_input_provide(ca_type, ca_group, id, "smith")
                                    .await
                                    .unwrap();
                            }

                            if name == "password" {
                                session
                                    .user_input_provide(ca_type, ca_group, id, "hunter2")
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                } else if err_str.contains("Backend VPN process is not ready") {
                    task::sleep(std::time::Duration::from_secs(1)).await;
                }
            } else {
                ready = true;
            }
        }

        session.connect().await.unwrap();

        // wait for signal to disconnect

        session.disconnect().await.unwrap();
    });
}
