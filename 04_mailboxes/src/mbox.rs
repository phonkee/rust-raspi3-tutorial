/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use super::MMIO_BASE;
use volatile_register::{RO, WO};

pub const VIDEOCORE_MBOX: u32 = MMIO_BASE + 0xB880;

#[allow(non_snake_case)]
#[repr(C, packed)]
pub struct Registers {
    READ: RO<u32>,       // 0x00
    reserved: [u8; 0xC], // 0x04
    POLL: RO<u32>,       // 0x10
    SENDER: RO<u32>,     // 0x14
    STATUS: RO<u32>,     // 0x18
    CONFIG: RO<u32>,     // 0x1C
    WRITE: WO<u32>,      // 0x20
}

// Custom errors
pub enum MboxError {
    ResponseError,
    UnknownError,
}
pub type Result<T> = ::core::result::Result<T, MboxError>;

// Channels
pub mod channel {
    pub const PROP: u32 = 8;
}

// Tags
pub mod tag {
    pub const GETSERIAL: u32 = 0x10004;
    pub const LAST: u32 = 0;
}

// Responses
mod response {
    pub const SUCCESS: u32 = 0x80000000;
    pub const ERROR: u32 = 0x80000001; // error parsing request buffer (partial response)
}

pub const REQUEST: u32 = 0;
const FULL: u32 = 0x80000000;
const EMPTY: u32 = 0x40000000;

// Public interface to the mailbox
#[repr(C)]
pub struct Mbox {
    // The address for buffer needs to be 16-byte aligned so that the
    // Videcore can handle it properly. We don't take precautions here
    // to achieve that, but for now it just works. Since alignment of
    // data structures in Rust is a bit of a hassle right now, we just
    // close our eyes and roll with it.
    pub buffer: [u32; 36],
    registers: *const Registers,
}

impl Mbox {
    pub fn new() -> Mbox {
        Mbox {
            buffer: [0; 36],
            registers: VIDEOCORE_MBOX as *const Registers,
        }
    }

    /// Make a mailbox call. Returns Err(MboxError) on failure, Ok(()) success
    pub fn call(&mut self, channel: u32) -> Result<()> {
        // wait until we can write to the mailbox
        loop {
            unsafe {
                if !(((*self.registers).STATUS.read() & FULL) == FULL) {
                    break;
                }
                asm!("nop" :::: "volatile");
            }
        }

        // write the address of our message to the mailbox with channel identifier
        unsafe {
            (*self.registers)
                .WRITE
                .write(((self.buffer.as_mut_ptr() as u32) & !0xF) | (channel & 0xF));
        }

        // now wait for the response
        loop {
            // is there a response?
            loop {
                unsafe {
                    if !(((*self.registers).STATUS.read() & EMPTY) == EMPTY) {
                        break;
                    }
                    asm!("nop" :::: "volatile");
                }
            }

            let resp: u32 = unsafe { (*self.registers).READ.read() };

            // is it a response to our message?
            if ((resp & 0xF) == channel) && ((resp & !0xF) == (self.buffer.as_mut_ptr() as u32)) {
                // is it a valid successful response?
                return match self.buffer[1] {
                    response::SUCCESS => Ok(()),
                    response::ERROR => Err(MboxError::ResponseError),
                    _ => Err(MboxError::UnknownError),
                };
            }
        }
    }
}
