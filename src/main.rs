use std::{thread};
use std::sync::{Arc, Mutex};
pub mod screen;
pub mod spl;

use crate::screen::screen::{set_color, set_bg, render};
fn main() {
    let s = screen::screen::new(1000,1000);
    let m = Mutex::new(s);

    let sm1 = Arc::new(m);
    let sm2 = Arc::clone(&sm1);
    let sm3 = Arc::clone(&sm1);
    let sm4 = Arc::clone(&sm1);

    // set_color(sm1, [0.001, 0.3, 0.65, 1.0]);
    // set_bg(sm2, [1.0, 1.0, 1.0, 1.0]);

    set_color(sm1, [0.02, 1.0, 1.0, 1.0]);
    set_bg(sm2, [1.0, 0.1, 0.35, 0.7]);

    // s.set_color([1.0, 1.0, 1.0, 0.0]);
    // s.set_bg([1.0, 0.0, 0.0, 0.0]);

    //spl::spl::fill_rand(s.tx.clone());
    
    // let _: Vec<_> = (0..7).map(|_| { 
    //     let sm = Arc::clone(&sm3);
    //     thread::spawn(move || {  
    //         for _ in 0..1 {
    //             spl::spl::fill_circle_splines(sm.clone());
    //         }
    //     });
    // }).collect();

    spl::spl::fill_circle_splines(sm3);

    render(sm4);
}

