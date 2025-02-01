use core::fmt::{self, Write};
use bootloader_api::info::FrameBufferInfo;
use core::ptr;

pub struct FrameBufferWriter<'a> {
    framebuffer: &'a mut [u8],
    info: FrameBufferInfo,
    x_pos: i32,
    y_pos: i32,
    current_color: (u8, u8, u8), // RGB values for color
}

impl<'a> FrameBufferWriter<'a> {
    pub fn new(framebuffer: &'a mut [u8], info: FrameBufferInfo) -> Self {
        let mut writer = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
            current_color: (255, 255, 255), // Default white color
        };
        writer.clear();
        writer
    }

    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.framebuffer.fill(0); // Clears the framebuffer
    }

    // Handle screen wrapping and overflow
    fn handle_overflow(&mut self) {
        if self.x_pos >= self.info.width as i32 {
            self.x_pos = 0;
            self.y_pos += 1;
        }
        if self.y_pos >= self.info.height as i32 {
            self.scroll_screen();
        }
    }

    fn scroll_screen(&mut self) {
        // Logic to scroll screen: Shift lines up when the screen is full
        let row_size = (self.info.width * 4) as usize; // Assuming 4 bytes per pixel
        let num_rows = self.info.height as usize;
        for i in 0..(num_rows - 1) {
            let start = i * row_size;
            let end = start + row_size;
            self.framebuffer.copy_within(start..end, start + row_size);
        }
        // Clear last row
        let last_row_start = (num_rows - 1) * row_size;
        let last_row_end = last_row_start + row_size;
        self.framebuffer[last_row_start..last_row_end].fill(0);
        self.y_pos = (self.y_pos - 1).max(0); // Prevent negative y position
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => {
                self.x_pos = 0;
                self.y_pos += 1;
            }
            '\t' => {
                self.x_pos += 4; // Move by 4 characters for a tab
            }
            '\x1b' => { // Escape character
                // Handle escape sequences here
                self.handle_escape_sequence();
            }
            _ => {
                // Write the character to the framebuffer at the current position
                let pos = (self.y_pos as usize * self.info.width as usize + self.x_pos as usize) * 4; // 4 bytes per pixel
                if pos < self.framebuffer.len() {
                    let (r, g, b) = self.current_color;
                    self.framebuffer[pos] = r;   // Red channel
                    self.framebuffer[pos + 1] = g; // Green channel
                    self.framebuffer[pos + 2] = b; // Blue channel
                }
                self.x_pos += 1;
            }
        }

        self.handle_overflow();
    }

    fn handle_escape_sequence(&mut self) {
        // Example for handling color change like \cBlue
        // In real scenarios, you should parse the sequence to change the color
        self.current_color = (0, 0, 255); // Set color to blue for this example
    }
}

impl<'a> Write for FrameBufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

unsafe impl<'a> Send for FrameBufferWriter<'a> {}
unsafe impl<'a> Sync for FrameBufferWriter<'a> {}
