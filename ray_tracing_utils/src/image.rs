use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel { r: 0x00, g: 0x00, b: 0x00, a: 0xff }
    }
}

#[derive(Default, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let size = width as usize * height as usize;
        let data = vec![Pixel::default(); size];
        Image { width, height, data }
    }

    pub fn from_path(filepath: &str) -> Self {
        let f = File::open(filepath)
            .unwrap_or_else(|_| panic!("Couldn't open the file: {}", filepath));
        let mut data: Vec<Pixel> = Vec::new();

        let mut width: u32 = 0;
        let mut height: u32 = 0;
        for (k, result) in BufReader::new(f).lines().enumerate() {
            let l = result.unwrap_or_else(|_| panic!("something went wrong reading the file"));
            if l == "P3" {
                println!("P3 is detected");
            } else if k == 1 {
                let d = l.split(" ").collect::<Vec<&str>>();
                width = d[0].parse::<u32>().unwrap();
                height = d[1].parse::<u32>().unwrap();
            } else if k >= 3 {
                let d = l.split(" ").collect::<Vec<&str>>();
                let r = d[0].parse::<u8>().unwrap();
                let g = d[1].parse::<u8>().unwrap();
                let b = d[2].parse::<u8>().unwrap();
                let a = 0xff;
                data.push(Pixel { r, g, b, a });
            }
        }

        Image { width, height, data }
    }

    pub fn get_pixel(&self, idx: usize) -> Pixel {
        return self.data[idx];
    }
}
