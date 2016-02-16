
extern crate mio;

use mio::*;

const SERVER_TOKEN: Token = Token(0);

struct Server {
    server: tcp::TcpListener,
}

impl Handler for Server {
    
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Server>, token: Token, events: EventSet) {

        if token != SERVER_TOKEN {
            return;
        }


    
    }

}

fn main() {
    println!("server");
}
