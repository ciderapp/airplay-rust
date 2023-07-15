pub mod discovery;
mod player;

use discovery::get_airplay_devices;

use player::udp_servers::UDPServers;
use tokio::{task, time};

// fn main(){
//     let all_devices = get_airplay_devices();
//     println!("Found {} devices", all_devices.len());
//     for airtune_device in all_devices {
//         println!("Device: {} Host: {}:{} Airplay2: {} Audio: {} Encoding: {:?} Security: {:?} Transient: {} Sonos: {}",
//             airtune_device.name, airtune_device.host, airtune_device.port, airtune_device.airplay2, airtune_device.audio_supported, airtune_device.encoding as u64, airtune_device.security as u64, airtune_device.transient, airtune_device.sonos_mfi);
//     }
// }

#[tokio::main]
async fn main(){
    let udp_servers = UDPServers::new().await.unwrap();
    println!("Timing Port: {}", udp_servers.timing_port);
    println!("Control Port: {}", udp_servers.control_port);
    task::spawn(async {
      udp_servers.run().await;
    });  
}