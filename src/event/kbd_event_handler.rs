use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    mem,
    sync::{Arc, Mutex},
    cell::RefCell,
    thread,
};

use libc::input_event;

pub use input_event_codes::*;

const KEY_RELEASE: i32 = 0;
const KEY_PRESS: i32 = 1;
const KEY_REPEAT: i32 = 2;

#[macro_export]
macro_rules! BIT {
    ($bit:expr) => {
        (1 << $bit)
    };
}

pub type KeyFlags = u8;
pub const KEY_RELEASED: KeyFlags = BIT!(0);
pub const KEY_PRESSED:  KeyFlags = BIT!(1);
pub const KEY_REPEATED: KeyFlags = BIT!(2);


pub type KbdEventHandler = Handler;

pub struct Handler {
    key_press: input_event,
    kbd_file: File, /* TODO Find this file manually */
    /* Arc-Mutex needed to share controller between threads */
    controller: Arc<Mutex<Controller>>, 
    handles: Vec<thread::JoinHandle<Option<()>>>, /* Handles on the Child threads */
}

/* Must be boxed since the trait's size in unknown */
type Task = Box<dyn Fn() + Sync + Send>;
type TaskMap = HashMap<u16, (Task, RefCell<KeyFlags>)>;

pub struct Controller {
    pub actions: TaskMap,
    pub done: bool,
}

impl KbdEventHandler {
    pub fn new(controller: Arc<Mutex<Controller>>) -> std::io::Result<KbdEventHandler> {
        Ok(Handler {
            key_press: unsafe { mem::zeroed() },
            kbd_file: File::open("/dev/input/event0")?,
            controller,
            handles: Vec::new(),
        })
    }
    /* This function is to run in another thread so it takes full ownership of
     * self */
    pub fn start(mut self) {
        while !self.controller.lock().unwrap().done {
            self.poll().unwrap(); /* polling should never fail. */
            let key_code = self.key_press.code;
            let key_value = self.key_press.value;  
            /* Continues if the key_press is not in the map. Stops creation 
             * of a needless thread. */
            if self.key_press.type_ != EV_KEY {continue;}
            if let None = &self.controller
                               .lock()
                               .unwrap()
                               .actions
                               .get(&key_code) 
                               {continue;}
            /* Clone of controller needed for each child_thread */
            let child_controller = Arc::clone(&self.controller);
            /* Creates the thread. Still has to check map for the closure
             * since another thread may have updated the actions HashMap */
            let handle = thread::spawn(move || {
                child_controller
                    .lock()
                    .unwrap()
                    .actions
                    .get(&key_code)
                    .map(|f| 
                         if (key_val_to_flag(key_value) & *f.1.borrow()) > 0 { 
                             f.0() 
                         })
            });
            self.handles.push(handle);
        }
        /* Must wait for all child threads to finish */
        for handle in self.handles {
            handle.join().unwrap();
        }
    }
    
    /* Reads keyboard device file */
    fn poll(&mut self) -> std::io::Result<()> {
        let mut buf: [u8; mem::size_of::<input_event>()] = unsafe { mem::zeroed() };
        self.kbd_file.read(&mut buf)?;
        self.key_press = unsafe { mem::transmute(buf) };
        Ok(())
    }

   // fn pressed(&self, key: u16) -> bool {
   //     return self.key_press.type_ == EV_KEY
   //         && self.key_press.code == key
   //         && self.key_press.value == KEY_PRESSED;
   // }
   // fn released(&self, key: u16) -> bool {
   //     return self.key_press.type_ == EV_KEY
   //         && self.key_press.code == key
   //         && self.key_press.value == KEY_RELEASED;
   // }
   // fn repeated(&self, key: u16) -> bool {
   //     return self.key_press.type_ == EV_KEY
   //         && self.key_press.code == key
   //         && self.key_press.value == KEY_REPEATED;
   // }
}

fn key_val_to_flag(val: i32) -> u8 {
    match val {
        KEY_RELEASE => KEY_RELEASED,
        KEY_PRESS => KEY_PRESSED,
        KEY_REPEAT => KEY_REPEATED,
        _ => 0,
    }
}
