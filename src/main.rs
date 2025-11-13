use std::net::UdpSocket;
use std::time::Duration;
use std::time::Instant;
/*
 * AprilTags listens to UDP packets with April tags on a port.
 * Each UDP packet contains N AprilTag updates.
 * 
 * UDP Packet Format:
 * Header:
 *   - Number Of Tags In Packet : byte
 *   - Padding: 3 bytes
 * Each Tag: 
 *   - April Tag ID  : int (4 bytes) --- really just 0-255
 *   - Timestamp (seconds): float (4 bytes)
 *   - x (meters) : float (4 bytes)
 *   - y (meters) : float (4 bytes)
 *   - z (meters) : float ( 4 bytes)
 *   - theta (radians) : float (4 bytes)
 *   - confidence (radians) : float ( 4 bytes)
 *   - 28 bytes/tag + 4 bytes => 36 tags / packet max.
 */

struct Tag
{
    id: u8,
    timestamp: f32,
    x: f32,
    y: f32,
    z: f32,
    theta: f32,
    confidence: f32,
}

fn serialize_tags(tags: Vec<Tag>) -> Vec<u8>{
    let mut buffer: Vec<u8> = Vec::new();
    buffer.push(tags.len() as u8);
        buffer.extend_from_slice(&[0u8; 3]);
    for tag in tags {
        buffer.extend_from_slice(&(tag.id as i32).to_le_bytes());
        buffer.extend_from_slice(&tag.timestamp.to_le_bytes());
        buffer.extend_from_slice(&tag.x.to_le_bytes());
        buffer.extend_from_slice(&tag.y.to_le_bytes());
        buffer.extend_from_slice(&tag.z.to_le_bytes());
        buffer.extend_from_slice(&tag.theta.to_le_bytes());
        buffer.extend_from_slice(&tag.confidence.to_le_bytes());
    }
    return buffer;
}

fn print_tag(tag: &Tag) {
    println!(
        "Tag ID: {}, timestamp: {:.2}, x: {:.2}, y: {:.2}, z: {:.2}, theta: {:.2}, confidence: {:.2}",
        tag.id, tag.timestamp, tag.x, tag.y, tag.z, tag.theta, tag.confidence
    );
}

// Print an array of tags
fn print_tags(tags: &Vec<Tag>) {
    for tag in tags {
        print_tag(tag);
    }
}

fn main() -> std::io::Result<()>{
    let mut x = 0;
    loop {
        let start = Instant::now();
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_write_timeout(Some(Duration::from_secs(1)))?;
        x+=1;
        let target = "127.0.0.1:4567";
        let mut tags: Vec<Tag> = Vec::new();
        for i in 0..13 {
            tags.push(Tag {
                id: i,
                timestamp: x as f32,
                x: i as f32 * 0.1,
                y: i as f32 * 0.2,
                z: i as f32 * 0.3,
                theta: 0.01 * i as f32,
                confidence: 0.5,
            });
        }
        print_tags(&tags);

        let message = serialize_tags(tags);
        socket.send_to(&message, target)?;

        println!("Sent message to {}", target);
        std::thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
