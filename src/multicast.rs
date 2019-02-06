use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

lazy_static! {
    //pub static ref IPV4: IpAddr = Ipv4Addr::new(224, 0, 0, 123).into();
    //pub static ref IPV6: IpAddr = Ipv6Addr::new(0xFF03, 0, 0, 0, 0, 0, 0, 0x0123).into();
    pub static ref STOP: &'static str = "stop";
}

// this will be common for all our sockets
fn new_socket(addr: &SocketAddr) -> io::Result<Socket> {
    let domain = if addr.is_ipv4() {
        Domain::ipv4()
    } else {
        Domain::ipv6()
    };

    let socket = Socket::new(domain, Type::dgram(), Some(Protocol::udp()))?;

    // we're going to use read timeouts so that we don't hang waiting for packets
    socket.set_read_timeout(Some(Duration::from_millis(100)))?;

    Ok(socket)
}

/// On Windows, unlike all Unix variants, it is improper to bind to the multicast address
///
/// see https://msdn.microsoft.com/en-us/library/windows/desktop/ms737550(v=vs.85).aspx
#[cfg(windows)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    let addr = match *addr {
        SocketAddr::V4(addr) => SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), addr.port()),
        SocketAddr::V6(addr) => {
            SocketAddr::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(), addr.port())
        }
    };
    socket.bind(&socket2::SockAddr::from(addr))
}

/// On unixes we bind to the multicast address, which causes multicast packets to be filtered
#[cfg(unix)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    socket.bind(&socket2::SockAddr::from(*addr))
}

fn join_multicast(addr: SocketAddr) -> io::Result<UdpSocket> {
    let ip_addr = addr.ip();

    let socket = new_socket(&addr)?;

    // depending on the IP protocol we have slightly different work
    match ip_addr {
        IpAddr::V4(ref mdns_v4) => {
            // join to the multicast address, with all interfaces
            socket.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0, 0, 0, 0))?;
        }
        IpAddr::V6(ref mdns_v6) => {
            // join to the multicast address, with all interfaces (ipv6 uses indexes not addresses)
            socket.join_multicast_v6(mdns_v6, 0)?;
            socket.set_only_v6(true)?;
        }
    };

    socket.set_reuse_address(true);
    
    #[cfg(unix)]
    socket.set_reuse_port(true);

    #[cfg(unix)]
    info!("Warning: Cannot set reuse of ports on Windows.");

    // bind us to the socket address.
    bind_multicast(&socket, &addr)?;

    // convert to standard sockets
    Ok(socket.into_udp_socket())
}

pub fn multicast_listener(
    addr: SocketAddr,
) -> JoinHandle<()> {
    // A barrier to not start the client test code until after the server is running
    let server_barrier = Arc::new(Barrier::new(2));
    let client_barrier = Arc::clone(&server_barrier);

    let join_handle = std::thread::Builder::new()
        .name(format!("dot:listener"))
        .spawn(move || {
            // socket creation will go here...
            let listener = join_multicast(addr).expect("failed to create listener");
            println!("dot:listener: joined: {}",  addr);

            server_barrier.wait();
            println!("dot:listener: is ready");

            let mut stop = false;
            //listener.set_nonblocking(true).unwrap();
            while !stop {
                // test receive and response code will go here...
                let mut buf = [0u8; 1024]; // receive buffer
                
                match listener.recv_from(&mut buf) {
                    Ok((len, remote_addr)) => {
                        let data = &buf[..len];
                        
                        // break when stop message recevied
                        if *STOP.as_bytes() == *data.clone() {
                            stop = true;
                        }

                        println!(
                            "dot:listener: received request: {} from: {}",
                            String::from_utf8_lossy(data),
                            remote_addr
                        );
                    }
                    Err(err) => {
                        //println!("dot:listener: got an error: {}", err);
                    }
                }    
            }

            println!("dot:listener: stopped!");
        })
        .unwrap();

    client_barrier.wait();
    join_handle
}

pub fn new_sender(addr: &SocketAddr) -> io::Result<UdpSocket> {
    let socket = new_socket(addr)?;

    if addr.is_ipv4() {
        socket.set_multicast_if_v4(&Ipv4Addr::new(0, 0, 0, 0))?;

        socket.bind(&SockAddr::from(SocketAddr::new(
            Ipv4Addr::new(0, 0, 0, 0).into(),
            0,
        )))?;
    } else {
        // *WARNING* THIS IS SPECIFIC TO THE AUTHORS COMPUTER
        //   find the index of your IPv6 interface you'd like to test with.
        socket.set_multicast_if_v6(5)?;

        socket.bind(&SockAddr::from(SocketAddr::new(
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(),
            0,
        )))?;
    }

    // convert to standard sockets...
    Ok(socket.into_udp_socket())
}
