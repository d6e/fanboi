use std::fs::{File, read_to_string};
use std::{thread, time};
use std::error::Error;
use std::io::prelude::*;
use clap::{Arg, App, value_t};
use pid::Pid;

type Temp = f32;
type Pwm = u16;

struct Config {
    dry_run: bool,
    verbose: bool,
    p_value: f32,
    i_value: f32,
    d_value: f32,
    target_temp: Temp,
    cpu_temperature_ctl: String,
    gpu_temperature_ctl: String,
    fan_pwm_ctl: String,
    poll_interval_secs: u64,
    minimum_pwm: Pwm,
}

impl Config {
    fn write_pwm(&self, pwm: Pwm) {
        if !self.dry_run {
            if self.verbose {
                println!("Writing file={} pwm={}", &self.fan_pwm_ctl, pwm);
            }
            let mut f = File::create(&self.fan_pwm_ctl).unwrap();
            f.write_all(pwm.to_string().as_bytes()).unwrap();
            f.sync_data().unwrap();
        } else {
            println!("Dry run enabled. Avoiding writes.")
        }
    }

    #[allow(dead_code)]
    fn read_pwm(&self) -> Pwm {
        if self.verbose {
            println!("Reading file={}", &self.fan_pwm_ctl);
        }
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
        let mut max: Temp = 0.0;
        for thermal_ctl in [&self.cpu_temperature_ctl, &self.gpu_temperature_ctl].iter() {
            if self.verbose {
                println!("Reading file={}", thermal_ctl);
            }
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

fn get_program_input() -> Config {
    let default_p_value = 1.0;
    let default_i_value = 0.7;
    let default_d_value = 4.0;
    let default_target_temp = 40.0;
    let default_fan_pwm_ctl = String::from("/sys/class/hwmon/hwmon0/pwm1");
    let default_cpu_temperature_ctl = String::from("/sys/class/thermal/thermal_zone0/temp");
    let default_gpu_temperature_ctl = String::from("/sys/class/thermal/thermal_zone1/temp");
    let default_poll_interval_secs = 10;
    let default_minimum_pwm = 50;
    let name_verbose = "verbose";
    let name_dry_run = "dry-run";
    let name_p_value = "pvalue";
    let name_i_value = "ivalue";
    let name_d_value = "dvalue";
    let name_target_temp = "target-temp";
    let name_cpu_temperature_ctl = "cpu-temp-ctl";
    let name_gpu_temperature_ctl = "gpu-temp-ctl";
    let name_fan_pwm_ctl = "fan-pwm-ctl";
    let name_poll_interval_secs = "poll-interval-secs";
    let name_minimum_pwm = "minimum-pwm";
    let matches = App::new("fanboi")
        .version("1.0")
        .author("Danielle <fanboi@d6e.io>")
        .about("Fanboi - A fan PID controller")
        .arg(Arg::with_name(name_verbose)
            .short("v")
            .long(name_verbose)
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name(name_dry_run)
            .long("dry-run")
            .help("Set program the dry run."))
        .arg(Arg::with_name(name_p_value)
            .short("p")
            .long("pvalue")
            .value_name("P")
            .help(&format!("Sets the P gain value of the PID controller. Default={}°C", default_p_value))
            .takes_value(true))
        .arg(Arg::with_name(name_i_value)
            .short("i")
            .long("ivalue")
            .value_name("I")
            .help(&format!("Sets the I gain value of the PID controller. Default={}°C", default_i_value))
            .takes_value(true))
        .arg(Arg::with_name(name_d_value)
            .short("d")
            .long("dvalue")
            .value_name("D")
            .help(&format!("Sets the D gain value of the PID controller. Default={}°C", default_d_value))
            .takes_value(true))
        .arg(Arg::with_name(name_target_temp)
            .short("t")
            .long(name_target_temp)
            .value_name("VALUE")
            .help(&format!("Target temperature. Default={}°C", default_target_temp))
            .takes_value(true))
        .arg(Arg::with_name(name_cpu_temperature_ctl)
            .short("c")
            .long(name_cpu_temperature_ctl)
            .value_name("CPU_TEMP_FILE")
            .help(&format!("The CPU temperature file. Default='{}'", default_cpu_temperature_ctl))
            .takes_value(true))
        .arg(Arg::with_name(name_gpu_temperature_ctl)
            .short("g")
            .long(name_gpu_temperature_ctl)
            .value_name("CPU_TEMP_FILE")
            .help(&format!("The GPU temperature file. Default='{}'", default_gpu_temperature_ctl))
            .takes_value(true))
        .arg(Arg::with_name(name_fan_pwm_ctl)
            .short("f")
            .long(name_fan_pwm_ctl)
            .value_name("FAN_FILE")
            .help(&format!("Fan control file. Default='{}'", default_fan_pwm_ctl))
            .takes_value(true))
        .arg(Arg::with_name(name_poll_interval_secs)
            .long(name_poll_interval_secs)
            .value_name("SECONDS")
            .help(&format!("The frequency at which to poll the temperature and update fan pwm. Default='{}'", default_poll_interval_secs))
            .takes_value(true))
        .arg(Arg::with_name(name_minimum_pwm)
            .long(name_minimum_pwm)
            .value_name("PWM")
            .help(&format!("The minimum control output pwm before activating the fan. Default='{}'", default_minimum_pwm))
            .takes_value(true))
        .get_matches();
    Config {
        dry_run: matches.is_present(name_dry_run),
        verbose: matches.is_present(name_verbose),
        p_value: value_t!(matches, name_p_value, f32).unwrap_or(default_p_value),
        i_value: value_t!(matches, name_i_value, f32).unwrap_or(default_i_value),
        d_value: value_t!(matches, name_d_value, f32).unwrap_or(default_d_value),
        target_temp: value_t!(matches, name_target_temp, Temp).unwrap_or(default_target_temp),
        cpu_temperature_ctl: value_t!(matches, name_cpu_temperature_ctl, String).unwrap_or(default_cpu_temperature_ctl).to_string(),
        gpu_temperature_ctl: value_t!(matches, name_gpu_temperature_ctl, String).unwrap_or(default_gpu_temperature_ctl).to_string(),
        fan_pwm_ctl: value_t!(matches, name_fan_pwm_ctl, String).unwrap_or(default_fan_pwm_ctl).to_string(),
        poll_interval_secs: value_t!(matches, name_poll_interval_secs, u64).unwrap_or(default_poll_interval_secs),
        minimum_pwm: value_t!(matches, name_minimum_pwm, Pwm).unwrap_or(default_minimum_pwm),
    }
}

#[test]
fn test_to_pwm() {
    assert_eq!(to_pwm(-20.0), 0);
    assert_eq!(to_pwm(0.0), 0);
    assert_eq!(to_pwm(20.0), 20);
}

fn to_pwm(x: f32) -> Pwm {
    // Round negative pwm values to zero since the fans can't go backwards.
    if x < 0.0 {0} else {x as Pwm}
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_program_input();

    println!("Starting PID fan controller...");

    // The values for the pid can be tuned to best match the temperature
    println!("PID initialized with p={} i={} d={} target_temp={}", config.p_value, config.i_value, config.d_value, config.target_temp);
    let mut pid = Pid::new(config.p_value, config.i_value, config.d_value, 100.0, 100.0, 100.0, config.target_temp);
    loop {
        let temp = config.get_thermal();
        let output = pid.next_control_output(temp);
        // Invert fan speed because the fan speed is inversely related to temperature.
        let inverted_output = (-1.0 * output.output).ceil();
        let new_pwm: Pwm = to_pwm(inverted_output);
        if config.verbose {
            println!("temp={} new_pwm={}", temp, new_pwm);
        }

        // The fan struggles to start at low pwm values. Only start the fan if the new pwm
        // value is high enough to actually start the fan.
        if new_pwm > config.minimum_pwm || new_pwm == 0 {
            config.write_pwm(new_pwm);
        }
        thread::sleep(time::Duration::from_secs(config.poll_interval_secs));
    }
}
