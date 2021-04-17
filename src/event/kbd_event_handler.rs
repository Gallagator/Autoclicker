use std::{fs::File, io::Read, mem, collections::HashMap, sync::Mutex};
use libc::input_event;

pub use input_event_codes::*;

const KEY_RELEASED: i32 = 0;
const KEY_PRESSED : i32 = 1;
const KEY_REPEATED: i32 = 2;

pub type KbdEventHandler = Handler;

struct Handler {
    key_press : input_event,
    kbd_file  : File,
    controller: Mutex<Controller>,
}

pub struct Controller {
    actions: HashMap<u16, Box<dyn FnOnce()>>,
    done   : bool,
}

impl KbdEventHandler {

    pub fn new(controller: Mutex<Controller>) 
        -> std::io::Result<KbdEventHandler>{
        Ok (
            Handler {
                key_press : unsafe { mem::zeroed() },
                kbd_file  : File::open("/dev/input/event0")?, 
                controller: controller,
            }
        )
    }
    fn poll(&mut self) -> std::io::Result<()>{
        let mut buf : [u8; mem::size_of::<input_event>()] = 
            unsafe { mem::zeroed() };
        self.kbd_file.read(&mut buf)?;
        self.key_press = unsafe { mem::transmute(buf) };
        Ok(())
    }
    fn pressed(&self, key : u16) -> bool {
       return self.key_press.type_ == EV_KEY
              && self.key_press.code == key 
              && self.key_press.value == KEY_PRESSED
    }
    fn released(&self, key : u16) -> bool {
       return self.key_press.type_ == EV_KEY
              && self.key_press.code == key 
              && self.key_press.value == KEY_RELEASED;
    }
    fn repeated(&self, key : u16) -> bool {
       return self.key_press.type_ == EV_KEY
              && self.key_press.code == key 
              && self.key_press.value == KEY_REPEATED;
    }
}
