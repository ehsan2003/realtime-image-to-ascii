use image::{
    imageops::{grayscale, resize, FilterType},
    GrayImage, ImageBuffer, RgbImage,
};
use rscam::{self, Config};
use termsize::Size;

fn main() {
    let mut chars = [' ', '.', ',', ':', ';', '+', '*', '?', '%', 'S', '#', '@'];
    chars.reverse();
    let path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("/dev/video0"));

    let mut cam = rscam::Camera::new(&path).unwrap();
    let resolution = match cam.resolutions(b"RGB3").unwrap() {
        rscam::ResolutionInfo::Discretes(d) => d[0],
        _ => panic!("invalid "),
    };
    cam.start(&Config {
        interval: (1, 30),
        resolution: (resolution.0, resolution.1),
        format: b"RGB3",
        ..Default::default()
    })
    .unwrap();
    loop {
        let Size { rows, cols } = termsize::get().unwrap();

        let frame = cam.capture().unwrap();

        let img: RgbImage =
            ImageBuffer::from_raw(frame.resolution.0, frame.resolution.1, frame.to_vec()).unwrap();

        let grayscaled = grayscale(&img);

        let resized = resize(&grayscaled, cols as u32, rows as u32, FilterType::Nearest);

        let pixels = image_to_ascii(resized, &chars);

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("{}", pixels);
    }
}

fn image_to_ascii(img: GrayImage, chars: &[char]) -> String {
    let buckets = 255u8 / (chars.len() as u8 - 1);
    img.rows()
        .map(|r| {
            r.map(|p| p.0[0])
                .map(|v| v / buckets)
                .map(|v| chars[v as usize])
        })
        .map(|r| r.collect::<String>())
        .fold(String::new(), |a, b| a + &b + "\n")
}
