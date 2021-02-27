use kyoto_data::Server;
use kyoto_network::Listener;
use kyoto_protocol::Result;

/* Main function for kyoto.
 * Start a webserver to listen to given port and accept new connections. */
pub fn main() -> Result<()> {
    /* Enable logging diagnostics. */
    tracing_subscriber::fmt::try_init()?;

    let server = Server::new();
    let mut listener = Listener::new(server);
    listener.run()?;
    Ok(())
}