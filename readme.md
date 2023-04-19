# DRIVE 74HC595

library for using IC 74HC595 Shift Register with embedded_hal


### how to use

first pass the correct pins to struct
```rust
let mut drive = ShiftRegister::new(OE, SER, SRCLR, SRCLK, RCLK);
```
OE and SRCLR can be dummy pins (if this is the case connect the pin of IC OE and SRCLR to GND and VCC respectively)

after that call the "begin" function
```rust
drive.begin();
```

now the IC is ready to be used. see more functions in the list below

### functions

`begin` starts the 74HC595

`load` loads an 8bit value to the 74HC595

`enable_output` enable output

`disable_output` disable the output (operations can still be performed)

`output_clear` clean the output

`shift_zero` shifts zero once

`shift_one` shift one once

`shift_zero_times` shifts zero N times

`shift_one_times` shift one N times