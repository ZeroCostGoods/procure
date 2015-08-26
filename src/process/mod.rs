use std::fs;
use std::iter::Iterator;

pub struct Pids {
    iter: Box<Iterator<Item=i32>>,
}

impl Iterator for Pids {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.iter.next()
    }
}

fn pids_from_path(proc_path: &str) -> Pids {
    let iter = fs::read_dir(proc_path).unwrap()
        // Process directories might have gone away since
        // the directory was read. It's fine to ignore those.
        .filter_map(|entry| entry.ok())
        // Map entry to a string, removing it if it fails to
        // parse as unicode.
        .filter_map(|entry| entry.file_name().into_string().ok())
        // Remove any entries that can't be converted to an integer.
        .filter_map(|entry| entry.parse::<i32>().ok());
    Pids{iter: Box::new(iter)}
}

pub fn pids() -> Pids {
    pids_from_path("/proc")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pids_from_path() {
        let mut pids: Vec<i32> = super::pids_from_path("testdata/proc").collect();
        pids.sort();
        assert_eq!(pids, vec![1, 16018, 24064, 24126]);
    }
}
