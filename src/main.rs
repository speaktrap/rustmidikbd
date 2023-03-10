#![no_std]
#![no_main]

//use arduino_hal::prelude::*;
use panic_halt as _;
use embedded_hal::serial::Write;
//use embedded_hal::digital::v2::InputPin;

const NOTE_ON: u8 = 144;
const NOTE_OFF: u8 = 128;
const NOTE_C: u8 = 24;
const SHIFT_KEYCODE: usize = 17;

/*fn get_pullup_pin<P: InputPin>(pin: &P) -> Pin<Input<PullUp>, PIN> {
    pin.into_pull_up_input();
	}*/

/*fn is_pin_high<P: InputPin>(pin: &P) -> bool {
    pin.is_high().unwrap_or(false)
	}*/

#[arduino_hal::entry]
fn main() -> ! {
	let dp = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(dp);
	let mut serial = arduino_hal::default_serial!(dp, pins, 31250);
	
	// Set all pins except RX/TX as input pull-up
	let key_c_1  = pins.d13.into_pull_up_input();
	let key_cs_1 = pins.d12.into_pull_up_input();
	let key_d_1  = pins.d11.into_pull_up_input();
	let key_ds_1 = pins.d10.into_pull_up_input();
	let key_e_1  = pins.d9.into_pull_up_input();
	let key_f_1  = pins.d8.into_pull_up_input();
	let key_fs_1 = pins.d7.into_pull_up_input();
	let key_g_1  = pins.d6.into_pull_up_input();
	let key_gs_1 = pins.d5.into_pull_up_input();
	let key_a_1  = pins.d4.into_pull_up_input();
	let key_as_1 = pins.d3.into_pull_up_input();
	let key_b_1  = pins.d2.into_pull_up_input();
	let key_c_2  = pins.a0.into_pull_up_input();
	let key_cs_2 = pins.a1.into_pull_up_input();
	let key_d_2  = pins.a2.into_pull_up_input();
	let key_ds_2 = pins.a3.into_pull_up_input();
	let key_e_2  = pins.a4.into_pull_up_input();
	let key_sh   = pins.a5.into_pull_up_input();

	let mut pressed_then = [false; 18];
	let mut transposition = 24;
	let mut channel = 0;
	let mut shift_on = false;

	loop {

		let pressed_now: [bool; 18] = [
			key_c_1.is_high(), //0
			key_cs_1.is_high(),//1
			key_d_1.is_high(), //2
			key_ds_1.is_high(),//3
			key_e_1.is_high(), //4
			key_f_1.is_high(), //5
			key_fs_1.is_high(),//6
			key_g_1.is_high(), //7
			key_gs_1.is_high(),//8
			key_a_1.is_high(), //9
			key_as_1.is_high(),//10
			key_b_1.is_high(), //11
			key_c_2.is_high(), //12
			key_cs_2.is_high(),//13
			key_d_2.is_high(), //14
			key_ds_2.is_high(),//15
			key_e_2.is_high(), //16
			key_sh.is_high()]; //17
			
		if shift_on {
			for i in 0..SHIFT_KEYCODE {
				if pressed_now[i] {
					match i {
						0 => transposition = 0,
						1 => channel = 0,
						2 => transposition = 12,
						3 => channel = 1,
						4 => transposition = 24,
						5 => transposition = 36,
						6 => channel = 2,
						7 => transposition = 48,
						8 => channel = 3,
						9 => transposition = 60,
						10 => channel = 4,
						11 => transposition = 72,
						12 => transposition = 84,
						13 => channel = 5,
						14 => transposition = 96,
						15 => channel = 6,
						16 => channel = 7,
						_ => (),
						};
						shift_on = false;
					}
				}
			}
		
		else {
			for i in 0..SHIFT_KEYCODE {
				if !pressed_then[i] && pressed_now[i] {
					//message = [NOTE_ON, NOTE_C + i as u8 + transposition, 127];
					nb::block!(serial.write(NOTE_ON + channel)).unwrap();
					nb::block!(serial.write(NOTE_C + i as u8 + transposition)).unwrap();
					nb::block!(serial.write(127)).unwrap();
					pressed_then[i] = pressed_now[i];
					}
				if pressed_then[i] && !pressed_now[i] {
					nb::block!(serial.write(NOTE_OFF + channel)).unwrap();
					nb::block!(serial.write(NOTE_C + i as u8 + transposition)).unwrap();
					nb::block!(serial.write(127)).unwrap();
					pressed_then[i] = pressed_now[i];
					}
				
				if pressed_now[SHIFT_KEYCODE] {
					shift_on = true;
					for y in 0..SHIFT_KEYCODE { pressed_then[y] = false; }
					}
				}
			
			}
		}
	}


/*for byte in &message {	 		
				nb::block!(serial.write(*byte)).unwrap();
				}*/
