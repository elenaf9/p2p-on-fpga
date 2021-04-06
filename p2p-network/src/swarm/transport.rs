use libp2p::{
    core::{
        muxing::StreamMuxerBox,
        transport::{self, upgrade::Version},
    },
    dns::DnsConfig,
    identity::Keypair,
    noise::{self, NoiseConfig},
    tcp::TcpConfig,
    yamux::YamuxConfig,
    PeerId, Transport,
};

use core::time::Duration;

// Low transport layer that is used in the swarm.
pub struct TransportLayer {
    keypair: Keypair,
}

impl TransportLayer {
    // Create a new TransportLayer with a new generated ed25519 keypair
    pub fn new() -> Self {
        let keypair = Keypair::generate_ed25519();
        TransportLayer { keypair }
    }

    // Create a libp2p transport using TCP with DNS wrapper to allow dns addresses.
    // Upgrade transport with noise-protocol for encryption and Yamux multiplexing.
    pub async fn build(&self) -> transport::Boxed<(PeerId, StreamMuxerBox)> {
        // TCP protocol for sending data
        let tcp_config = TcpConfig::new().nodelay(true);

        // Enable dns adresses in multiaddress
        let transport = DnsConfig::system(tcp_config).await.unwrap();

        // Noise encryption with Diffie-Hellman key exchange
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&self.keypair)
            .unwrap();

        // Upgrade the transport with Yamux multiplexing and noise-authentication with xx-handshake
        transport
            .upgrade(Version::V1)
            .authenticate(NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(YamuxConfig::default())
            .timeout(Duration::from_secs(30))
            .boxed()
    }

    // Return the underlying keypair.
    // Only used for building gossipsub protocol.
    // Not safe because the private key is revelead.
    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }

    // Local peer id derived from the public key.
    pub fn local_peer_id(&self) -> PeerId {
        PeerId::from(self.keypair.public())
    }
}
