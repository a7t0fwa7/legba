use std::time::Duration;

use async_trait::async_trait;
use ctor::ctor;
use vnc::{PixelFormat, VncConnector};

use crate::session::{Error, Loot};
use crate::Plugin;
use crate::{utils, Options};

use crate::creds::Credentials;

#[ctor]
fn register() {
    crate::plugins::manager::register("vnc", Box::new(VNC::new()));
}

#[derive(Clone)]
pub(crate) struct VNC {
    address: String,
}

impl VNC {
    pub fn new() -> Self {
        VNC {
            address: String::new(),
        }
    }
}

#[async_trait]
impl Plugin for VNC {
    fn description(&self) -> &'static str {
        "VNC password authentication."
    }

    fn single_credential(&self) -> bool {
        true
    }

    fn setup(&mut self, opts: &Options) -> Result<(), Error> {
        let (host, port) = utils::parse_target(opts.target.as_ref(), 5900)?;
        self.address = format!("{}:{}", host, port);

        Ok(())
    }

    async fn attempt(&self, creds: &Credentials, timeout: Duration) -> Result<Option<Loot>, Error> {
        let stream = crate::utils::net::async_tcp_stream(&self.address, timeout, false).await?;
        // being this plugin single credentials, this is going to be the password
        let password = creds.single().to_owned();
        let vnc = tokio::time::timeout(
            timeout,
            VncConnector::new(stream)
                .set_auth_method(async move { Ok(password) })
                .add_encoding(vnc::VncEncoding::Tight)
                .add_encoding(vnc::VncEncoding::Zrle)
                .add_encoding(vnc::VncEncoding::CopyRect)
                .add_encoding(vnc::VncEncoding::Raw)
                .allow_shared(false)
                .set_pixel_format(PixelFormat::bgra())
                .build()
                .map_err(|e| e.to_string())?
                .try_start(),
        )
        .await
        .map_err(|e| e.to_string())?;

        if vnc.is_ok() && vnc.unwrap().finish().is_ok() {
            return Ok(Some(Loot::from([
                ("username".to_owned(), creds.username.to_owned()),
                ("password".to_owned(), creds.password.to_owned()),
            ])));
        }

        Ok(None)
    }
}
