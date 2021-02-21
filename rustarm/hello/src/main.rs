extern crate sys_info;

use sys_info::*;

fn main() {
    println!("Hello, world!");
    println!("os: {} {}", os_type().unwrap(), os_release().unwrap());
    println!("cpu: {} cores, {} MHz", cpu_num().unwrap(), cpu_speed().unwrap());
    println!("proc total: {}", proc_total().unwrap());
    println!("hostname: {}", hostname().unwrap());
    let mem = mem_info().unwrap();
    println!("mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
             mem.total, mem.free, mem.avail, mem.buffers, mem.cached);
}
