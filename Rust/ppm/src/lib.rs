use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::ptr::null;
use std::ffi::{OsString, OsStr};
use std::fs::OpenOptions;
use std::io::prelude::*;

/// Function called outside (Not finished)
#[no_mangle]
pub extern "C" fn dummy() -> i32 {42}

/// Pixel structure with RGB colors
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

/// Implementation for Pixel
impl Pixel {
    /// Creates a new Pixel
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        return Pixel { r: r, g: g, b: b };
    }
    /// Return the red value of the pixel
    pub fn red(&self) -> u8 {
        self.r
    }
    /// Return the green value of the pixel
    pub fn green(&self) -> u8 {
        self.g
    }
    /// Return the blue value of the pixel
    pub fn blue(&self) -> u8 {
        self.b
    }
    /// Displays the pixel value
    pub fn display(&self) {
        println!("({}, {}, {})", self.r, self.g, self.b);
    }
    /// Inverts the pixels value
    pub fn invert(&mut self) {
        self.r = self.r ^ 255;
        self.g = self.g ^ 255;
        self.b = self.b ^ 255;
    }
    /// Compares two pixels
    pub fn eq(&self, other: &Pixel) -> bool {
        if self.r == other.r && self.g == other.g && self.b == other.b
        {
            return true;
        }
        return false;
    }
    /// Do a grayscalling on the pixel
    pub fn grayscale(&mut self) {
        let average = self.r / 3 + self.g / 3 + self.b / 3;
        self.r = average;
        self.g = average;
        self.b = average;
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Pixel) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}
/// Image structure
pub struct Image {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
    id: String,
    color_depth: String,
}

impl Image {
    /// Calls the get file info function
    pub fn new_with_file() -> Image {
        return Image::get_file_info(0, Path::new("/home/yassine/Downloads/testFile.ppm"));
    }
    /// Creates a new ppm file with Image specs
    pub fn write_new_file(&mut self)
    {
        let mut file = File::create("/home/yassine/Downloads/myFile.ppm").expect("Unable to create file");
        let mut filePixels = "".to_string();
        file.write_all("P3".as_bytes());
        file.write_all("\n".as_bytes());
        file.write_all(format!("{} {}{}", &self.height, &self.width, "\n").as_bytes());
        file.write_all(&self.color_depth.as_bytes());
        file.write_all("\n".as_bytes());
        for x in 0..self.pixels.len()
        {
            filePixels = format!(" {} {} {}", &self.pixels[x].red().to_string(), &self.pixels[x].green().to_string(), &self.pixels[x].blue().to_string());
            file.write_all(filePixels.as_bytes());
            //println!("{}", filePixels);
        }
    }

    /// Calls the pixel.invert function on all the Pixel vector
    pub fn invert_img(&mut self)
    {
        for x in 0..self.pixels.len()
        {
            self.pixels[x].invert();
        }
    }
    /// Calls the pixel.greyscale function on all the Pixel vector
    pub fn greyscale_img(&mut self)
    {
        for x in 0..self.pixels.len()
        {
            self.pixels[x].grayscale();
        }
    }
    /// Creates a new image
    pub fn new(pixels: Vec<Pixel>, width: usize, height: usize, id: String, color_depth: String) -> Image
    {
        return Image {pixels : pixels, width : width, height: height, id: id, color_depth: color_depth};
    }
    /// Returns the image height
    pub fn height(&self) -> usize
    {
        return self.height;
    }
    /// Returns the image width
    pub fn width(&self) -> usize
    {
        return self.width;
    }
    /// Get the image information and returns a new image
    pub fn get_file_info(skip: usize, filename: &Path) -> Image {
        if let Ok(file) = File::open(filename) {
            let mut buffer = BufReader::new(file);
            let (width, height, pixel_list, id, color_depth) = Image::read_file(&mut buffer).expect("");
            return Image::new(pixel_list, width, height, id, color_depth);
        }
        return Image::new(Vec::new(), 0, 0, String::new(), String::new())
    }
    /// Reads the input file
    fn read_file(reader: &mut BufReader<File>) -> Result<(usize, usize, Vec<Pixel>, String, String), Box<dyn Error>> {
        let mut string_buffer = String::new();
        let mut lastLine: Vec<u8> = Vec::with_capacity(300000);
        /// Get the 3 first lines
        for _i in 0..3 {
            reader.read_line(&mut string_buffer).unwrap();
        }
        /// Get the pixel information line
        reader.read_until(b'\n',&mut lastLine).unwrap();
        let ppm_id = string_buffer.lines().nth(0usize).unwrap();

        let image_size = string_buffer
            .lines()
            .nth(1usize)
            .unwrap()
            .to_string()
            .clone();
        let (width, height) = Image::extract_image_size(image_size);

        let color_depth = string_buffer
            .lines()
            .nth(2usize)
            .unwrap()
            .to_string()
            .clone();

        let mut pixel_list : Vec<Pixel> = Vec::with_capacity(width * height);
        let mut r : u8 = 0;
        let mut rbool : bool = false;
        let mut g : u8 = 0;
        let mut gbool : bool = false;
        let mut b : u8 = 0;
        let mut bbool : bool = false;
        for x in lastLine {
            if rbool == false
            {
                r = x;
                rbool = true;
            }
            else if gbool == false
            {
                g = x;
                gbool = true;
            }
            else if bbool == false
            {
                b = x;
                bbool = true;
            }
            else {
                //println!("{} {} {}", x, g, b);
                pixel_list.push(Pixel::new(x, g, b));
                rbool = false;
                gbool = false;
                bbool = false;
            }
        }
        Ok((width, height, pixel_list, ppm_id.to_string(), color_depth))
    }
    /// Get image size caracs
    fn extract_image_size(size: String) -> (usize, usize) {
        let image_size: Vec<String> = size
            .split_whitespace()
            .into_iter()
            .map(|w| w.to_string())
            .collect();
        let width = image_size
            .first()
            .unwrap()
            .parse::<usize>()
            .expect("Invalid format");
        let height = image_size
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("Invalid format");
        (width, height)
    }
}
