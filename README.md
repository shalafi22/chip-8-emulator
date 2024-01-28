# Chip-8 Emulator
**Work In Progress!!**
## What is this?

This is a small practice project that aims to emulate a Chip-8 device. For more info on Chip-8: https://en.wikipedia.org/wiki/CHIP-8

## How to use?

Although it is still under construction, the emulator can be used:\
    1. I am not sharing the executable yet, you have to have rustc installed on your device.\
    2. Download this repo and compile it with rustc\
    3. At the time of writing, the .ch8 filename is accepted as a command line argument\
    4. Get a .ch8 binary under a /roms directory in the root path to get it working

### Debug Mode

If you want to execute the ROM in debug mode, you need to set an environment variable. 
Setting CH8_DEBUG=1 enables debug mode and any other value dsiables it.\
In debug mode you can press the right arrow key to execute the next instruction, press M to view the current memory state, 
Y to view current register state and K to print the current instruction about to be executed.

### Example execution
Normal mode:
```
    cargo run -- maze.ch8

```

Debug mode:
```
    $Env:CH8_DEBUG=1
    cargo run -- maze.ch8
```

## Requirements:

Appropriate SDL2 libraries must be installed for the SDL2 dependency to work correctly.

A set of demo .ch8 binaries can be found here: https://github.com/kripod/chip8-roms 