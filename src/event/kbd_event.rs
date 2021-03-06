use std::{
    collections::HashMap,
    debug_assert,
    sync::{Arc, RwLock},
    thread,
};

use super::kbd_event_handler::{self, KbdEventHandler,};
pub use super::kbd_event_handler::{
    KeyFlags,
    KBD_RELEASED,
    KBD_PRESSED,
    KBD_REPEATED,
    KBD_SHIFT,
    KBD_CTRL,
    KBD_ALT,
    KBD_ANY,
};

pub type KbdEvent = Event;

pub struct Event {
    controller: Arc<RwLock<kbd_event_handler::Controller>>,
    kbd_event_handler_child: Option<thread::JoinHandle<()>>,
}

/* No support for held keys at the moment. That would require an array
* containing the state of each key, for now use pressed or released and
* manage the state youself.
*/
impl KbdEvent {
    pub fn new() -> KbdEvent {
        let controller = kbd_event_handler::Controller {
            actions: HashMap::new(),
            done: false,
        };
        Event {
            controller: Arc::new(RwLock::new(controller)),
            kbd_event_handler_child: None,
        }
    }

    pub fn add_event<F: 'static>(&mut self, key: u16, f: F, flags: KeyFlags)
    where
        F: Fn() + Sync + Send,
    {
        self.controller
            .write()
            .unwrap()
            .actions
            .insert(key, (Box::new(f), flags));
    }

    pub fn alter_event(&mut self, key: u16, flags: KeyFlags) {
        debug_assert!(flags == KBD_PRESSED || flags == KBD_REPEATED 
                      || flags == KBD_RELEASED); 
        self.controller
            .write()
            .unwrap()
            .actions
            .get_mut(&key)
            .map(|f| f.1 = flags);

    }
    
    pub fn remove_event(&mut self, key: u16) {
        self.controller
            .write()
            .unwrap()
            .actions
            .remove(&key);
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        let child_controller = Arc::clone(&self.controller);
        let kbd_event_handler = KbdEventHandler::new(child_controller)?;

        let handle = thread::spawn(move || kbd_event_handler.start());
        self.kbd_event_handler_child = Some(handle);
        Ok(())
    }

    pub fn stop(&mut self) {
        //debug_assert!(self.kbd_event_handler_child.map_or(false, |_| true));
        self.controller.write().unwrap().done = true;
        self.kbd_event_handler_child
            .take()
            .map(|handle| handle.join().unwrap());
    }
}
