use core::ptr;
use wideslice::*;

static mut BUF: &mut [f64] = &mut [0.0; 8];
static mut PTR_BUF: &mut [*mut f64] = &mut [ptr::null_mut(); 4];

fn main() {
    let mut mem = StaticMem::new(unsafe { &mut BUF }, unsafe { &mut PTR_BUF });
    //let mut mem = HeapMem::new(2, 4);
    println!("mem: {:#?}", mem);

    let ws = mem.into_wideslice();
    println!("ws: {:#?}", ws);

    ws.slices.rotate_right(1);

    println!("ws: {:#?}", ws);

    println!("s1: {:#?}", &ws[0]);
    println!("s2: {:#?}", &ws[1]);
    println!("s3: {:#?}", &ws[2]);
    println!("s4: {:#?}", &ws[3]);

    // println!("");
    // println!("");
    // println!("");

    // let mut mem = StackMem::new();
    // println!("mem: {:?}", mem);

    // let ws = mem.into_wideslice();
    // println!("ws: {:?}", ws);

    // ws.slices.rotate_right(1);

    // println!("ws: {:?}", ws);

    // println!("s1: {:?}", ws.as_slice(0));
    // println!("s2: {:?}", ws.as_slice(1));
    // println!("s3: {:?}", ws.as_slice(2));
    // println!("s4: {:?}", ws.as_slice(3));
}
