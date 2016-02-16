
extern crate mio;

use mio::*;
use mio::tcp::*;

const CLIENT_TOKEN: Token = Token(0);

struct Client {
    stream: TcpStream,
    remaining: Vec<Vec<u8>>,
}

impl Client {
    fn ready_read(&mut self) {
    }

    fn ready_write(&mut self) {
    }
}

impl Handler for Client {
    
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Client>, token: Token, events: EventSet) {
        
        if CLIENT_TOKEN != token || events.is_error() {
            return;
        }

        if events.is_writable() && self.remaining.len() != 0 {
            self.ready_write();
        }

        if events.is_readable() {
            self.ready_read();
        }

        let event_set;

        if self.remaining.len() != 0 {
            event_set = EventSet::readable();
        } else {
            event_set = EventSet::readable() | EventSet::writable();
        }
    }
}

fn main() {
    println!("client");
}
