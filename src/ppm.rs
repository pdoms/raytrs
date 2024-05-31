use std::fs::OpenOptions;
use std::io::Write;

pub fn write_ppm(file: &str, buf: &[&[u8]], width: usize, height: usize, max: usize) {
    let mut buffer = String::new();
    buffer.push_str(format!("P3\n{width} {height}\n{max}\n").as_str());
    for y in 0..height {
        for x in 0..width {
            let i = (y*width) + x;
            buffer.push_str(format!("{} {} {}\n", buf[i][0], buf[i][1], buf[i][2]).as_str());
        }
    }
    let mut fd = OpenOptions::new().create(true).write(true).truncate(true).open(file).expect("Could not open file '{file}'");
    let bytes = buffer.into_bytes();
    fd.write_all(&bytes).expect("Could not write to '{file}'");
}

#[cfg(test)]
mod test {
    use super::write_ppm;

    #[test]
    fn ppm() {
        let width = 3;
        let height = 2;
        let max = 255;
        let data: Vec<&[u8]> = vec![
            &[255,0,0],
            &[0,255,0],
            &[0,0,255],
            &[255,255,0],
            &[255,255,255],
            &[0,0,0]
        ];
        write_ppm("data/test_out.ppm", data.as_slice(), width, height, max);
    }
}
