use std::{
    collections::HashMap,
    debug_assert, result,
    sync::{Arc, Mutex},
    thread,
    cell::RefCell,
};

use super::kbd_event_handler::{self, KbdEventHandler,};
pub use super::kbd_event_handler::{
    KeyFlags,
    KEY_RELEASED,
    KEY_PRESSED,
    KEY_REPEATED,
};

pub type KbdEvent = Event;

pub struct Event {
    controller: Arc<Mutex<kbd_event_handler::Controller>>,
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
            controller: Arc::new(Mutex::new(controller)),
            kbd_event_handler_child: None,
        }
    }

    pub fn add_event<F: 'static>(&mut self, key: u16, f: F, flags: KeyFlags)
    where
        F: Fn() + Sync + Send,
    {
        self.controller
            .lock()
            .unwrap()
            .actions
            .insert(key, (Box::new(f), RefCell::new(flags)));
    }

    pub fn alter_event(&mut self, key: u16, flags: KeyFlags) {
        debug_assert!(flags == KEY_PRESSED || flags == KEY_REPEATED 
                      || flags == KEY_RELEASED); 
        self.controller
            .lock()
            .unwrap()
            .actions
            .get(&key)
            .map(|f| *f.1.borrow_mut() = flags);

    }
    
    pub fn remove_event(&mut self, key: u16) {
        self.controller
            .lock()
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
        self.controller.lock().unwrap().done = true;
        self.kbd_event_handler_child
            .take()
            .map(|handle| handle.join().unwrap());
    }
}
