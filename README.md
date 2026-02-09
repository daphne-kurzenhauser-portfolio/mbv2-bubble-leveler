# micro:bit v2 Bubble Leveler

## Description

This is a simple implementation of a bubble leveler on the micro:bit v2. It uses
the on-board IMU to read acceleration from gravitational orientation and renders
a corresponding "bubble" on the micro:bit's LED display. The program rules are as follows:

- The program runs at 5 frames per second (reading the IMU every 200 milliseconds)
- The program starts in "coarse" mode, determining the bubble position by splitting
  the acceleration range from -500 mG to 500 mG into five intervals of 200 mG each.
- There is an additional "fine" mode, which determines bubble position using a range
  of -50 mG to 50 mG.
- When the board is upside-down (i.e. the z-acceleration is positive), no bubble
  is present.
- Pressing the B button on the board will set the bubbler to "fine" mode; pressing
  the A button will set the bubbler to "coarse" mode.

## Build and run

- To build the application without flashing: `cargo build --release` from repo root
- To build the application and flash to the micro::bit: `cargo embed --release` from repo root

## Implementation details

The IMU is read through the micro:bit's I2C bus, which is instantiated by using
the twim module in the microbit's HAL and the dedicated driver crate for the 
LSM303AGR accelerometer. The actual implementation is quite simple--the `render_bubble`
function takes in an `x` and `y` acceleration value and determines the bubble
position linearly. `render_bubble` also takes in a value for the range limit of mG
values, allowing for easy extension and switching between coarse/fine modes.

At the start of each frame, the buttons are checked and the range size parameter
is updated (if relevant), the bubble position is calculated via `render_bubble`,
and then the display is updated for 200 ms.
