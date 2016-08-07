use mio::tcp::*;
use mio::util::Slab;
use ::SERVER;

pub struct ControllerHandler {
    pub server: TcpListener,
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
                        event_loop.shutdown();
                    }
                    Ok(None) => {
                        println!("the server socket wasn't actually ready");
                    }
                    Err(e) => {
                        println!("listener.accept() errored: {}", e);
                        event_loop.shutdown();
                    }
                }
            }
            _ => panic!("Received unknown token"),
        }
    }
}
