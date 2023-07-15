use std::net::SocketAddr;
use tokio::net::UdpSocket;
use socket2::{Socket, Domain, Type};
use super::ntp::NTP;
use super::utils::get_local_ip_port ;
use std::sync::Arc;

struct TimingServer {
    socket: UdpSocket,
    hosts: String,
    ntp: NTP,
    port: u16,
}

impl TimingServer {
    async fn new() -> Result<Self, std::io::Error> {
        let domain = Domain::IPV4;
        let socket = Socket::new(domain, Type::DGRAM, None)?;
        let hosts = get_local_ip_port().unwrap();
        // get port from hosts string
        let port = hosts.split(":").collect::<Vec<&str>>()[1].parse::<u16>().unwrap();
        socket.set_reuse_address(true)?;
        socket.bind( &hosts.parse::<SocketAddr>().unwrap().into())?;

        let socket = UdpSocket::from_std(socket.into())?;
        Ok(TimingServer { socket, hosts, ntp: NTP::new(), port })
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    async fn run(&self) -> Result<(), std::io::Error> {
        loop {
            let mut buf = [0u8; 1024];
            let (size, addr) = self.socket.recv_from(&mut buf).await?;

            let msg = &buf[..size];

            if !self.hosts.contains(&addr.ip().to_string()) {
                continue;
            }

            let ts1 = u32::from_be_bytes([msg[24], msg[25], msg[26], msg[27]]);
            let ts2 = u32::from_be_bytes([msg[28], msg[29], msg[30], msg[31]]);

            let mut reply = [0u8; 32];
            reply[..2].copy_from_slice(&0x80d3u16.to_be_bytes());
            reply[2..4].copy_from_slice(&0x0007u16.to_be_bytes());
            reply[4..8].copy_from_slice(&0x00000000u32.to_be_bytes());

            reply[8..12].copy_from_slice(&ts1.to_be_bytes());
            reply[12..16].copy_from_slice(&ts2.to_be_bytes());

            let ntp_time = self.ntp.timestamp(); // Replace with your NTP timestamp logic

            reply[16..24].copy_from_slice(&ntp_time);
            reply[24..32].copy_from_slice(&ntp_time);

            self.socket.send_to(&reply, addr).await?;
        }
    }

}

struct ControlServer {
    socket: UdpSocket,
    hosts: String,
    port: u16,
}

impl ControlServer {
    async fn new() -> Result<Self, std::io::Error> {
        let domain = Domain::IPV4;
        let socket = Socket::new(domain, Type::DGRAM, None)?;
        let hosts = get_local_ip_port().unwrap();
        // get port from hosts string
        let port = hosts.split(":").collect::<Vec<&str>>()[1].parse::<u16>().unwrap();
        socket.set_reuse_address(true)?;
        socket.bind( &hosts.parse::<SocketAddr>().unwrap().into())?;

        let socket = UdpSocket::from_std(socket.into())?;
        Ok(ControlServer { socket, hosts, port})
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    async fn run(&self) -> Result<(), std::io::Error> {
        let mut buf = [0u8; 1024];
        loop {
            let (size, addr) = self.socket.recv_from(&mut buf).await?;

            let msg = &buf[..size];

            if !self.hosts.contains(&addr.ip().to_string()) {
                continue;
            }

            let resend_requested = msg[1] == (0x80 | 0x55);
            if resend_requested {
                let missed_seq = u16::from_be_bytes([msg[4], msg[5]]);
                let count = u16::from_be_bytes([msg[6], msg[7]]);
               // self.resend_requested(missed_seq, count).await;
            }
        }
    }

    async fn resend_requested(&self, missed_seq: u16, count: u16) {
        // Handle resendRequested logic here
        // You can emit an event or perform any other action
        println!("Resend requested: Missed Seq: {}, Count: {}", missed_seq, count);
    }
}

pub struct UDPServers {
    control_server: Arc<ControlServer>,
    timing_server: Arc<TimingServer>,
    pub timing_port: u16,
    pub control_port: u16,

}

impl UDPServers {
    pub async fn new(
    ) -> Result<Self, std::io::Error> {
        let control_server = Arc::new(ControlServer::new().await?);
        let timing_server = Arc::new(TimingServer::new().await?);
        let timing_port = timing_server.get_port();
        let control_port = control_server.get_port();

        Ok(UDPServers {
            control_server,
            timing_server,
            timing_port,
            control_port
        })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let control_handle = tokio::spawn(async move {
            self.control_server.run().await
        });
        let timing_handle = tokio::spawn(async move {
            self.timing_server.run().await
        });

        control_handle.await?;
        timing_handle.await?;

        Ok(())
    }

    pub fn control_port(&self) -> u16 {
        self.control_server.get_port()
    }

    pub fn timing_port(&self) -> u16 {
        self.timing_server.get_port()
    }

}