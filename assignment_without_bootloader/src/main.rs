#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

// Panic handler for `no_std`
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// VGA text mode framebuffer address and dimensions
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const TAB_SIZE: usize = 8;


const DEFAULT_COLOR: u8 = 0x0F; 
const RED_COLOR: u8 = 0x0C; 
const GREEN_COLOR: u8 = 0x0A; 
const BLUE_COLOR: u8 = 0x09; 


unsafe fn write_char(row: usize, col: usize, c: u8, color: u8) {
    let index = (row * VGA_WIDTH + col) * 2; 
    *VGA_BUFFER.offset(index as isize) = c;
    *VGA_BUFFER.offset((index as isize) + 1) = color;
}


static mut CURSOR_ROW: usize = 0;
static mut CURSOR_COL: usize = 0;
static mut CURRENT_COLOR: u8 = DEFAULT_COLOR;

 fn print(s: &str) {
    unsafe {
        let mut chars = s.bytes();
        while let Some(byte) = chars.next() {
            match byte {
                b'\n' => {
                    CURSOR_ROW += 1;
                    CURSOR_COL = 0;
                }
                b'\t' => {
                    CURSOR_COL = (CURSOR_COL / TAB_SIZE + 1) * TAB_SIZE;
                }
                b'\\' => {
                    if let Some(b'c') = chars.next() {
                        if let Some(color_code) = chars.next() {
                            match color_code {
                                b'0' => CURRENT_COLOR = RED_COLOR,   
                                b'1' => CURRENT_COLOR = GREEN_COLOR, 
                                b'2' => CURRENT_COLOR = BLUE_COLOR,  
                                b'F' => CURRENT_COLOR = DEFAULT_COLOR, 
                                _ => {
                                    write_char(CURSOR_ROW, CURSOR_COL, b'\\', CURRENT_COLOR);
                                    CURSOR_COL += 1;
                                    write_char(CURSOR_ROW, CURSOR_COL, b'c', CURRENT_COLOR);
                                    CURSOR_COL += 1;
                                    write_char(CURSOR_ROW, CURSOR_COL, color_code, CURRENT_COLOR);
                                    CURSOR_COL += 1;
                                }
                            }
                        }
                    } else {
                        write_char(CURSOR_ROW, CURSOR_COL, b'\\', CURRENT_COLOR);
                        CURSOR_COL += 1;
                    }
                }
                _ => {
                    write_char(CURSOR_ROW, CURSOR_COL, byte, CURRENT_COLOR);
                    CURSOR_COL += 1;

                    // If the cursor reaches the end of the line, wrap to the next row
                    if CURSOR_COL >= VGA_WIDTH {
                        CURSOR_ROW += 1;
                        CURSOR_COL = 0;
                    }
                }
            }

            // Stop if we exceed the screen height
            if CURSOR_ROW >= VGA_HEIGHT {
                break;
            }
        }
    }
}

/// Starts the program and displays formatted text using the `print` function.
#[no_mangle] // Prevent name mangling for `_start`
pub extern "C" fn _start() -> ! {
    print("Hello World!\n");
    print("Welcome to my OS!\n\\c0This is red text.\n\\c1This is green text.\n\\c2This is blue text.\n\\cFThis is default text.\n");
    print("This is a new line.\tAnd this is a tab.\n");
    print("This approach makes it easy to dynamically change text colors by embedding sequences in the string This code was the epitome of my stress and the fact the errors i was getting werent related to the code is what made it way wors but in the end om done and my code works fine i believe if theres a need to debug i dont think i can do that as i have wasted brain power to do so.\n");


    loop {} // Infinite loop to prevent exiting
}
