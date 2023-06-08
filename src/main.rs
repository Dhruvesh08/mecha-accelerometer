use std::fs::File;
use std::io::{BufRead, BufReader};

struct Accelerometer {
    x: i32,
    y: i32,
    z: i32,
}

impl Accelerometer {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn read(&mut self) -> std::io::Result<()> {
        let x_file = File::open("/sys/bus/iio/devices/iio:device2/in_accel_x_raw")?;
        let y_file = File::open("/sys/bus/iio/devices/iio:device2/in_accel_y_raw")?;
        let z_file = File::open("/sys/bus/iio/devices/iio:device2/in_accel_z_raw")?;

        let mut x_reader = BufReader::new(x_file);
        let mut y_reader = BufReader::new(y_file);
        let mut z_reader = BufReader::new(z_file);

        let mut x_value = String::new();
        let mut y_value = String::new();
        let mut z_value = String::new();

        x_reader.read_line(&mut x_value)?;
        y_reader.read_line(&mut y_value)?;
        z_reader.read_line(&mut z_value)?;

        self.x = x_value.trim().parse::<i32>().unwrap();
        self.y = y_value.trim().parse::<i32>().unwrap();
        self.z = z_value.trim().parse::<i32>().unwrap();

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut accelerometer = Accelerometer::new();

    accelerometer.read()?;

    println!("X-axis: {}", accelerometer.x);
    println!("Y-axis: {}", accelerometer.y);
    println!("Z-axis: {}", accelerometer.z);

    Ok(())
}
