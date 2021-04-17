use std::{collections::HashMap, sync::Mutex};

mod kbd_event_handler;
use kbd_event_handler::KbdEventHandler;

pub type KbdEvent = Event;

struct Event { 
    controller: Mutex<kbd_event_handler::Controller>;
}

/* No support for held keys at the moment. That would require an array 
* containing the state of each key, for now use pressed or released and
* manage the state youself.
*/
impl KbdEvent {
    pub fn new() -> KbdEvent{
        Event {
            Mutex::new(
                kbd_event_handler::Controller {
                    actions: Mutex::new(Hasmap::new()),
                    done: false,
                })
        }
    }

    pub fn add_event<F: 'static>(&mut self, key: u16, f: F) where F: FnOnce(){
        self.controller
            .lock()
            .unwrap()
            .actions
            .insert(key, Box::new(f));
    }


}

