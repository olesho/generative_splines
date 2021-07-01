pub mod screen {
    extern crate minifb;
    use std::usize;
    use minifb::{Key, Window, WindowOptions};
    use std::sync::mpsc::{channel, Sender, Receiver};
    pub struct Screen {
        width: usize,
        height: usize,
        buffer: Vec<u32>,
        f64rgba: [f64; 4],
        rgba: [u8; 4],
        pub tx: Sender<ndarray::Array2<f64>>,
        rx: Receiver<ndarray::Array2<f64>>,
    }

    pub fn new (w: usize, h: usize) -> Screen {
        let (tx, rx): (Sender<ndarray::Array2<f64>>, Receiver<ndarray::Array2<f64>>) = channel();
        let f64rgba = [1.0, 0.0, 0.0, 0.0];
        Screen{
            width: w,
            height: h,
            buffer: vec![0; w * h],
            f64rgba: f64rgba,
            rgba: as_u8(&f64rgba),
            tx,
            rx,
        }
    }

    impl Screen {
        pub fn set_color(&mut self, f64rgba: [f64; 4]) {
            self.f64rgba[0] = f64rgba[0];
            self.f64rgba[1] = f64rgba[1] * f64rgba[0];
            self.f64rgba[2] = f64rgba[2] * f64rgba[0];
            self.f64rgba[3] = f64rgba[3] * f64rgba[0];
            self.rgba = as_u8(&self.f64rgba);
        }

        pub fn set_bg(&mut self, f64rgba: [f64; 4]) {
            let rgba = as_u8(&f64rgba);
            self.buffer.fill(as_u32_be(&rgba));
        }

        pub fn dot(&mut self, xf64: f64, yf64: f64) {
            let mut x = (self.width as f64 * xf64) as usize;
            if x >= self.width {
                x = self.width-1;
            }
            let mut y = (self.height as f64 * yf64) as usize;
            if y >= self.height {
                y = self.height-1;
            }

            let mut current = as_f64(&u32_to_u8(self.buffer[x * self.width + y]));

            let invaa = 1.0 - self.f64rgba[0];
            current[0] = self.f64rgba[0] + current[0] * invaa;
            current[1] = self.f64rgba[1] + current[1] * invaa;
            current[2] = self.f64rgba[2] + current[2] * invaa;
            current[3] = self.f64rgba[3] + current[3] * invaa;

            self.buffer[x * self.width + y] = as_u32_be(&as_u8(&current)); //as_u32_be(&self.rgba);
        }

        pub fn dots(&mut self, xy: ndarray::Array2<f64>) {
            for row in xy.rows().into_iter() {
                if !(row[0] > 1.0 || row[0] < 0.0 || row[1] > 1.0 || row[1] < 0.0) {
                    let mut x = (self.width as f64 * row[0]) as usize;
                    if x >= self.width {
                        x = self.width-1;
                    }
                    let mut y = (self.height as f64 * row[1]) as usize;
                    if y >= self.height {
                        y = self.height-1;
                    }
        
                    let mut current = as_f64(&u32_to_u8(self.buffer[x * self.width + y]));
        
                    let invaa = 1.0 - self.f64rgba[0];
                    current[0] = self.f64rgba[0] + current[0] * invaa;
                    current[1] = self.f64rgba[1] + current[1] * invaa;
                    current[2] = self.f64rgba[2] + current[2] * invaa;
                    current[3] = self.f64rgba[3] + current[3] * invaa;
        
                    self.buffer[x * self.width + y] = as_u32_be(&as_u8(&current));
                }
            }
        }

        pub fn render(&mut self) {           
            let mut window = Window::new(
                "Test - ESC to exit",
                self.width,
                self.height,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

            // Limit to max ~60 fps update rate
            window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
            while window.is_open() && !window.is_key_down(Key::Escape) {
                match self.rx.recv() {
                    Ok(xys) => {
                        for row in xys.rows().into_iter() {
                            if !(row[0] > 1.0 || row[0] < 0.0 || row[1] > 1.0 || row[1] < 0.0) {
                                self.dot(row[0], row[1]);
                            }
                        }

                        window
                            .update_with_buffer(&self.buffer, self.width, self.height)
                            .unwrap();
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        }    
    }

    fn as_u8(array: &[f64; 4]) -> [u8; 4] {
        let r = (array[0] * 255.0) as u8;
        let g = (array[1] * 255.0) as u8;
        let b = (array[2] * 255.0) as u8;
        let a = (array[3] * 255.0) as u8;
        [r, g, b, a]
    }


    fn as_f64(array: &[u8; 4]) -> [f64; 4] {
        let r = array[0] as f64 / 255.0;
        let g = array[1] as f64 / 255.0;
        let b = array[2] as f64 / 255.0;
        let a = array[3] as f64 / 255.0;
        [r, g, b, a]
    }

    fn as_u32_be(array: &[u8; 4]) -> u32 {
        ((array[0] as u32) << 24) +
        ((array[1] as u32) << 16) +
        ((array[2] as u32) <<  8) +
        ((array[3] as u32) <<  0)
    }

    // fn as_u32_le(array: &[u8; 4]) -> u32 {
    //     ((array[0] as u32) <<  0) +
    //     ((array[1] as u32) <<  8) +
    //     ((array[2] as u32) << 16) +
    //     ((array[3] as u32) << 24)
    // }

    fn u32_to_u8(x:u32) -> [u8;4] {
        let b1 : u8 = ((x >> 24) & 0xff) as u8;
        let b2 : u8 = ((x >> 16) & 0xff) as u8;
        let b3 : u8 = ((x >> 8) & 0xff) as u8;
        let b4 : u8 = (x & 0xff) as u8;
        return [b1, b2, b3, b4]
    }
}
