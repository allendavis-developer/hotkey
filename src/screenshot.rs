use scap::{capturer::*, frame::*};
use std::sync::mspc;
use tokio::task;


pub struct ScreenshotManager {
    // We'll store screenshot capabilites here
}

impl ScreenshotManager {
    pub fn new() -> Result<Self, Box<std::error::Error>> {
        // We need to check if the platform is supported
        if !scap::is_supported() {
            return Err("Platform is not supported for screenshots".into());
        }

        // Check permissions
        if !scap::has_permission() {
            println!("Requesting Permission..");
            if !scap::request_permission() {
                return Err("Screenshot permission declined".into());
            }
        }

        Ok(Self {})
    }
} 