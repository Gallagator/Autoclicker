use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    mem,
    sync::{Arc, RwLock},
    thread,
};

use libc::input_event;

pub use input_event_codes::*;

const KEY_RELEASED: i32 = 0;
const KEY_PRESSED: i32 = 1;
const KEY_REPEATED: i32 = 2;

#[macro_export]
macro_rules! BIT {
    ($bit:expr) => {
        (1 << $bit)
    };
}

pub type KeyFlags = u8;
pub const KBD_RELEASED: KeyFlags = BIT!(0);
pub const KBD_PRESSED : KeyFlags = BIT!(1);
pub const KBD_REPEATED: KeyFlags = BIT!(2);
pub const KBD_SHIFT   : KeyFlags = BIT!(3);
pub const KBD_CTRL    : KeyFlags = BIT!(4);
pub const KBD_ALT     : KeyFlags = BIT!(5);
pub const KBD_ANY     : KeyFlags = BIT!(6);


pub type KbdEventHandler = Handler;

pub struct Handler {
    key_press: input_event,
    kbd_file: File, /* TODO Find this file manually */
    /* Arc-Mutex needed to share controller between threads */
    controller: Arc<RwLock<Controller>>, 
    handles: Vec<thread::JoinHandle<Option<()>>>, /* Handles on the Child threads */
    shift_presses: u8,
    ctrl_presses: u8,
    alt_presses: u8,
}

/* Must be boxed since the trait's size in unknown */
type Task = Box<dyn Fn() + Sync + Send>;
type TaskMap = HashMap<u16, (Task, KeyFlags)>;

pub struct Controller {
    pub actions: TaskMap,
    pub done: bool,
}

impl KbdEventHandler {
    pub fn new(controller: Arc<RwLock<Controller>>) -> std::io::Result<KbdEventHandler> {
        Ok(Handler {
            key_press: unsafe { mem::zeroed() },
            kbd_file: File::open("/dev/input/event0")?,
            controller,
            handles: Vec::new(),
            shift_presses: 0,
            ctrl_presses: 0,
            alt_presses: 0,
    })
    }
    /* This function is to run in another thread so it takes full ownership of
     * self */
    pub fn start(mut self) {
        while !self.controller.read().unwrap().done {
            self.poll().unwrap(); /* polling should never fail. */
            let key_code = self.key_press.code;
            let key_value = self.key_press.value;  
            /* Continues if the key_press is not in the map. Stops creation 
             * of a needless thread. */
            if self.key_press.type_ != EV_KEY {continue;}
            if let None = &self.controller //TODO check the kind of key_press
                               .read()
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
                    .read()
                    .unwrap()
                    .actions
                    .get(&key_code)
                    .map(|f| 
                         if (key_val_to_flag(key_value) & f.1) > 0 { 
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
}

fn key_val_to_flag(val: i32) -> u8 {
    match val {
        KEY_RELEASED => KBD_RELEASED,
        KEY_PRESSED  => KBD_PRESSED,
        KEY_REPEATED => KBD_REPEATED,
        _ => 0,
    }
}
