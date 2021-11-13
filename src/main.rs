use core::time;
use std::{env, thread};
use std::sync::{Arc, Mutex};
pub mod screen;
pub mod spl;

use rand::Rng;

use crate::screen::screen::{set_color, set_bg, render, renderWithTimeout};
fn main() {
    //paint_circle_spline();
    //paint_complex_spline();
    paint_random();
    //paint_rand_splines();
    //paint_complex_circle();
    //paint_script();
    //paint_hieroglyphs();
}

fn paint_rand_splines() {
    let s = screen::screen::new(1000,1000);
    let m = Mutex::new(s);
    let sm = Arc::new(m);
    set_color(sm.clone(), [0.3, 1.0, 1.0, 1.0]);
    set_bg(sm.clone(), [0.02, 0.05, 0.05, 0.05]);
    spl::spl::fill_rand_splines(sm.clone());
    render(sm.clone());
}

fn paint_circle_spline() {
    let s = screen::screen::new(1000,1000);
    let m = Mutex::new(s);
    let sm = Arc::new(m);
    set_color(sm.clone(), [0.02, 1.0, 1.0, 1.0]);
    set_bg(sm.clone(), [0.02, 0.1, 0.35, 0.7]);

    let mut ncircles = 3;
    match env::var("N") {
        Ok(v) => {
            ncircles = v.parse::< usize >().unwrap();
        },
        Err(_) => {}
    }

    let mut opts = spl::spl::CircleSplineOpts{
        inum: 200,
        stp: 0.000002,
        pnum_from: 100,
        pnum_to: 150,
        iterations: 80,
    };

    spl::spl::from_env(& mut opts);

    let _: Vec<_> = (0..ncircles).map(|_| { 
        let sc = sm.clone();
        let o = opts.clone();
        thread::spawn(move || {  
            spl::spl::fill_circle_splines(sc.clone(), o);
        });
    }).collect();
    render(sm.clone());
}

fn paint_complex_circle() {
    let s = screen::screen::new(1000,1000);
    let m = Mutex::new(s);
    let sm = Arc::new(m);
    set_color(sm.clone(), [0.2, 0.6, 1.0, 0.5]);
    set_bg(sm.clone(), [1.0, 0.0, 0.0, 0.0]);

    spl::spl::fill_complex_circle(sm.clone(), vec![[1.0, 1.0, 1.0]]);
    render(sm.clone());
}

fn paint_complex_spline() {
    let s = screen::screen::new(1000,1000);
    let m = Mutex::new(s);
    let sm = Arc::new(m);
    set_color(sm.clone(), [0.2, 0.6, 1.0, 0.5]);
    set_bg(sm.clone(), [1.0, 0.0, 0.0, 0.0]);
    
    // let params = vec![
    //     [1.0, 10.0, 0.01], 
    //     [3.0, -3.0, 0.01], 
    //     [0.5, -0.5, 0.001]];
    // spl::spl::fill_complex_spline(sm.clone(), spl::spl::CircleSplineOpts{
    //     inum: 200,
    //     stp: 0.000002,
    //     pnum_from: 100,
    //     pnum_to: 141,
    //     iterations: 40,
    // }, params);

    spl::spl::fill_complex_spline(sm.clone(), spl::spl::CircleSplineOpts{
        inum: 200,
        stp: 0.000002,
        pnum_from: 100,
        pnum_to: 151,
        iterations: 80,
    }, vec![
        [3.1, 1.0, 0.01], 
        [3.0, -3.0, 0.01], 
        [0.5, 0.5, 0.01]]);

    // spl::spl::fill_complex_spline(sm.clone(), spl::spl::CircleSplineOpts{
    //     inum: 200,
    //     stp: 0.000001,
    //     pnum_from: 150,
    //     pnum_to: 181,
    //     iterations: 50,
    // }, vec![
    //         [2.1, 10.0, 0.01],
    //         [3.0, -1.0, 0.01],
    //         [0.5, 0.5, 0.01],
    //     ]);

    

    render(sm.clone());
}

fn paint_random() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let s = screen::screen::new(1000,1000);
        let m = Mutex::new(s);
        let sm = Arc::new(m);
        set_color(sm.clone(), [0.2, 0.6, 1.0, 0.5]);
        set_bg(sm.clone(), [1.0, 0.0, 0.0, 0.0]);

        spl::spl::fill_complex_spline(sm.clone(), spl::spl::CircleSplineOpts{
            inum: 200,
            stp: 0.000001,
            pnum_from: 150,
            pnum_to: 181,
            iterations: 50,
        }, vec![
                [rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0), rng.gen_range(-0.1..0.1)],
                [rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0), rng.gen_range(-0.1..0.1)],
                [rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0), rng.gen_range(-0.1..0.1)],
            ]);
        renderWithTimeout(sm.clone());
    }
}

fn paint_script() {
    let s = screen::screen::new(1000,1000);
    let m = Mutex::new(s);
    let sm = Arc::new(m);
    set_color(sm.clone(), [0.1, 1.0, 1.0, 1.0]);
    set_bg(sm.clone(), [0.1, 0.1, 0.15, 0.1]);
    spl::spl::fill_spline_script(sm.clone(), spl::spl::ScriptOpts{
        row_count: 12,
        col_count: 20,
        padding: 0.04,
        char_width: 0.02,
        height: 0.03,
        points: 6,
    });
    render(sm.clone());
}

fn paint_hieroglyphs() {
    let s = screen::screen::new(1000,1000);
    let m = Mutex::new(s);
    let sm = Arc::new(m);
    set_color(sm.clone(), [0.1, 1.0, 1.0, 1.0]);
    set_bg(sm.clone(), [0.1, 0.15, 0.35, 0.4]);
    spl::spl::fill_spline_hieroglyph(sm.clone(), spl::spl::HieroglyphOpts {
        row_count: 16,
        col_count: 16,
        padding:  0.02,
        width: 0.04,
        height: 0.03,
        points: 10,
    });
    render(sm.clone());
}