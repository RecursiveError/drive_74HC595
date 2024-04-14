#![no_std]
#![allow(unused)]

use embedded_hal::digital::OutputPin;

pub struct ShiftRegister<OE, SER, SRCLR, SRCLK, RCLK>
where
    OE: OutputPin,
    SER: OutputPin,
    SRCLR: OutputPin,
    SRCLK: OutputPin,
    RCLK: OutputPin,
{
    oe: OE,
    ser: SER,
    srclr: SRCLR,
    srclk: SRCLK,
    rclk: RCLK,
}

impl<OE, SER, SRCLR, SRCLK, RCLK> ShiftRegister<OE, SER, SRCLR, SRCLK, RCLK>
where
    OE: OutputPin,
    SER: OutputPin,
    SRCLR: OutputPin,
    SRCLK: OutputPin,
    RCLK: OutputPin,
{
    pub fn new(
        oe: OE,
        ser: SER,
        srclr: SRCLR,
        srclk: SRCLK,
        rclk: RCLK,
    ) -> ShiftRegister<OE, SER, SRCLR, SRCLK, RCLK> {
        ShiftRegister {
            oe,
            ser,
            srclr,
            srclk,
            rclk,
        }
    }

    /// starts the 74HC595
    pub fn begin(&mut self) {
        self.oe.set_low();
        self.srclr.set_high();
    }

    /// loads an 8bit value to the 74HC595
    pub fn load(&mut self, mut data: u8) {
        for _ in 0..8 {
            if (data & 0x80) != 0 {
                self.ser.set_high();
            } else {
                self.ser.set_low();
            }
            self.pulse_srclk();
            data <<= 1;
        }
        self.pulse_rclk();
    }

    /// enable output
    #[inline]
    pub fn enable_output(&mut self) {
        self.oe.set_low();
    }

    /// disable the output (operations can still be performed)
    #[inline]
    pub fn disable_output(&mut self) {
        self.oe.set_high();
    }

    /// clean the output
    pub fn output_clear(&mut self) {
        self.srclr.set_low();
        self.pulse_rclk();
        self.srclr.set_high();
    }
    /// shifts zero once
    pub fn shift_zero(&mut self) {
        self.ser.set_low();
        self.pulse_srclk();
        self.pulse_rclk();
    }
    /// shift one once
    pub fn shift_one(&mut self) {
        self.ser.set_high();
        self.pulse_srclk();
        self.pulse_rclk();
    }

    ///shifts zero N times
    pub fn shift_zero_times(&mut self, shift_qtd: u8) {
        for _ in 0..shift_qtd {
            self.shift_zero();
        }
    }

    /// shift one N times
    pub fn shift_one_times(&mut self, shift_qtd: u8) {
        for _ in 0..shift_qtd {
            self.shift_one();
        }
    }

    fn pulse_srclk(&mut self) {
        self.srclk.set_high();
        self.srclk.set_low();
    }

    fn pulse_rclk(&mut self) {
        self.rclk.set_high();
        self.rclk.set_low();
    }
}
