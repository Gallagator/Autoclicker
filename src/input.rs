
pub mod kbd_input {
    use std::fs::File;
    use std::io::Read;
    use std::mem;
    use libc::input_event;
    pub use input_event_codes::*;

    const KEY_RELEASED: i32 = 0;
    const KEY_PRESSED : i32 = 1;
    const KEY_REPEATED: i32 = 2;

    pub struct kbd_in { 
        key_press : input_event,
        kbd_file  : File,
    }
/* No support for held keys at the moment. That would require an array 
 * containing the state of each key, for now use pressed or released and
 * manage the state youself.
 */
    impl kbd_in {
        pub fn new() -> std::io::Result<kbd_in>{
            Ok (
                kbd_in {
                    key_press : unsafe { mem::zeroed() },
                    kbd_file : File::open("/dev/input/event0")?, 
                }
            )
        }
        pub fn poll(&mut self) -> std::io::Result<()>{
            let mut buf : [u8; mem::size_of::<input_event>()] = 
                unsafe { mem::zeroed() };
            self.kbd_file.read(&mut buf)?;
            self.key_press = unsafe { mem::transmute(buf) };
            Ok(())
        }
        pub fn pressed(&self, key : u16) -> bool {
           return self.key_press.type_ == EV_KEY
                  && self.key_press.code == key 
                  && self.key_press.value == KEY_PRESSED
        }
        pub fn released(&self, key : u16) -> bool {
           return self.key_press.type_ == EV_KEY
                  && self.key_press.code == key 
                  && self.key_press.value == KEY_RELEASED;
        }
        pub fn repeated(&self, key : u16) -> bool {
           return self.key_press.type_ == EV_KEY
                  && self.key_press.code == key 
                  && self.key_press.value == KEY_REPEATED;
        }
    }
}

