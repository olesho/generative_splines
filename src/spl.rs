pub mod spl {
    use std::{thread, time};
    use ndarray::Array1;
    use rand::Rng;
    use std::ops::{Add, Mul};
    use std::f64::consts::PI;
    use ndarray::{Array, prelude::*};
    use ndarray_rand::RandomExt;
    use ndarray_rand::rand_distr::Uniform;
    use std::sync::mpsc::{Sender};
    use std::sync::{Arc, Mutex};

    use crate::screen::screen::{Screen, send_buf};

    const TWOPI: f64 = 2.0 * PI;
    const STP: f64 = 0.000002;
    const INUM: u32 = 120;
    struct Spline {
        g: f64,
        path: ndarray::Array2<f64>,
        num_segments: u32,
        scale: ndarray::Array1<f64>,
        pnum: usize,
        interpolated_path: ndarray::Array2<f64>,
        noise: ndarray::Array1<f64>,
        i: i64,
    }

    fn new(path: ndarray::Array2<f64>, num_segments: u32, scale: ndarray::Array1<f64>) -> Spline {
        let pnum = path.column(0).len();
        let interpolated_path = rnd_interpolate(& mut path.clone(), num_segments);
        Spline {
            g: 0.5,
            path: path,
            num_segments: num_segments,
            scale: scale,
            pnum: pnum,
            interpolated_path: interpolated_path,
            noise: Array1::<f64>::zeros(pnum),
            i: 0,
        }
    }

    impl Spline {
        fn next(&mut self) -> Array2<f64> {
            let rand = Array::random(self.pnum, Uniform::new(-2.0, 0.0));
            let r = rand.map(|n| 1.0-n);
            self.noise = self.noise.clone().add(r.mul(self.scale.clone()));
            let a = Array::random(self.pnum, Uniform::new(0.0, TWOPI));

            let rnd_x = a.map(|n| n.cos()).mul(self.noise.clone());
            let rnd_y = a.map(|n| n.sin()).mul(self.noise.clone());
            let p = ndarray::stack(ndarray::Axis(1), &[rnd_x.view(), rnd_y.view()]).unwrap();

            self.path = self.path.clone().add(p);
            self.interpolated_path = rnd_interpolate(& mut self.path.clone(), self.num_segments);

            self.i += 1;
            self.interpolated_path.map(|n| *n + self.g)
        }
    }

    // cubic_spline v1.0.0 (working fine)
    fn rnd_interpolate(xy: &mut ndarray::Array2<f64>, num_segments: u32) -> Array2<f64> {
        use cubic_spline::{Points, Point, SplineOpts, TryFrom};
        let mut vec: Vec<Point> = Vec::<Point>::with_capacity(xy.column(0).len());
        for r in xy.rows() {
            vec.push(Point::new(r[0], r[1]));
        }
        let opts = SplineOpts::new().tension(0.5).num_of_segments(num_segments);
        let points = Points::try_from(&vec).unwrap();
        let result = points.calc_spline(&opts).unwrap();

        let result_vec = result.get_ref();
        let ax: Vec<f64> = result_vec.into_iter().map(|p| p.x).collect();
        let ay: Vec<f64> = result_vec.into_iter().map(|p| p.y).collect();
        ndarray::stack(ndarray::Axis(1), &[arr1(&ax).view(), arr1(&ay).view()]).unwrap()
    }

    pub fn fill_circle(tx: Sender<ndarray::Array2<f64>>) {
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let pnum: usize = 10000;
            let shift = rng.gen_range(0.0..TWOPI);
            let a2 = Array1::linspace(0.0, TWOPI, pnum);
            let a = a2.map(|n| n + shift);
    
            let path_stack = ndarray::stack(ndarray::Axis(1), &[a.map(|n| n.cos()).view(), a.map(|n| n.sin()).view()]).unwrap();
            let scale_path = rng.gen_range(0.1..0.5);
            let path = path_stack.map(|n| n*scale_path+0.5);

            for _ in 0..10000 {
                tx.send(path.clone()).unwrap();
            }
        });
    }


    pub fn fill_circle_splines (screen: Arc<Mutex<Screen>>) {
        let mut rng = rand::thread_rng();
    
        let scale_path= rng.gen_range(0.1..0.4);
        let pnum: usize = rng.gen_range(150..250);
        let shift = rng.gen_range(0.0..TWOPI);

        let a2 = Array1::linspace(0.0, TWOPI, pnum);
        let a = a2.map(|n| n + shift);

        let path_stack = ndarray::stack(ndarray::Axis(1), &[a.map(|n| n.cos()).view(), a.map(|n| n.sin()).view()]).unwrap();
        let path = path_stack.map(|n| n * scale_path);

        //let scale = Array::range(0., (pnum as f64)*STP, STP);
        let scale = Array::range(-1.0 * (pnum as f64)*STP / 2.0, (pnum as f64)*STP / 2.0, STP );
        let mut s = new(path, INUM, scale);
        for _ in 0..300 {
            send_buf(screen.clone(), s.next());
        }
    }

    pub fn fill_rand_splines(tx: Sender<ndarray::Array2<f64>>) {
        extern crate peroxide;
        use peroxide::prelude::{CubicSpline};

        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..2000 {
                let r1 = Array::random((1, 4), Uniform::new(0., 1.));
                let r2 = Array::random((1, 4), Uniform::new(0., 1.));
                
                let x = r1.into_raw_vec();
                let y = r2.into_raw_vec();
    
                let spl = CubicSpline::from_nodes(x, y);
    
                let a = Array1::linspace(0.01, 0.99, rng.gen_range(200..1800));
                let b = a.map(|n| spl.eval(*n));
                let d = ndarray::stack(ndarray::Axis(1), &[a.view(), b.view()]).unwrap();
    
                tx.send(d).unwrap();
                thread::sleep(time::Duration::from_millis(100));   
            }
        });
    }

    pub fn fill_rand(tx: Sender<ndarray::Array2<f64>>) {
        thread::spawn(move || {
            for _ in 0..1000 {
                let a = Array::random((5, 2), Uniform::new(0., 1.));
                tx.send(a).unwrap();
                thread::sleep(time::Duration::from_millis(200));   
            }
        });
    }
}