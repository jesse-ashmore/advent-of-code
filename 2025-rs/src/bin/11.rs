advent_of_code::solution!(11);

struct Device {
    id: usize,
    name: String,
    outputs: Vec<usize>,
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    // Build nodes with usize IDs and connectors
    // Store in array and traverse this way
    // BFS, avoiding seen paths *within current traversal*
    (None, None)
}

fn parse_devices(input: &str) -> Vec<Device> {
    // Collect and sort all devices into map of name -> ID
    // Re-read and create devices with IDs and outputs using map of IDs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (None, None));
    }
}
