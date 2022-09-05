#![no_main]
#![no_std]
mod lang_items;

use core::arch::global_asm;
// include_str! 类似 C 的 #include, 直接把文件原地展开
global_asm!(include_str!("entry.asm"));