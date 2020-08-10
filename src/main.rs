use std::fs::{File, read_to_string};
use std::{thread, time};
use serde::Deserialize;
use std::error::Error;
use std::io::prelude::*;
use toml;
use pid::Pid;

type Temp = i16;
type Pwm = u16;

#[derive(Deserialize)]
struct Config {
    cpu_temperature_ctl: String,
    gpu_temperature_ctl: String,
    fan_pwm_ctl: String,
}

impl Config {
    fn write_pwm(&self, pwm: Pwm, dry_run: bool) {
        println!("setPWM={}", pwm);
        if !dry_run {
            println!("Writing file={}", &self.fan_pwm_ctl);
            let mut f = File::create(&self.fan_pwm_ctl).unwrap();
            f.write_all(pwm.to_string().as_bytes()).unwrap();
            f.sync_data().unwrap();
        }
    }

    fn read_pwm(&self) -> Pwm {
        println!("Reading file={}", &self.fan_pwm_ctl);
        let temp_str: String = read_to_string(&self.fan_pwm_ctl).unwrap();
        let parsed = match temp_str.trim().parse::<u32>() {
            Err(e) => {
                eprintln!("ERROR='{}' string='{}'", e, temp_str);
                Err(e)
            },
            Ok(u) => Ok(u)
        };
        parsed.unwrap() as Pwm
    }

    // Read over the two thermal zones, return the max.
    fn get_thermal(&self) -> Temp {
        let mut max: Temp = 0;
        for thermal_ctl in [&self.cpu_temperature_ctl, &self.gpu_temperature_ctl].iter() {
            println!("Reading file={}", thermal_ctl);
            let temp_str: String = read_to_string(thermal_ctl).unwrap();
            let parsed = match temp_str.trim().parse::<i32>() {
                Err(e) => {
                    eprintln!("ERROR='{}' string='{}'", e, temp_str);
                    Err(e)
                },
                Ok(u) => Ok(u)
            };
            let temp: Temp = (parsed.unwrap() / 1000) as Temp;
            if temp > max {
                max = temp;
            }
        }
        return max;
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let dry_run = false;

    let file = read_to_string("fanboi.toml")?;
    let config: Config = toml::from_str(&file)?;
    drop(file);

    println!("Starting PID fan controller...");
    let target_temp: f32 = 40.0;
    let poll_interval_secs = 10;

    // The values for the pid can be tuned to best match the temperature
    let mut pid = Pid::new(1.0, 0.7, 4.0, 100.0, 100.0, 100.0, target_temp);
    loop {
        let temp = config.get_thermal();
        let output = pid.next_control_output(temp as f32);
        // let pwm: Pwm = output.output as Pwm;
        let inverted_output = -1.0 * output.output;
        let new_pwm = if inverted_output < 0.0 {0.0} else {inverted_output};
        // let pwm: Pwm =  as Pwm; // have to invert because going higher makes it lower
        println!("temp={} pwm={} output={} inverted_output={}", temp, new_pwm, output.output, inverted_output);
        let is_fan_running = config.read_pwm() != 0;
        // The fan struggles to start at low pwm values. Only start the fan if the new pwm
        // value is high enough to actually start the fan.
        if is_fan_running || new_pwm > 50.0 {
            config.write_pwm(new_pwm as Pwm, dry_run);
        }
        thread::sleep(time::Duration::from_secs(poll_interval_secs));
    }
}
