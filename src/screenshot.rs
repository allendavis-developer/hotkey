use fs_extra::dir;
use xcap::Monitor;


pub fn take_screenshot() -> Result<(), Box<dyn std::error::Error>> {
    // Create the directory for screenshots if it doesn't exist
    dir::create_all("screenshots", true).unwrap();
        
    // Get a list of all connected monitors
    let monitors = Monitor::all().unwrap();
        
    // Loop through each monitor and capture it
    for monitor in monitors {
        let image = monitor.capture_image().unwrap();
        
        // Create a filename based on the monitor's name
        let filename = format!("screenshots/monitor-{}.png", monitor.name().unwrap());
        
        // Save the image to the file
        image.save(&filename).unwrap();
    }
        
    Ok(())
}

