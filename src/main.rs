use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

pub enum Features {
    Video = 1,
    Photo = 1 << 1,
    VideoFairPlay = 1 << 2,
    VideoVolumeControl = 1 << 3,
    VideoHTTPLiveStreams = 1 << 4,
    Slideshow = 1 << 5,

    Screen = 1 << 7,
    ScreenRotate = 1 << 8,
    Audio = 1 << 9,

    AudioRedundant = 1 << 11,
    Fpsapv2pt5AesGcm = 1 << 12,
    PhotoCaching = 1 << 13,
    Authentication4 = 1 << 14,
    MetadataFeature1 = 1 << 15,
    MetadataFeature2 = 1 << 16,
    MetadataFeature0 = 1 << 17,
    AudioFormat1 = 1 << 18,
    AudioFormat2 = 1 << 19,
    AudioFormat3 = 1 << 20,
    AudioFormat4 = 1 << 21,

    Authentication1 = 1 << 23,

    HasUnifiedAdvertiserInfo = 1 << 26,
    SupportsLegacyPairing = 1 << 27,

    RAOP = 1 << 30,

    IsCarPlaySupportsVolume = 1 << 32,
    SupportsAirPlayVideoPlayQueue = 1 << 33,
    SupportsAirPlayFromCloud = 1 << 34,

    SupportsCoreUtilsPairingAndEncryption = 1 << 38,
    SupportsBufferedAudio = 1 << 40,
    SupportsPTP = 1 << 41,
    SupportsScreenMultiCodec = 1 << 42,
    SupportsSystemPairing = 1 << 43,

    SupportsHKPairingAndAccessControl = 1 << 46,

    SupportsTransientPairing = 1 << 48,

    MetadataFeature4 = 1 << 50,
    SupportsUnifiedPairSetupAndMFi = 1 << 51,
    SupportsSetPeersExtendedMessage = 1 << 52,
}

pub enum Status {
    ProblemDetected = 1,
    NotConfigured = 1 << 1,
    AudioCableAttached = 1 << 2,
    PinRequired = 1 << 3,

    SupportsAirPlayFromCloud = 1 << 6,
    PasswordRequired = 1 << 7,

    OneTimePairingRequired = 1 << 9,
    DeviceWasSetupForHKAccessControl = 1 << 10,
    DeviceSupportsRelay = 1 << 11,
    SilentPrimary = 1 << 12,
    TightSyncIsGroupLeader = 1 << 13,
    TightSyncBuddyNotReachable = 1 << 14,
    IsAppleMusicSubscriber = 1 << 15,
    CloudLibraryIsOn = 1 << 16,
    ReceiverSessionIsActive = 1 << 17,
}

pub enum AudioEncoding {
    Alac = 0,
    PCM = 1
}

pub enum AirPlaySecurity {
    None = 0,
    Pin = 1,
    Password = 2,
}

pub struct AirplayDevice {
    active: bool,
    audio_supported: bool,
    host: String,
    port: u16,
    airplay2: bool,
    encoding: AudioEncoding,
    security: AirPlaySecurity,
    transient: bool,
    legacy_shairport: bool,
    sonos_mfi: bool
}

fn main() {
    // Create a daemon
    let raop_mdns = String::from("_raop._tcp.local.");
    let airplay_mdns: String = String::from("_airplay._tcp.local.");
    let mut all_devices : Vec<ServiceInfo> = Vec::new();
    all_devices.append(&mut scan_devices( &raop_mdns, 3));
    all_devices.append(&mut scan_devices( &airplay_mdns, 3));
    println!("Found {} devices", all_devices.len());
    for device in all_devices {
            //  println!(
            //     "Found new airplay devices: {} host: {} port: {} IP: {:?} TXT properties: {:?}",
            //         device.get_fullname(),
            //         device.get_hostname(),
            //         device.get_port(),
            //         device.get_addresses(),
            //         device.get_properties(),
            //  );
            device_conv(device);
    }
    
}

/* Scan all devices */
fn scan_devices(service_type: &str, timeout: u64) -> Vec<ServiceInfo> {
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");
    let mut devices: Vec<ServiceInfo> = Vec::new();
    let receiver = mdns.browse(&service_type).expect("Failed to browse");

    let now = std::time::Instant::now();
    while let Ok(event) = receiver.recv() {
        if now.elapsed().as_secs() > timeout {
            break;
        }
        match event {
            ServiceEvent::ServiceResolved(info) => {
                // println!(
                //     "Resolved a new service: {} host: {} port: {} IP: {:?} TXT properties: {:?}",
                //     info.get_fullname(),
                //     info.get_hostname(),
                //     info.get_port(),
                //     info.get_addresses(),
                //     info.get_properties(),
                // );
                devices.push(info);
            }
            _other_event => {
            }
        }
    }
    mdns.stop_browse(&service_type).expect("Failed to shutdown daemon");
    mdns.shutdown().expect("Failed to shutdown daemon");
    devices

   
}

/* Convert ServiceInfo to our own managable struct */
fn device_conv(device : ServiceInfo) 
//-> AirplayDevice 
{
    let mut airplay2 = false;
    if device.get_fullname().contains("airplay") {
        airplay2 = true;
    }
    for property in device.get_properties().iter(){
     //   println!("{}: {}", property.key(), property.val_str());

        if property.key() == "features" || property.key() == "ft" {
            // Split the features key into 2 hex strings (if there is a comma)
            println!("Feature string : {}", property.val_str());
            let features_strings: Vec<&str> = property.val_str().split(",").collect();
            if features_strings.len() == 2 {
                let features1 = features_strings[0].trim_start_matches("0x");
                let features2 = features_strings[1].trim_start_matches("0x");
                let mut features_joined = String::from(features2);
                features_joined.push_str(features1);
                let features = u64::from_str_radix( &features_joined, 16).unwrap();
                println!("Feature: {}", features);
            }
            else {
                let features = u64::from_str_radix(property.val_str().trim_start_matches("0x"), 16).unwrap();
                println!("Feature: {}", features);
            }
            

            
        }
    }
}

