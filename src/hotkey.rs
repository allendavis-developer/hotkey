use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}};
use global_hotkey::GlobalHotKeyEvent;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum HotkeyMessage {
    Screenshot,   
    ShowHide,    
}

pub fn setup_hotkeys() -> Option<mpsc::Receiver<HotkeyMessage>> {
    // Create manager on main thread
    let manager = GlobalHotKeyManager::new().ok()?;
    
    // Register hotkey
    let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);
    manager.register(hotkey).ok()?;
    println!("Successfully registered hotkey!");

    // Create channel
    let (sender, receiver) = mpsc::channel();
    
    // Spawn thread only for event processing
    thread::spawn(move || {
        loop {
            if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                println!("Hotkey pressed: {:?}", event);
                
                // Match hotkey ID and send appropriate message
                // You'll need to store the hotkey ID when registering
                sender.send(HotkeyMessage::ShowHide).unwrap();
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Return the receiver to main thread
    Some(receiver)
}
