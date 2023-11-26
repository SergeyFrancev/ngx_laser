use std::{fs::File, io::BufReader};
use std::io::BufRead;

use {
    glassbench::*,
    ngx_laser::*,
    std::path::PathBuf,
};

fn bench_parse_lines(bench: &mut Bench) {
    bench.task("parse custom format", |task| {
        let parser = Parser::new("$remote_addr - $remote_user [$time_local] \"$request\" $status $body_bytes_sent \"$http_referer\" \"$http_user_agent\"").unwrap();

        let package_dir = std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir not set");
        let path = PathBuf::from(package_dir).join("test_data/access.log");
        let file = File::open(path).unwrap();
        let mut file = BufReader::new(file);
        let mut line = String::new();

        task.iter(|| {
            line.clear();
            if file.read_line(&mut line).unwrap() == 0 {
                return; // EOF
            }
            let log_line = parser.parse(&line).unwrap();
            pretend_used(log_line);
        });
    });
}

glassbench!(
    "Parse log lines",
    bench_parse_lines,
);
