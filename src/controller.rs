use mio::tcp::*;
use mio::util::Slab;
use ::SERVER;
use connection::Connection;

pub struct ControllerHandler {
    pub server: TcpListener,
    pub connections : Slab<Connection>
}

impl ControllerHandler {
    pub fn new(server : TcpListener) -> ControllerHandler {
        ControllerHandler {
            server: server,
            connections: Slab::new_starting_at(::mio::Token(1), 1024)
        }
    }
}


impl ::mio::Handler for ControllerHandler {
    type Timeout = ();
    type Message = ();

    fn ready(
        &mut self,
        event_loop: &mut ::mio::EventLoop<ControllerHandler>,
        token: ::mio::Token,
        events: ::mio::EventSet
    ) {
        match token {
            SERVER => {
                // Only receive readable events
                assert!(events.is_readable());

                println!("the server socket is ready to accept a connection");
                // http://rustdoc.s3-website-us-east-1.amazonaws.com/mio/v0.5.x/mio/tcp/struct.TcpListener.html#method.accept
                match self.server.accept() {
                    Ok(Some(socket)) => {
                        println!("accepted a socket, exiting program");

                        let stream = socket.0;

                        let connection = self
                            .connections
                            .insert_with(|connection| Connection::new(stream, connection))
                            .expect("failed to handle connection, increase slab size?");

                        event_loop.register(
                            &self.connections[connection].stream,
                            connection,
                            ::mio::EventSet::readable(),
                            ::mio::PollOpt::edge() | ::mio::PollOpt::oneshot()
                        ).expect("cant register socket on event loop");
                        //event_loop.shutdown();
                    }
                    Ok(None) => {
                        println!("the server socket wasn't actually ready");
                    }
                    Err(e) => {
                        println!("listener.accept() errored: {}", e);
                        event_loop.shutdown();
                    }
                }
            },
            _ => match self.connections.get(token) {
                Some(connection) => {
                    panic!("i've go a connection!")
                },
                None => panic!("Received unknown token")
            }
        }
    }
}
