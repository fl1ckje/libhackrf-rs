use clap::{arg, command, ArgMatches};
use std::ops::RangeInclusive;

const FS_RANGE: RangeInclusive<usize> = 2_000_000..=20_000_000;
const FC_RANGE: RangeInclusive<usize> = 1_000_000..=6_000_000_000;
const FBW_RANGE: RangeInclusive<usize> = 1_750_000..=28_000_000;
const LNA_RANGE: RangeInclusive<usize> = 0..=40;
const VGA_RANGE: RangeInclusive<usize> = 0..=62;

#[derive(Debug)]
pub struct Args {
    pub fs: u32,
    pub fc: u64,
    pub fbw: u32,
    pub lna: u16,
    pub vga: u16,
    pub amp: bool,
    pub bias_tee: u8,
    pub file_name: String,
}

pub fn parse() -> Args {
    let matches: ArgMatches = command!() // requires `cargo` feature
        .arg(
            arg!(--fs <Hz> "sample rate in range [2_000_000; 20_000_000] Hz")
                .required(true)
                .value_parser(fs_in_range),
        )
        .arg(
            arg!(--fc <Hz> "carrier frequency in range [1_000_000; 6_000_000_000] Hz")
                .required(true)
                .value_parser(fc_in_range),
        )
        .arg(
            arg!(--fbw <Hz> "baseband filter bandwidth in range [1_750_000; 28_000_000] Hz")
                .required(true)
                .value_parser(fbw_in_range),
        )
        .arg(
            arg!(--lna <dB> "lna (if) gain in range [0;40] dB with step 8")
                .required(true)
                .value_parser(lna_in_range),
        )
        .arg(
            arg!(--vga <dB> "vga (baseband) gain in range [0; 62] dB with step 2")
                .required(true)
                .value_parser(vga_in_range),
        )
        .arg(
            arg!(--amp <int> "enable amplifier [0;1]")
                .required(true)
                .value_parser(amp_in_range),
        )
        .arg(
            arg!(--bias_tee <int> "enable bias tee (antenna power) [0;1]")
                .required(true)
                .value_parser(bias_in_range),
        )
        .arg(arg!(--file_name <string> "output file name").required(true))
        .get_matches();

    let args: Args = Args {
        fs: *matches.get_one::<u32>("fs").unwrap(),
        fc: *matches.get_one::<u64>("fc").unwrap(),
        fbw: *matches.get_one::<u32>("fbw").unwrap(),
        lna: *matches.get_one::<u16>("lna").unwrap(),
        vga: *matches.get_one::<u16>("vga").unwrap(),
        amp: *matches.get_one::<bool>("amp").unwrap(),
        bias_tee: *matches.get_one::<u8>("bias_tee").unwrap(),
        file_name: (*matches.get_one::<String>("file_name").unwrap()).to_string(),
    };
    println!("{:#?}", args);

    args
}

fn parse_val(s: &str) -> Result<usize, String> {
    s.parse().map_err(|_| format!("`{s}` isn't a number"))
}

fn fs_in_range(s: &str) -> Result<u32, String> {
    let fs: usize = parse_val(s)?;
    if FS_RANGE.contains(&fs) {
        Ok(fs as u32)
    } else {
        Err(format!(
            "fs is not in range [{};{}]",
            FS_RANGE.start(),
            FS_RANGE.end()
        ))
    }
}

fn fc_in_range(s: &str) -> Result<u64, String> {
    let fc: usize = parse_val(s)?;
    if FC_RANGE.contains(&fc) {
        Ok(fc as u64)
    } else {
        Err(format!(
            "fc is not in range [{};{}]",
            FC_RANGE.start(),
            FC_RANGE.end()
        ))
    }
}

fn fbw_in_range(s: &str) -> Result<u32, String> {
    let fbw: usize = parse_val(s)?;
    if FBW_RANGE.contains(&fbw) {
        Ok(fbw as u32)
    } else {
        Err(format!(
            "fbw is not in range [{};{}]",
            FBW_RANGE.start(),
            FBW_RANGE.end()
        ))
    }
}

fn lna_in_range(s: &str) -> Result<u16, String> {
    let lna: usize = parse_val(s)?;
    if LNA_RANGE.contains(&lna) {
        if lna % 8 != 0 {
            Err("lna is not a multiple of 8".to_owned())
        } else {
            Ok(lna as u16)
        }
    } else {
        Err(format!(
            "lna is not in range [{};{}]",
            LNA_RANGE.start(),
            LNA_RANGE.end()
        ))
    }
}

fn vga_in_range(s: &str) -> Result<u16, String> {
    let vga: usize = parse_val(s)?;
    if VGA_RANGE.contains(&vga) {
        if vga % 2 != 0 {
            Err("lna is not a multiple of 2".to_owned())
        } else {
            Ok(vga as u16)
        }
    } else {
        Err(format!(
            "vga is not in range [{};{}]",
            VGA_RANGE.start(),
            VGA_RANGE.end()
        ))
    }
}

fn amp_in_range(s: &str) -> Result<bool, String> {
    let amp: usize = parse_val(s)?;
    match amp {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err("amp is not in range [0;1]".to_owned()),
    }
}

fn bias_in_range(s: &str) -> Result<u8, String> {
    let bias: usize = parse_val(s)?;

    match bias {
        0 => Ok(0),
        1 => Ok(1),
        _ => Err("amp is not in range [0;1]".to_owned()),
    }
}
