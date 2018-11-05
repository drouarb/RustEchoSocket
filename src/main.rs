extern crate tokio;
extern crate tokio_core;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_core::reactor::Core;

fn main() {

    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(|socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

        // Process socket here.

        let buf = vec![0, 0, 0, 0];
        let connection = io::read_exact(socket, buf).and_then(|(socket, buf)| {
            println!("{:?}", buf);
            io::write_all(socket, buf)
        }).then(|_| Ok(()));

        tokio::spawn(connection);

        Ok(())
    })
        .map_err(|err| {
            // All tasks must have an `Error` type of `()`. This forces error
            // handling and helps avoid silencing failures.
            //
            // In our example, we are only going to log the error to STDOUT.
            println!("accept error = {:?}", err);
        });

    println!("Running LOL");
    tokio::run(server)
}
