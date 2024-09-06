use image::{open, DynamicImage, imageops::FilterType, GenericImageView};

fn resize_image(path: &str, width: u32, height: u32) -> DynamicImage {
    let img = open(path).expect("Couldn't open");
    img.resize(width, height, FilterType::Lanczos3)
}

fn resize_image_maintain_ratio(path: &str, width: Option<u32>, height: Option<u32>) -> DynamicImage {
    let img = open(path).expect("Couldn't open");
    let (img_width, img_height) = img.dimensions();

    let ratio = img_width as f32 / img_height as f32;
    let (new_width, new_height) = match (width, height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => (width, (width as f32 / ratio).round() as u32),
        (None, Some(height)) => ((height as f32 * ratio).round() as u32, height),
        (None, None) => (img_width, img_height),
    };
    img.resize(new_width, new_height, FilterType::Lanczos3)
}

fn save_image(img: &DynamicImage, output_path: &str) {
    img.save_with_format(output_path, image::ImageFormat::Png).expect("Couldn't save image");
    println!("Image saved to {}", output_path);
}

fn rotate_image(path: &str, degrees: u32) -> DynamicImage {
    let img = open(path).expect("Couldn't open");
    match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => panic!("Invalid rotation angle"),
    }
}

fn main() {
    println!("Hello, world!");
    let img = open("/Users/wmalachy/Downloads/wallpaperflare.com_wallpaper.jpg").expect("Couldn't open");

    let resized_img = resize_image("/Users/wmalachy/Downloads/wallpaperflare.com_wallpaper.jpg", 1920, 1080);
    save_image(&resized_img, "resized_wallpaper.png");
}
