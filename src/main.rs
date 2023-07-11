use std::net::Ipv4Addr;

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
    PCM = 0,
    Alac = 1
}

pub enum AirPlaySecurity {
    None = 0,
    Pin = 1,
    Password = 2,
}

pub struct AirplayDevice {
    active: bool,
    audio_supported: bool,
    name: String,
    host: Ipv4Addr,
    port: u16,
    airplay2: bool,
    encoding: AudioEncoding,
    security: AirPlaySecurity,
    transient: bool,
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
            let airtune_device = device_conv(device);
            println!("Device: {} Host: {}:{} Airplay2: {} Audio: {} Encoding: {:?} Security: {:?} Transient: {} Sonos: {}",
            airtune_device.name, airtune_device.host, airtune_device.port, airtune_device.airplay2, airtune_device.audio_supported, airtune_device.encoding as u64, airtune_device.security as u64, airtune_device.transient, airtune_device.sonos_mfi);

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
-> AirplayDevice 
{
    let mut airplay2: bool = false;
    let mut features : u64 = 0;
    let mut status : u64 = 0;
    let mut sonos_flag : bool = false;
    let mut encryption_type : Vec<&str> = Vec::new();
    let mut audio_formats : Vec<&str> = Vec::new();
    let mut model = "";
    let mut sv = "";
    let mut pw = "";
    if device.get_fullname().contains("_airplay._tcp") {
        airplay2 = true;
    }
    for property in device.get_properties().iter(){
      //  println!("{}: {}", property.key(), property.val_str());

        if property.key() == "features" || property.key() == "ft" {
            // Split the features key into 2 hex strings (if there is a comma)
            // println!("Feature string : {}", property.val_str());
            let features_strings: Vec<&str> = property.val_str().split(",").collect();
            if features_strings.len() == 2 {
                let features1 = features_strings[0].trim_start_matches("0x");
                let features2 = features_strings[1].trim_start_matches("0x");
                let mut features_joined = String::from(features2);
                features_joined.push_str(features1);
                features = u64::from_str_radix( &features_joined, 16).unwrap();
                // println!("Feature: {}", features);
                // println!("Video: {}", get_nth_bit(features, Features::Video as u64));
            }
            else {
                features = u64::from_str_radix(property.val_str().trim_start_matches("0x"), 16).unwrap();
                // println!("Feature: {}", features);
            }            
        }

        if property.key() == "flags" || property.key() == "sf" {
            status = u64::from_str_radix(property.val_str().trim_start_matches("0x"), 16).unwrap();
        }

        if property.key() == "et" {
            encryption_type = property.val_str().split(",").collect();
        }

        if property.key() == "cn" {
            audio_formats = property.val_str().split(",").collect();
        }

        if property.key() == "manufacturer" {
            sonos_flag = property.val_str().contains("Sonos");
        } 

        if property.key() == "am" {
            model = property.val_str();
        } 

        if property.key() == "sv" {
            sv = property.val_str();
        } 

        if property.key() == "pw" {
            pw = property.val_str();
        } 
  
    }

    let mut codec = match get_nth_bit(features, Features::AudioFormat1 as u64) {
        0 => AudioEncoding::PCM,
        _ => AudioEncoding::Alac
    };

    if audio_formats.len() > 0 {
        if audio_formats.contains(&"0") {
            codec = AudioEncoding::PCM;
        }
    }

    if model.contains("AppleTV3,1") || model.contains("AirReceiver3,1") || model.contains("AirRecever3,1") || model.contains("Shairport") || sv.contains("false") {
        codec = AudioEncoding::Alac;
    }
      
    if model.contains("Shairport") {
        // shairport sync doesn't support airplay 2 via NTP
        airplay2 = false
    }

    if encryption_type.contains(&"4") {
        sonos_flag = true;
    }


    let mut audio_supported = true;
    if features > 0 {
        audio_supported = match get_nth_bit(features, Features::Audio as u64) {
            0 => false,
            _ => true
        };
    }

    let tranisient_supported = match get_nth_bit(features, Features::SupportsTransientPairing as u64) {
        0 => false,
        _ => true
    };

    let mut need_password = match get_nth_bit(status, Status::PasswordRequired as u64) {
        0 => false,
        _ => true
    };

    need_password = need_password || pw.contains("true");

    let need_pin = match get_nth_bit(status, Status::PinRequired as u64) {
        0 => false,
        _ => true
    };

    let one_time_pairing = match get_nth_bit(status, Status::OneTimePairingRequired as u64) {
        0 => false,
        _ => true
    };

    let device_security: AirPlaySecurity = match (need_password, need_pin, one_time_pairing) {
        (true, false, false) => AirPlaySecurity::Password,
        (false, true, false) => AirPlaySecurity::Pin,
        (false, false, true) => AirPlaySecurity::Pin,
        (false, true, true) => AirPlaySecurity::Pin,
        _ => AirPlaySecurity::None
    };

    let mut device_name = String::from(device.get_fullname());
    device_name = device_name.replace("._raop._tcp.local.", "").replace("._airplay._tcp.local.", "");

    // remove all text before @ in device_name (if it exists)
    let at_index = device_name.find("@");
    if at_index.is_some() {
        device_name = device_name.split_at(at_index.unwrap() + 1).1.to_string();
    }

    AirplayDevice {
        active: true,
        audio_supported: audio_supported,
        name: String::from(device_name),
        host: *device.get_addresses().iter().next().unwrap(),
        port: device.get_port(),
        airplay2: airplay2,
        encoding: codec,
        security: device_security,
        transient: tranisient_supported,
        sonos_mfi: sonos_flag
    }

}

fn get_n_from_shift(shift_value: u64) -> u32 {
    (shift_value as f64).log2() as u32
}


/* Get the nth bit of a number */
fn get_nth_bit(number: u64, mask: u64) -> u64 {
    let n = get_n_from_shift(mask);
    (number & mask) >> n
}

