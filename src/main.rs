#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use embedded_hal::{digital::InputPin, delay::DelayNs};
use microbit::{
    board::{Board},
    display::blocking::Display,
    hal::{twim, timer::Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

const FRAME_LENGTH_MS: u32 = 200;
const RANGE_SIZE_COARSE: i32 = 1000;
const RANGE_SIZE_FINE: i32 = 100;

fn render_bubble(bubble: &mut [[u8; 5]; 5], x: i32, y: i32, total_range: i32) {
    bubble.fill([0u8; 5]);

    let range_offset = total_range / 2;
    let interval_size = total_range / 5;
    let mut x_coord = (x+range_offset) / interval_size;
    let mut y_coord = (y+range_offset) / interval_size;

    if x_coord < 0 {
        x_coord = 0;
    } else if x_coord > 4 {
        x_coord = 4;
    }
    x_coord = 4 - x_coord; // we have to reverse the axes for the x reading

    if y_coord < 0 {
        y_coord = 0;
    } else if y_coord > 4 {
        y_coord = 4;
    }

    bubble[y_coord as usize][x_coord as usize] = 1;
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_mode_and_odr(
        &mut timer,
        AccelMode::HighResolution,
        AccelOutputDataRate::Hz50,
    ).unwrap();

    let mut bubbler = [[0u8; 5]; 5];
    let mut range_size = RANGE_SIZE_COARSE;

    loop {
        if button_a.is_low().unwrap() { range_size = RANGE_SIZE_COARSE };
        if button_b.is_low().unwrap() { range_size = RANGE_SIZE_FINE };

        let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
        if z > 0 {
            rprintln!("board is upside down");
            timer.delay_ms(FRAME_LENGTH_MS);
        } else {
            rprintln!("Acceleration: ({}, {})", x, y);
            render_bubble(&mut bubbler, x, y, range_size);
            display.show(&mut timer, bubbler, FRAME_LENGTH_MS);
        }
    }
}


