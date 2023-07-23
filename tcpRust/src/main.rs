extern crate tun_tap;
use std::io;


fn main()  -> io::Result<()> {
    println!("Hello, world!");
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop{
        let nbytes = nic.recv(&mut buf[..])?;
        eprintln!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    }
    
    Ok(( ))

}
