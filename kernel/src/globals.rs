use core::mem::MaybeUninit;

use bootloader_api::info::{FrameBuffer, FrameBufferInfo};
use spin::Mutex;

pub static FRAMEBUFFER: Mutex<MaybeUninit<&'static mut [u8]>> = Mutex::new(MaybeUninit::uninit());
pub static FRAMEINFO: Mutex<MaybeUninit<FrameBufferInfo>> = Mutex::new(MaybeUninit::uninit());
