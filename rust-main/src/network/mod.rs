use mdns_sd::{ServiceDaemon, ServiceEvent};

pub fn start_scan() {
    let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
    // let receiver = mdns.browse("_services._dns-sd._udp.local.").expect("Failed to browse");
    // let service_type = "_airplay._tcp.local."; 
    // let receiver = mdns.browse(service_type).expect("Failed to browse");
    let services = vec![
        "_airplay._tcp.local.",
        "_googlecast._tcp.local.",
        "_ipp._tcp.local.",
        "_hap._tcp.local.",
        "_spotify-connect._tcp.local.",
        "_smb._tcp.local.",
        "_axis-video._tcp.local.",
        "_print._tcp.local.",
        "_http._tcp.local.",
        "_ssdp._udp.local.",
        "_apple-mobdev2._tcp.local.",
        "_remotetv._tcp.local.",
    ];
    let receiver = mdns.browse("_airplay._tcp.local.").expect("Failed to browse");
    for service in services {
        mdns.browse(service).expect("Failed to browse");
    }

    println!("Scanning for devices... (Press Ctrl+C to stop)");

    while let Ok(event) = receiver.recv() {
        if let ServiceEvent::ServiceResolved(info) = event {
            println!("-------------------------------------------");
            println!("NAME:     {}", info.get_fullname());
            println!("HOSTNAME: {}", info.get_hostname());
            println!("IPs:      {:?}", info.get_addresses());
            
            // FIX: Access properties directly without .as_ref()
            let props = info.get_properties();
            
            // FIX: Use explicit type hints for 'get' to solve type inference
            let model = props.get("model");
            let apple_model = props.get("am");

            // Combine them safely
            if let Some(m) = model.or(apple_model) {
                println!("MODEL:    {}", m);
            }
        }
    }
}
