use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
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

    println!("X-axis: {}", x_value.trim());
    println!("Y-axis: {}", y_value.trim());
    println!("Z-axis: {}", z_value.trim());

    Ok(())
}
