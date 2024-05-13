// Gurleen Dhaliwal
// Drop - Embedded Rust - Winter 2023

#![no_main]
#![no_std]

// Importing crates and modules 
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use critical_section_lock_mut::LockMut;
use lsm303agr::{interface::I2cInterface, mode::MagOneShot, Lsm303agr};
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{delay::Delay, gpio::{p0::P0_00, Level, Output, PushPull},
        pac::{self, interrupt, TIMER1, TWIM0}, prelude::*, twim, Timer, Twim,
    },
    pac::{twim0::frequency::FREQUENCY_A, TIMER2},
};

// Global display lock for thread-safe access to the LED matrix.
static DISPLAY: LockMut<Display<TIMER1>> = LockMut::new();

// Interrupt handler for TIMER1, used for non-blocking display updates.
#[interrupt]
fn TIMER1() {
    DISPLAY.with_lock(|display| display.handle_display_event());
}

// Function to determine if the MicroBit is in a free-falling state.
fn falling(sensor: &mut Lsm303agr<I2cInterface<Twim<TWIM0>>, MagOneShot>) -> bool {
    // Check if new XYZ data is available from the accelerometer.
    if sensor.accel_status().unwrap().xyz_new_data() {
        // Retrieve acceleration data and calculate the total acceleration.
        let accel_data = sensor.acceleration().unwrap();
        let accel_x = (accel_data.x_mg() / 1000) as f32;
        let accel_y = (accel_data.y_mg() / 1000) as f32;
        let accel_z = (accel_data.z_mg() / 1000) as f32;
        let total_accel = accel_x * accel_x + accel_y * accel_y + accel_z * accel_z; // Manual squaring
        rprintln!("{}", total_accel);
         // Determine falling if total acceleration is less than a threshold.
        return 0.25 < total_accel; 
    }
    false
}

// For emittin a 1kHz yell tone using the speaker for a specified duration.
fn emit_tone(sound: &mut P0_00<Output<PushPull>>, timing: &mut Delay) {
    let duration: u16 = 100; 
    for _ in 0..duration {
        sound.set_high().unwrap();
        timing.delay_us(500u16);
        sound.set_low().unwrap();
        timing.delay_us(500u16);
    }
}

// Function to display a single dot in the center of the LED screen.
fn display_dot(image: &mut [[u8; 5]; 5]) {
    // Reset the image to all zeros first
    *image = [[0; 5]; 5];
    // Set the center dot to the brightest level (9)
    image[2][2] = 9;
    
    // Create a GreyscaleImage and display it
    let led_display = GreyscaleImage::new(image);
    DISPLAY.with_lock(|display| display.show(&led_display));
}

// Function to display an exclamation mark on the LED screen when falling.
fn display_exclamation(pattern: &mut [[u8; 5]; 5], timer: &mut Timer<TIMER2>) {
    // Clear the pattern matrix to all zeros
    *pattern = [[0; 5]; 5];
    // Assign high intensity to specific LEDs to form an exclamation mark
    pattern[0][2] = 9; // Top of exclamation
    pattern[1][2] = 9; // Middle of exclamation
    pattern[2][2] = 9; // Bottom of exclamation upper part
    pattern[4][2] = 9; // Dot of exclamation
    
    // Convert to greyscale and display
    let greyscale_image = GreyscaleImage::new(pattern);
    DISPLAY.with_lock(|disp| disp.show(&greyscale_image));
    // Keep the display for 1 second
    timer.delay_ms(1000u32);
}


// Main function: initializes hardware, enters loop to check for falling and react.
#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let display = Display::new(board.TIMER1, board.display_pins);
    DISPLAY.init(display);

    // NVIC for interrupts
    unsafe {
        board.NVIC.set_priority(pac::Interrupt::TIMER1, 128);
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }

    // Initialize speaker, delay, clock, I2C interface, and LSM303AGR accelerometer
    let mut clock = Timer::new(board.TIMER2);
    let mut delay = Delay::new(board.SYST);
    let mut tone = board.speaker_pin.into_push_pull_output(Level::Low);
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let mut lsm303 = Lsm303agr::new_with_i2c(i2c);
    lsm303.init().unwrap();
    lsm303.set_accel_mode_and_odr(&mut delay, lsm303agr::AccelMode::Normal, lsm303agr::AccelOutputDataRate::Hz50).unwrap();

    let mut pattern = [[0; 5]; 5];

    // Main loop
    loop {
        display_dot(&mut pattern);
        if falling(&mut lsm303) {
            display_exclamation(&mut pattern, &mut clock);
            emit_tone(&mut tone, &mut delay);
            clock.delay_ms(1000u32);
        }
    }
}


