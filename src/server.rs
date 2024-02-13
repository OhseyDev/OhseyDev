extern crate dotenvy;

struct Addresses {
    http: SocketAddr,
    https: SockerAddr,
}

struct Listeners {
    http: TcpListener,
    https: TcpListener,
}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv()?;
    let pub_cert = dotenvy::var("PUB_CERT")?;
    let priv_cert = dotenvy::var("PRIV_CERT")?;
   
    let addresses = Addresses {
        http: SocketAddr::from(([127, 0, 0, 1], 80)),
        https: SockerAddr::from(([127, 0, 0, 1], 443))
    };
    let listeners = Listeners {
        http: TcpListener::bind(addresses.http).await?,
        https: TcpListener::bind(addresses.https).await?
    };
    
}

