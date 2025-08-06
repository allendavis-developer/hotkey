use fs_extra::dir;
use xcap::Monitor;


pub fn take_screenshot() -> Result<(), Box<dyn std::error::Error>> {
    dir::create_all("screenshots", true).unwrap();
        
    let monitors = Monitor::all().unwrap();
        
    for monitor in monitors {
        let image = monitor.capture_image().unwrap();
        let filename = format!("screenshots/monitor-{}.png", monitor.name().unwrap());
        image.save(&filename).unwrap();
    }
        
    Ok(())
}

