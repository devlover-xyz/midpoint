use image::{ImageBuffer, Rgb};

extern crate image;

fn main() {
    // Define the parameters of the circle
    let center_x = 200;
    let center_y = 200;
    let radius = 100;

    // Create a blank image with a white background
    let width = 400;
    let height = 400;
    let mut img = ImageBuffer::new(width, height);

    // Calculate the initial point
    let mut x = radius;
    let mut y = 0;
    let mut d = 1 - radius as i32; // Initial decision parameter

    // Plot the initial points (8-way symmetry)
    plot_points(&mut img, center_x, center_y, x, y);

    // Midpoint algorithm
    while x > y {
        y += 1;
        if d <= 0 {
            d = d + 2 * y as i32 + 1;
        } else {
            x -= 1;
            d = d + 2 * (y as i32 - x as i32) + 1;
        }
        plot_points(&mut img, center_x, center_y, x, y);
    }

    // Save the image to a file
    img.save("circle.png").unwrap();
}

// Function to plot points with 8-way symmetry
fn plot_points(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, xc: u32, yc: u32, x: u32, y: u32) {
    img.put_pixel(xc + x, yc + y, Rgb([0, 0, 0])); // Octant 1
    img.put_pixel(xc - x, yc + y, Rgb([0, 0, 0])); // Octant 4
    img.put_pixel(xc + x, yc - y, Rgb([0, 0, 0])); // Octant 5
    img.put_pixel(xc - x, yc - y, Rgb([0, 0, 0])); // Octant 8
    img.put_pixel(xc + y, yc + x, Rgb([0, 0, 0])); // Octant 2
    img.put_pixel(xc - y, yc + x, Rgb([0, 0, 0])); // Octant 3
    img.put_pixel(xc + y, yc - x, Rgb([0, 0, 0])); // Octant 6
    img.put_pixel(xc - y, yc - x, Rgb([0, 0, 0])); // Octant 7
}
