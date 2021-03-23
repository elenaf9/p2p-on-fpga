use libp2p::{
    core::{
        muxing::StreamMuxerBox,
        transport::{self, upgrade::Version},
    },
    dns::DnsConfig,
    identity::{Keypair, PublicKey},
    noise::{self, NoiseConfig},
    pnet::{PnetConfig, PreSharedKey},
    tcp::TcpConfig,
    yamux::YamuxConfig,
    PeerId, Transport,
};

use core::{str::FromStr, time::Duration};

use std::{env, fs, path::Path};

pub struct TransportLayer {
    keypair: Keypair,
    psk: PreSharedKey,
}

impl TransportLayer {
    pub fn new() -> Result<Self, ()> {
        let path = TransportLayer::get_keys_path();
        let psk = fs::read_to_string(path.join("psk"))
            .map_err(|_| ())
            .and_then(|psk_str| PreSharedKey::from_str(&psk_str).map_err(|_| ()))?;
        let keypair = fs::read(path.join("private.pk8"))
            .ok()
            .and_then(|mut bytes| Keypair::secp256k1_from_der(&mut bytes).ok())
            .unwrap_or_else(Keypair::generate_ed25519);
        Ok(TransportLayer { keypair, psk })
    }

    pub async fn build(&self) -> transport::Boxed<(PeerId, StreamMuxerBox)> {
        // tcp layer
        let tcp_config = TcpConfig::new().nodelay(true);
        // noise encryption with Diffie-Hellman key exchange
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&self.keypair)
            .unwrap();
        // enable dns adresses in multiaddress
        let psk = self.psk;
        let transport = DnsConfig::system(tcp_config)
            .await
            .unwrap()
            // additional security by using a pre-shared key within one network
            .and_then(move |socket, _| PnetConfig::new(psk).handshake(socket));
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
