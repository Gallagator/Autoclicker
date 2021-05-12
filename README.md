# Autoclicker
Rust_Autoclicker for linux
The autoclicker clicks at a normally distributed period. 
This has been done to trick servers in to thinking that the clicks are by humans.

The clicker initially spawns a thread which listens for keyboard input using blocking IO.
When this thread notices a certain key, it will run a thread using a threadpool.

To start toggle the autoclicker, press the 'r' key.
