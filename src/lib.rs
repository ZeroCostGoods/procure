extern crate sysconf;

use std::result;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::{BufRead,BufReader};
use std::num::ParseIntError;

use sysconf::{sysconf,SysconfVariable};


pub type Result<T> = result::Result<T, ProcureError>;


#[derive(Debug)]
pub enum ProcureError {
    RuntimeError(String),
    IoError(io::Error),
    ParseError(ParseIntError),
}


#[derive(Debug, PartialEq)]
pub struct CpuTimes {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    // Linux >= 2.6.11
    pub steal: u64,
    // Linux >= 2.6.24
    pub guest: u64,
    // Linux >= 2.6.33
    pub guest_nice: u64,
}


impl CpuTimes {

    fn from_line(line: &str) -> Result<CpuTimes> {
        let parts: Vec<_> = line.split_whitespace().skip(1).collect();
        Ok(CpuTimes{
            user: try!(parts[0].parse().map_err(ProcureError::ParseError)),
            nice: try!(parts[1].parse().map_err(ProcureError::ParseError)),
            system: try!(parts[2].parse().map_err(ProcureError::ParseError)),
            idle: try!(parts[3].parse().map_err(ProcureError::ParseError)),
            iowait: try!(parts[4].parse().map_err(ProcureError::ParseError)),
            irq: try!(parts[5].parse().map_err(ProcureError::ParseError)),
            softirq: try!(parts[6].parse().map_err(ProcureError::ParseError)),
            steal: try!(parts.get(7).unwrap_or(&"0").parse().map_err(ProcureError::ParseError)),
            guest: try!(parts.get(8).unwrap_or(&"0").parse().map_err(ProcureError::ParseError)),
            guest_nice: try!(parts.get(9).unwrap_or(&"0").parse().map_err(ProcureError::ParseError)),
        })
    }

    fn get_total_times_from_path(stat_path: &Path) -> Result<CpuTimes> {
        let fh = try!(File::open(stat_path).map_err(ProcureError::IoError));
        let reader = BufReader::new(fh);

        let line = match reader.lines().next() {
            Some(Ok(line)) => line,
            _ => return Err(ProcureError::RuntimeError(
                "Expected cpu line but none found.".into()
            )),
        };

        CpuTimes::from_line(&line)

    }

    pub fn get_total_times() -> Result<CpuTimes> {
        CpuTimes::get_total_times_from_path(Path::new("/proc/stat"))
    }

    fn get_per_cpu_times_from_path(stat_path: &Path) -> Result<Vec<CpuTimes>> {
        let num_cpus = sysconf(SysconfVariable::ScNprocessorsOnln).unwrap_or(0) as usize;
        let fh = try!(File::open(stat_path).map_err(ProcureError::IoError));
        let reader = BufReader::new(fh);
        let mut cpus: Vec<CpuTimes> = Vec::with_capacity(num_cpus);
        let mut lines = reader.lines();
        lines.next();

        for line in lines {
            let line = match line {
                Ok(line) => line,
                _ => return Err(ProcureError::RuntimeError(
                    "Failed to read line.".into()
                )),
            };

            if !line.starts_with("cpu") { break; }
            cpus.push(try!(CpuTimes::from_line(&line)));
        };

        Ok(cpus)
    }

    pub fn get_per_cpu_times() -> Result<Vec<CpuTimes>> {
        CpuTimes::get_per_cpu_times_from_path(Path::new("/proc/stat"))
    }

}


#[test]
fn test_total_times() {
    assert_eq!(
        CpuTimes::get_total_times_from_path(Path::new("testdata/stat-0001")).unwrap(),
        CpuTimes {
            user: 7969864,
            nice: 6735,
            system: 1633028,
            idle: 43336958,
            iowait: 48613,
            irq: 180,
            softirq: 5043,
            steal: 0,
            guest: 0,
            guest_nice: 0,
        }
    );
}


#[test]
fn test_per_cpu_times() {
    assert_eq!(
        CpuTimes::get_per_cpu_times_from_path(Path::new("testdata/stat-0001")).unwrap(),
        vec![
            CpuTimes {
                user: 2036657, nice: 3176, system: 538690, idle: 40502503, iowait: 48123,
                irq: 180, softirq: 4562, steal: 0, guest: 0, guest_nice: 0,
            },
            CpuTimes {
                user: 1895483, nice: 1224, system: 350858, idle: 947119, iowait: 194,
                irq: 0, softirq: 244, steal: 0, guest: 0, guest_nice: 0,
            },
            CpuTimes {
                user: 2129079, nice: 1332, system: 413982, idle: 937158, iowait: 218,
                irq: 0, softirq: 138, steal: 0, guest: 0, guest_nice: 0,
            },
            CpuTimes {
                user: 1908644, nice: 1002, system: 329497, idle: 950176, iowait: 76,
                irq: 0, softirq: 96, steal: 0, guest: 0, guest_nice: 0,
            },
        ]
    );
}
