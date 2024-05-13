use core::panic;
use std::convert::TryInto;

use image::{ImageBuffer, Pixel, RgbImage};

fn new_image() -> RgbImage {
    ImageBuffer::new(512, 512)
}

/* 
 * Returns a 512 x 512 pixel RgbImage of the Mandelbrot set.
 * You may call new_image() above to get an empty image to fill in.
 * See documentation: https://docs.rs/crate/image/0.23.14
 * 
 * Map the number of iterations in a linear fashion to grayscale; the result is the color of each pixel.
 * Use x in [-2.5, 1] and y in [-1, 1].
 * We encourage you to translate the pseudocode at https://en.wikipedia.org/wiki/Mandelbrot_set.
 * The pseudocode is replicated below in case someone edits Wikipedia during the project.
 * 
 * for each pixel (Px, Py) on the screen do
 *   x0 := scaled x coordinate of pixel (scaled to lie in the Mandelbrot X scale (-2.5, 1))
 *   y0 := scaled y coordinate of pixel (scaled to lie in the Mandelbrot Y scale (-1, 1))
 *   x := 0.0
 *   y := 0.0
 *   iteration := 0
 *   max_iteration := 1000
 *   while (x*x + y*y ≤ 2*2 AND iteration < max_iteration) do
 *       xtemp := x*x - y*y + x0
 *       y := 2*x*y + y0
 *       x := xtemp
 *       iteration := iteration + 1
 *   
 *   color := palette[iteration]
 *   plot(Px, Py, color)
 * 
 */
pub fn make_mandelbrot_image() -> RgbImage {
    let mut image = new_image();
    let (width, height) = image.dimensions();

    let max_iter  = 1000i32;

    for i in 0..width {
        for j in 0..height {
            let x0 = scale_x(i as f64);
            let y0 = scale_y(j as f64);

            let (mut x,mut y, mut iter) = (0f64, 0f64, 0i32);

            println!(" before {} {}", x*x, y*y);
            while (x*x + y*y) <= 4.0f64 && iter < max_iter {
                let xtemp = x*x - y*y + x0;
                y = 2.0*x*y + y0;
                x = xtemp;
                iter +=1;
            }
            println!(" after {} {}", x*x, y*y);
            println!("{iter}");
            image.put_pixel(i, j, image::Rgb([iter as u8, iter as u8, iter as u8]));
            return image;

        } 
    }
    image
}

fn scale_x(coord : f64) -> f64{
    // original range is from 0 to 512
    // map to -2.5 to 1 
    coord * 3.5 / 512.0 - 2.5

}

fn scale_y (coord: f64) -> f64 {
    coord * 2.0 / 512.0 - 1.0
}