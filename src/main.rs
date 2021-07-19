pub mod screen;
pub mod spl;
fn main() {
    let mut s = screen::screen::new(1000,1000);
    


    //spl::spl::fill_rand(s.tx.clone());
    
    // s.set_color([0.01, 1.0, 0.0, 1.0]);
    // s.set_bg([1.0, 1.0, 1.0, 1.0]);
    //let _: Vec<_> = (0..4).map(|_| { spl::spl::fill_circle_splines(s.tx.clone()) }).collect();
    //spl::spl::fill_circle_splines(s.tx.clone());

    s.set_color([1.0, 1.0, 1.0, 0.0]);
    s.set_bg([1.0, 0.0, 0.0, 0.0]);
    // spl::spl::fill_spline_hieroglyph(s.tx.clone(), spl::spl::HieroglyphOpts{
    //     row_count: 10,
    //     col_count: 10,
    //     padding: 0.02,
    //     width: 0.08,
    //     height: 0.08,
    //     points: 9,
    // });

    // spl::spl::fill_spline_script(s.tx.clone(), spl::spl::ScriptOpts{
    //         row_count: 20,
    //         col_count: 30,
    //         padding: 0.02,
    //         char_width: 0.0152, 
    //         height: 0.03,
    //         points: 6,
    // });

    spl::spl::fill_circle(s.tx.clone());

    s.render();
}

