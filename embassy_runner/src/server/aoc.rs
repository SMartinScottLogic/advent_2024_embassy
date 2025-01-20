use defmt::{error, info, warn};
use embassy_net::tcp::TcpSocket;
use embedded_io_async::Write as _;

use super::SocketServer;
//use crate::aoc::Solution;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }
}

impl SocketServer for Server {
    async fn run(&mut self, mut socket: TcpSocket<'_>) {
        crate::aoc::run_sample();
        crate::aoc::run_full();

        let mut buf = [0; 4096];
        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    warn!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    warn!("read error: {:?}", e);
                    break;
                }
            };

            info!("rxd {}", core::str::from_utf8(&buf[..n]).unwrap());
        }

//         if let Ok(mut solution) = crate::aoc::day1::Solution::try_from(
//             r"3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3",
//         ) {
//             solution.analyse(true);
//             match solution.answer_part1(true) {
//                 Ok(answer) => {
//                     info!("Part 1 answer: {}", answer)
//                 }
//                 Err(e) => {
//                     info!("Part 1 FAILED: {:?}", e)
//                 }
//             }
//             match solution.answer_part2(true) {
//                 Ok(answer) => {
//                     info!("Part 2 answer: {}", answer)
//                 }
//                 Err(e) => {
//                     info!("Part 2 FAILED: {:?}", e)
//                 }
//             }
//         }

        match socket.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await {
            Ok(()) => {}
            Err(e) => {
                error!("write error: {:?}", e);
            }
        };
    }
}

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();
