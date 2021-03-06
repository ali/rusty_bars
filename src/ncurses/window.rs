#![allow(missing_copy_implementations)]
extern crate libc;

use std::ffi::CString;
use self::libc::{c_int, c_char};
use ncurses::ext;


/// Safe wrapper for the ncurses endwin function. Call this when you are done
/// with ncurses.
pub fn endwin() -> Result<(), c_int> {
    let result = unsafe{ ext::endwin() };
    if result == 0 {
        return Ok(())
    } else {
        return Err(result)
    }
}


/// Initialize the screen and get a window
fn initscr() -> Result<Window, c_int> {
    let window = unsafe { ext::initscr() };
    if window.is_null() {
        Err(-1)
    } else {
        Ok(Window { w: window })
    }
}


/// Wraps an ncruses WINDOW struct with the basic functions for manipulating
/// the window.
pub struct Window {
    w: *mut ext::Window
}


impl Window {
    pub fn new() -> Window {
        initscr().unwrap()
    }

    /// Add a string to the screen starting at the given location
    pub fn addstr(&mut self, y: c_int, x: c_int, text: &str) -> Result<c_int, c_int> {
        let text_vec: Vec<u8> = text.bytes().collect();
        let c_text =  CString::new(text_vec).unwrap();
        handle_err(unsafe{
            ext::mvwaddstr(self.w, y, x, c_text.as_ptr())
        })
    }

    /// Add a raw array of c_char to the window
    pub fn addbytes(&mut self, y: c_int, x: c_int, text: &Vec<c_char>) -> Result<c_int, c_int> {
        handle_err(unsafe{
            ext::mvwaddnstr(self.w, y, x, text.as_ptr(), text.len() as c_int)
        })
    }

    /// Refresh the output on the display
    pub fn refresh(&mut self) -> Result<c_int, c_int> {
        handle_err(unsafe{ ext::wrefresh(self.w) })
    }

    /// Get the maximum y on the screen
    pub fn get_max_y(&self) -> Result<c_int, c_int> {
        handle_err(unsafe{ ext::getmaxy(self.w) })
    }

    /// Get the maximum x on the screen
    pub fn get_max_x(&self) -> Result<c_int, c_int> {
        handle_err(unsafe{ ext::getmaxx(self.w) })
    }

    /// Get a tuple containing the maximum y and x on the screen
    pub fn get_max_yx(&self) -> Result<(c_int, c_int), c_int> {
        Ok((try!(self.get_max_y()), try!(self.get_max_x())))
    }

    /// Set the visibility of the cursor
    pub fn curs_set(&mut self, visibility: c_int) -> Result<c_int, c_int> {
        handle_err(unsafe{ ext::curs_set(visibility) })
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        endwin().unwrap();
    }
}


/// Turns an ncurses error code into a result so we can toss errors up the stack
fn handle_err(result: c_int) -> Result<c_int, c_int> {
    if result < 0 {
        Err(result)
    } else {
        Ok(result)
    }
}
