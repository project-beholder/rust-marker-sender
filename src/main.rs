pub mod marker_detection;

use marker_detection::create_marker_detector;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:37374").expect("Couldn't bind to address");
    socket.connect("127.0.0.1:37373").expect("Couldn't connect to port");
    let mut marker_detector = create_marker_detector();

    loop {
        // Detect and update markers
        marker_detector.observation_loop();

        let send_str = marker_detector.print_markers();
        let send_bytes = send_str.as_bytes();
        socket.send(send_bytes).expect("Couldn't send message");
    }
}
