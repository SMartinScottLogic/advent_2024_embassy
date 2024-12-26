use embassy_net::tcp::TcpSocket;

pub mod aoc;

pub trait SocketServer {
    async fn run(&mut self, socket: TcpSocket)
    where
        Self: Sized;
}
