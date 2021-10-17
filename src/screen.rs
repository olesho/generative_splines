pub mod screen {
    extern crate minifb;
    use std::{thread::sleep, time::Duration};
    use std::usize;
    use minifb::{Key, Window, WindowOptions};
    use std::sync::{Arc, Mutex};
    pub struct Screen {
        width: usize,
        height: usize,
        buffer: Vec<u32>,
        f64rgba: [f64; 4],
        rgba: [u8; 4],
    }

    pub fn new (w: usize, h: usize) -> Screen {
        let f64rgba = [0.0, 0.0, 0.0, 0.0];
        Screen{
            width: w,
            height: h,
            buffer: vec![0; w * h],
            f64rgba,
            rgba: as_u8(&f64rgba),
        }
    }
    
    pub fn set_color(screen: Arc<Mutex<Screen>>, f64rgba: [f64; 4]) {
        let m  = Arc::clone(&screen);
        let mut s = m.lock().unwrap();

        s.f64rgba[0] = f64rgba[0];
        s.f64rgba[1] = f64rgba[1] * f64rgba[0];
        s.f64rgba[2] = f64rgba[2] * f64rgba[0];
        s.f64rgba[3] = f64rgba[3] * f64rgba[0];
        s.rgba = as_u8(&s.f64rgba);

        std::mem::drop(s);
    }

    pub fn set_bg(screen: Arc<Mutex<Screen>>, f64rgba: [f64; 4]) {
        let m  = Arc::clone(&screen);
        let mut s = m.lock().unwrap();
        let rgba = as_u8(&f64rgba);
        s.buffer.fill(as_u32_be(&rgba));
    }

    pub fn send_buf(screen: Arc<Mutex<Screen>>, xys: ndarray::Array2<f64>) {
        let m  = Arc::clone(&screen);
        let mut s = m.lock().unwrap();
        for row in xys.rows().into_iter() {
            if !(row[0] > 1.0 || row[0] < 0.0 || row[1] > 1.0 || row[1] < 0.0) {
                let xf64 = row[0];
                let yf64 = row[1];
                
                let mut x = (s.width as f64 * xf64) as usize;
                if x >= s.width {
                    x = s.width-1;
                }
                let mut y = (s.height as f64 * yf64) as usize;
                if y >= s.height {
                    y = s.height-1;
                }
    
                let mut current = as_f64(&u32_to_u8(s.buffer[x * s.width + y]));
    
                let invaa = 1.0 - s.f64rgba[0];
                current[0] = s.f64rgba[0] + current[0] * invaa;
                current[1] = s.f64rgba[1] + current[1] * invaa;
                current[2] = s.f64rgba[2] + current[2] * invaa;
                current[3] = s.f64rgba[3] + current[3] * invaa;
    
                let w = s.width;
                s.buffer[x * w + y] = as_u32_be(&as_u8(&current));
            }
        }

        std::mem::drop(s);
    }

    pub fn renderWithTimeout(screen: Arc<Mutex<Screen>>) {     
        let m  = Arc::clone(&screen);
        let s = m.lock().unwrap();
        let mut window = Window::new(
            "Test - ESC to exit",
            s.width,
            s.height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        std::mem::drop(s);

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        while window.is_open() && !window.is_key_down(Key::Escape) {
            {
                let s = m.lock().unwrap();                
                window
                    .update_with_buffer(&s.buffer, s.width, s.height)
                    .unwrap();

            }
        }
    }

    pub fn render(screen: Arc<Mutex<Screen>>) {     
        let m  = Arc::clone(&screen);
        let s = m.lock().unwrap();
        let mut window = Window::new(
            "Test - ESC to exit",
            s.width,
            s.height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        std::mem::drop(s);

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        while window.is_open() && !window.is_key_down(Key::Escape) {
            {
                let s = m.lock().unwrap();                
                window
                    .update_with_buffer(&s.buffer, s.width, s.height)
                    .unwrap();

            }
            sleep(Duration::from_millis(500));
        }
    }

    impl Screen {
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
