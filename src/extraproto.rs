//! TCP port 8046

use std::{
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
};

/// The TCP socket address of the control socket.
pub const CONTROL_SOCKET_ADDR: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)), 8046);

pub fn query_battery_level_sync() -> std::io::Result<u8> {
    let mut sock = TcpStream::connect(CONTROL_SOCKET_ADDR)?;
    sock.write_all(b"SMPH\x02\0\0")?;

    let mut buf = [0; 7];
    sock.read_exact(&mut buf)?;
    Ok(buf[6])
}

pub fn set_light_level_sync(light_level: u8) -> std::io::Result<()> {
    assert!(
        light_level <= 3,
        "{} should be less than or equal to 3",
        light_level
    );
    let mut sock = TcpStream::connect(CONTROL_SOCKET_ADDR)?;

    let buf = [b'S', b'M', b'P', b'H', 1, 0, light_level];
    sock.write_all(&buf)?;
    Ok(())
}

pub fn set_password_sync(password: &str) -> std::io::Result<()> {
    let mut sock = TcpStream::connect(CONTROL_SOCKET_ADDR)?;

    let mut buf = vec![b'S', b'M', b'P', b'H', 1, 1];
    buf.extend_from_slice(password.as_bytes());
    sock.write_all(&buf)?;
    Ok(())
}
