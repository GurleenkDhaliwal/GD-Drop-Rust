# Drop

## Submitted by: Gurleen Dhaliwal

## Overview
"Drop" is a Rust-based embedded project using the MB2 that detects free-fall motion and responds with visual and audio alerts. And it also uses the LSM303AGR accelerometer, it demonstrates real-time data processing on embedded devices.

## Implementation Details
- **Free-Fall Detection:** Leverages the accelerometer to monitor real-time acceleration and detect free-fall conditions.
- **Visual Alert:** Displays an exclamation mark on the MicroBit's LED matrix during free-fall.
- **Audio Alert:** Emits a 1KHz tone using the MicroBit's speaker to signal free-fall.

I used Github at https://github.com/pdx-cs-rust-embedded/hello-audio as refernce for the audio portion. And https://github.com/pdx-cs-rust-embedded/mb2-grayscale for non blocking display.

## Observations
Working on "Drop" was challenging, the display functions were not as difficult. Initially, interfacing with the accelerometer presented challenges, but I ended up significantly improving my understanding of Rust and embedded system dynamics. Also figuring out how to incorporate the timer for controlling the duration of sound, so that its not one continous sound.  

## How to Run
1. Ensure Rust and Cargo are installed.
2. Clone the project repository and navigate to the project directory.
3. Build the project with `cargo build`.
