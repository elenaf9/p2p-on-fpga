use libp2p::{
    core::{
        muxing::StreamMuxerBox,
        transport::{self, upgrade::Version},
    },
    dns::DnsConfig,
    identity::{Keypair, PublicKey},
    noise::{self, NoiseConfig},
    tcp::TcpConfig,
    yamux::YamuxConfig,
    PeerId, Transport,
};

use crate::chain;
use core::time::Duration;

use std::{env, fs, path::Path};

pub struct TransportLayer {
    keypair: Keypair,
}

impl TransportLayer {
    pub fn new() -> Result<Self, ()> {
        let path = TransportLayer::get_keys_path();
        let read_keypair = fs::read(path.join("private.pk8"));
        let keypair = chain!(read_keypair => |mut bytes| Keypair::secp256k1_from_der(&mut bytes))
            .unwrap_or_else(|()| Keypair::generate_ed25519());
        Ok(TransportLayer { keypair })
    }

    pub async fn build(&self) -> transport::Boxed<(PeerId, StreamMuxerBox)> {
        // tcp layer
        let tcp_config = TcpConfig::new().nodelay(true);
        // enable dns adresses in multiaddress
        let transport = DnsConfig::system(tcp_config)
            .await
            .unwrap();
        // noise encryption with Diffie-Hellman key exchange
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&self.keypair)
            .unwrap();
        // upgrade the transport with multiplexing and noise-authentication with xx-handshake
        transport
            .upgrade(Version::V1)
            .authenticate(NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(YamuxConfig::default())
            .timeout(Duration::from_secs(30))
            .boxed()
    }

    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }

    pub fn public_key(&self) -> PublicKey {
        self.keypair.public()
    }

    pub fn local_peer_id(&self) -> PeerId {
        PeerId::from(self.public_key())
    }

    fn get_keys_path() -> Box<Path> {
        env::var("P2P_NET_PATH")
            .map(|path| Path::new(&path).into())
            .unwrap_or_else(|_| Path::new(".p2p").into())
    }
}
