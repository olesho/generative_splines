pub mod screen;
pub mod spl;
fn main() {
    let mut s = screen::screen::new(1000,1000);
    s.set_color([0.01, 1.0, 0.65, 1.0]);
    s.set_bg([1.0, 1.0, 1.0, 1.0]);

    // s.set_color([1.0, 1.0, 1.0, 0.0]);
    // s.set_bg([1.0, 0.0, 0.0, 0.0]);

    //spl::spl::fill_rand(s.tx.clone());
    
    let _: Vec<_> = (0..16).map(|_| { spl::spl::fill_circle_splines(s.tx.clone()) }).collect();
    // spl::spl::fill_circle_splines(s.tx.clone());


    //spl::spl::fill_circle(s.tx.clone());

    s.render();
}

