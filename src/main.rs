pub mod discovery;
mod player;

use discovery::get_airplay_devices;

use player::udp_servers::UDPServers;

// fn main(){
//     let all_devices = get_airplay_devices();
//     println!("Found {} devices", all_devices.len());
//     for airtune_device in all_devices {
//         println!("Device: {} Host: {}:{} Airplay2: {} Audio: {} Encoding: {:?} Security: {:?} Transient: {} Sonos: {}",
//             airtune_device.name, airtune_device.host, airtune_device.port, airtune_device.airplay2, airtune_device.audio_supported, airtune_device.encoding as u64, airtune_device.security as u64, airtune_device.transient, airtune_device.sonos_mfi);
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let servers = UDPServers::new().await?;
    println!("Timing Port: {}", servers.timing_port);
    println!("Control Port: {}", servers.control_port);
    servers.run().await;
    

    Ok(())
}