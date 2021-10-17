pub mod spl {
    use std::{env, thread, time};
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
    
    #[derive(Clone)]
    pub struct CircleSplineOpts {
        pub stp: f64,
        pub inum: u32,
        pub pnum_from: usize,
        pub pnum_to: usize,
        pub iterations: usize,
    }

    pub fn from_env <'a> (s: & 'a mut CircleSplineOpts) {
        // let mut s = CircleSplineOpts{
        //     stp: default.stp,
        //     inum: default.inum,
        //     pnum_from: default.pnum_from,
        //     pnum_to: default.pnum_to,
        //     iterations: default.iterations,
        // };

        match env::var("STP") {
            Ok(v) => {
                s.stp = v.parse::< f64 >().unwrap();
            },
            Err(_) => {}
        }

        match env::var("INUM") {
            Ok(v) => {
                s.inum = v.parse::< u32 >().unwrap();
            },
            Err(_) => {}
        }

        match env::var("PNUM_FROM") {
            Ok(v) => {
                s.pnum_from = v.parse::< usize >().unwrap();
            },
            Err(_) => {}
        }

        match env::var("PNUM_TO") {
            Ok(v) => {
                s.pnum_to = v.parse::< usize >().unwrap();
            },
            Err(_) => {}
        }

        match env::var("ITERATIONS") {
            Ok(v) => {
                s.iterations = v.parse::< usize >().unwrap();
            },
            Err(_) => {}
        }
    }
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

    pub fn fill_circle_splines (screen: Arc<Mutex<Screen>>, opts: CircleSplineOpts) {
        let mut rng = rand::thread_rng();
    
        let scale_path= rng.gen_range(0.1..0.4);
        let pnum: usize = rng.gen_range(opts.pnum_from..opts.pnum_to);
        let shift = rng.gen_range(0.0..TWOPI);

        let a2 = Array1::linspace(0.0, TWOPI, pnum);
        let a = a2.map(|n| n + shift);

        let path_stack = ndarray::stack(ndarray::Axis(1), &[a.map(|n| n.cos()).view(), a.map(|n| n.sin()).view()]).unwrap();
        let path = path_stack.map(|n| n * scale_path);

        //let scale = Array::range(0., (pnum as f64)*STP, STP);
        let scale = Array::range(-1.0 * (pnum as f64) * opts.stp / 2.0, (pnum as f64)*opts.stp / 2.0, opts.stp );
        let mut s = new(path, opts.inum, scale);
        for _ in 0..opts.iterations {
            send_buf(screen.clone(), s.next());
        }
    }



    pub fn fill_rand_splines(screen: Arc<Mutex<Screen>>) {
        extern crate peroxide;
        use peroxide::prelude::{CubicSpline};

        let mut rng = rand::thread_rng();
        for _ in 0..700 {
            let r1 = Array::random((1, 4), Uniform::new(0., 1.));
            let r2 = Array::random((1, 4), Uniform::new(0., 1.));
            
            let x = r1.into_raw_vec();
            let y = r2.into_raw_vec();

            let spl = CubicSpline::from_nodes(x, y);

            let a = Array1::linspace(0.01, 0.99, rng.gen_range(200..1800));
            let b = a.map(|n| spl.eval(*n));
            let d = ndarray::stack(ndarray::Axis(1), &[a.view(), b.view()]).unwrap();

            send_buf(screen.clone(), d);
        }
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



    ///////////
    /// 
    /// 
    /// 
    /// 
    use num::Complex;
    use cubic_spline::{Points, Point, SplineOpts, TryFrom};

    fn circle(x: f64, y: f64, segments: usize, scale: f64) -> ndarray::Array2<f64> {
        let a = Array1::linspace(0.0, TWOPI, segments);
        ndarray::stack(ndarray::Axis(1), &[a.map(|n| n.cos() * scale/2.0 + y).view(), a.map(|n| n.sin() * scale/2.0 + x).view()])
            .unwrap()
    }

    pub struct ComplexEquationParams {
        pub params: Vec<[f64; 3]>
    }

    impl ComplexEquationParams {
        fn f(&self, theta: f64) -> Complex<f64> {
            let row = self.params[0];
            let mut c = Complex::new(row[0], row[1] * theta).exp() * row[2];
            for row in self.params[1..].iter() {
                c = c + Complex::new(row[0], row[1] * theta).exp() * row[2];
            }
            return c;
        }

        fn complex_circle(&self, pnum: usize, shift_x: f64, shift_y: f64 ) -> ndarray::Array2<f64> {
            let l = Array1::linspace(0.0, TWOPI, pnum);
            let c = l.map(|theta| self.f(*theta));
            let x = c.map(|n| n.re + shift_x);
            let y = c.map(|n| n.im + shift_y);
            ndarray::stack(ndarray::Axis(1), &[x.view(), y.view()]).unwrap()
        }
    }

    fn complex_circle(pnum: usize, f: fn(f64) -> Complex<f64>, shift_x: f64, shift_y: f64 ) -> ndarray::Array2<f64> {
        let l = Array1::linspace(0.0, TWOPI, pnum);
        let c = l.map(|theta| f(*theta));
        let x = c.map(|n| n.re + shift_x);
        let y = c.map(|n| n.im + shift_y);
        ndarray::stack(ndarray::Axis(1), &[x.view(), y.view()]).unwrap()
    }

    pub fn fill_complex_circle(screen: Arc<Mutex<Screen>>, params: Vec<[f64; 3]>) {
        let cc = ComplexEquationParams{params: params};
        send_buf(screen.clone(), cc.complex_circle(10000, 0.5, 0.5));
    }

    pub fn random_complex_splines(screen: Arc<Mutex<Screen>>) {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            fill_complex_spline(screen.clone(), CircleSplineOpts{
                inum: 200,
                stp: 0.000001,
                pnum_from: 150,
                pnum_to: 181,
                iterations: 50,
            }, vec![
                    [rng.gen_range(-5.0..5.0), 10.0, 0.01],
                    [3.0, -1.0, 0.01],
                    [0.5, 0.5, 0.01],
                ]);
            
            thread::sleep(time::Duration::from_millis(1000))
        }
    }

    pub fn fill_complex_spline(screen: Arc<Mutex<Screen>>, opts: CircleSplineOpts, params: Vec<[f64; 3]>) {
        let mut rng = rand::thread_rng();
    
        let scale_path = 0.8;
        let pnum: usize = rng.gen_range(opts.pnum_from..opts.pnum_to);
       
        let cc = ComplexEquationParams{params: params};
        let path_stack = cc.complex_circle(pnum, 0.0, 0.0);

        let path = path_stack.map(|n| n * scale_path);

        let scale = Array::range(-1.0 * (pnum as f64) * opts.stp / 2.0, (pnum as f64)*opts.stp / 2.0, opts.stp );
        let mut s = new(path, opts.inum, scale);
        for _ in 0..opts.iterations {
            send_buf(screen.clone(), s.next());
        }
    }

    fn new_spline(x1: f64, x2: f64, y1: f64, y2: f64, points: usize) -> Array2<f64> {
        let x = Array::random(points, Uniform::new(x1, x2));
        let y = Array::random(points, Uniform::new(y1, y2));

        let xy = ndarray::stack(ndarray::Axis(1), &[x.view(), y.view()]).unwrap();
        let mut vec: Vec<Point> = Vec::<Point>::with_capacity(xy.column(0).len());
        for r in xy.rows() {
            vec.push(Point::new(r[0], r[1]));
        }
        let opts = SplineOpts::new().tension(0.5).num_of_segments(128);
        let points = Points::try_from(&vec).unwrap();
        let result = points.calc_spline(&opts).unwrap();

        let result_vec = result.get_ref();
        let ax: Vec<f64> = result_vec.into_iter().map(|p| p.x).collect();
        let ay: Vec<f64> = result_vec.into_iter().map(|p| p.y).collect();
        ndarray::stack(ndarray::Axis(1), &[arr1(&ax).view(), arr1(&ay).view()]).unwrap()
    }

    pub struct HieroglyphOpts {
        pub row_count: usize,// = 12;
        pub col_count: usize,// = 12;
        pub padding: f64,// = 0.02;
        pub width: f64,// = 0.08;
        pub height: f64,// = 0.03;
        pub points: usize,// = 6;
    }

    pub fn fill_spline_hieroglyph(screen: Arc<Mutex<Screen>>, opts: HieroglyphOpts) {
        for i in 0..opts.row_count {
            for j in 0..opts.col_count {
                let ii = i as f64;
                let jj = j as f64;

                let x1 = opts.padding + ((opts.height + opts.padding) * ii);
                let x2 = x1 + opts.height;

                let y1 = opts.padding + ((opts.width + opts.padding) * jj);
                let y2 = y1 + opts.width;

                let d = new_spline(x1, x2, y1, y2, opts.points);
                send_buf(screen.clone(), d)
            }
        }
    }

    fn new_spline_row(x1: f64, x2: f64, y1: f64, y2: f64, points: usize, iterations: usize) -> Array2<f64> {
        let mut rng = rand::thread_rng();

        let points_count = rng.gen_range(points-2..points+2);

        let mut xx: ndarray::Array1<f64> = arr1(&[]);
        let mut yy: ndarray::Array1<f64> = arr1(&[]);
        for i in 0..iterations {

            // space
            if rng.gen_range(0..3) == 3 {
                continue;
            }

            let mut x = Array::random(points_count, Uniform::new(x1, x2));

            // transition (up or down)
            // if i % 2 == 0 {
            //     x[points-1] = x1;
            // } else {
            //     x[points-1] = x2;
            // }

            let mut y = Array::random(points_count, Uniform::new(y1, y2));
            y = y.map(|n| n + i as f64 * (y2-y1));
            xx.append(Axis(0), x.view()).unwrap();
            yy.append(Axis(0), y.view()).unwrap();
        }
        let xy = ndarray::stack(ndarray::Axis(1), &[xx.view(), yy.view()]).unwrap();
        let mut vec: Vec<Point> = Vec::<Point>::with_capacity(xy.column(0).len());
        for r in xy.rows() {
            vec.push(Point::new(r[0], r[1]));
        }
        let opts = SplineOpts::new().tension(0.5).num_of_segments(128);
        let points = Points::try_from(&vec).unwrap();
        let result = points.calc_spline(&opts).unwrap();

        let result_vec = result.get_ref();
        let mut ax: Vec<f64> = result_vec.into_iter().map(|p| p.x).collect();
        let mut ay: Vec<f64> = result_vec.into_iter().map(|p| p.y).collect();
        
        // incline
        let mut incline = y2-y1 / 2.0;
        let change = incline / ay.len() as f64;
        for i in 0..ay.len() {
            ay[i] += ay[i] + incline;
            incline = incline - change;
        }

        ndarray::stack(ndarray::Axis(1), &[arr1(&ax).view(), arr1(&ay).view()]).unwrap()
    }

    pub struct ScriptOpts {
        pub row_count: usize,// = 12;
        pub col_count: usize,// = 12;
        pub padding: f64,// = 0.02;
        pub char_width: f64,// = 0.08;
        pub height: f64,// = 0.03;
        pub points: usize,// = 6;
    }

    pub fn fill_spline_script(screen: Arc<Mutex<Screen>>, opts: ScriptOpts) {
        for i in 0..opts.row_count {
            let ii = i as f64;

            let x1 = opts.padding + ((opts.height + opts.padding) * ii);
            let x2 = x1 + opts.height;

            let y1 = opts.padding;
            let y2 = y1 + opts.char_width;

            let d = new_spline_row(x1, x2, y1, y2, opts.points, opts.col_count);
            send_buf(screen.clone(), d);
        }
    }
}