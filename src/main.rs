#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

const MORSE_CODE: [Option<&'static str>; 256] = {
    let mut table = [None; 256]; // A table of 256 elements
    table[b'A' as usize] = Some("01");
    table[b'B' as usize] = Some("1000");
    table[b'C' as usize] = Some("1010");
    table[b'D' as usize] = Some("100");
    table[b'E' as usize] = Some("0");
    table[b'F' as usize] = Some("0010");
    table[b'G' as usize] = Some("110");
    table[b'H' as usize] = Some("0000");
    table[b'I' as usize] = Some("00");
    table[b'J' as usize] = Some("0111");
    table[b'K' as usize] = Some("101");
    table[b'L' as usize] = Some("0100");
    table[b'M' as usize] = Some("11");
    table[b'N' as usize] = Some("10");
    table[b'O' as usize] = Some("111");
    table[b'P' as usize] = Some("0110");
    table[b'Q' as usize] = Some("1101");
    table[b'R' as usize] = Some("010");
    table[b'S' as usize] = Some("000");
    table[b'T' as usize] = Some("1");
    table[b'U' as usize] = Some("001");
    table[b'V' as usize] = Some("0001");
    table[b'W' as usize] = Some("011");
    table[b'X' as usize] = Some("1001");
    table[b'Y' as usize] = Some("1011");
    table[b'Z' as usize] = Some("1100");
    table[b'0' as usize] = Some("11111");
    table[b'1' as usize] = Some("01111");
    table[b'2' as usize] = Some("00111");
    table[b'3' as usize] = Some("00011");
    table[b'4' as usize] = Some("00001");
    table[b'5' as usize] = Some("00000");
    table[b'6' as usize] = Some("10000");
    table[b'7' as usize] = Some("11000");
    table[b'8' as usize] = Some("11100");
    table[b'9' as usize] = Some("11110");
    table[b'.' as usize] = Some("010101");
    table[b',' as usize] = Some("110011");
    table[b'?' as usize] = Some("001100");
    table
};

async fn blink<'a>(led: &mut Output<'a>, c:char, basetime:u64) {
    if c=='0'{
        led.set_high();
        Timer::after_millis(basetime).await;
        led.set_low();
        Timer::after_millis(basetime).await;
    }else if c=='1'{
        led.set_high();
        Timer::after_millis(basetime*3).await;
        led.set_low();
        Timer::after_millis(basetime).await;
    }
}

async fn display_message<'a>(led: &mut Output<'a>, message: &str, basetime:u64) {
    for word in message.split_whitespace(){
        for character in word.chars(){
            if let Some(morse) = MORSE_CODE[character as usize] {
                for signal in morse.chars() {
                    blink(led,signal,basetime).await;
                }
                Timer::after_millis(basetime*3).await;
            }else{
                continue;
            }
        }
        Timer::after_millis(basetime*7).await;
    }
}


#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    let basetime:u64 = 200;
    
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);
    let message = "YYZ";
    let hello_world = "HELLO, WORLD?";
    let me = "HI, IM KEVIN, AND THIS IS MY FIRST EMBEDDED PROJECT.";
    let sos = "SOS";
    loop {
        //blink message:
        display_message(&mut led, message, basetime).await;
        Timer::after_millis(basetime * 14).await;
        
        display_message(&mut led, hello_world, basetime).await;
        Timer::after_millis(basetime * 14).await;
        
        display_message(&mut led, me, basetime).await;
        Timer::after_millis(basetime * 14).await;
        
        display_message(&mut led, sos, basetime).await;
        Timer::after_millis(basetime * 14).await;
    }
}
