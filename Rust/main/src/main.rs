extern crate ppm;

fn main() {
    let mut pixel = ppm::Pixel::new(255, 0, 0);
    let mut toComp = ppm::Pixel::new(2, 45, 22);
    println!("{}", pixel.red());
    pixel.invert();
    pixel.display();
    pixel.grayscale();
    pixel.display();
    println!("{}", pixel.eq(&toComp));
    let mut img = ppm::Image::new_with_file();
    img.invert_img();
    img.greyscale_img();
    println!("{} {}", img.height(), img.width());
    img.write_new_file();
}
