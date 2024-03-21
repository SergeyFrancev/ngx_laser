use std::{time::Instant, path::PathBuf};

use ngx_laser::{Consumers, Parser, FileFinder, LogFile};

#[macro_use]
extern crate cli_log;

fn main() -> Result<(), ngx_laser::NgxLaserError> {
    init_cli_log!("ngx-laser");
    ngx_laser::run()?;

    Ok(())
}

fn run_parser() {
    let start = Instant::now();
    let format: &str = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
    // let format: &str = r#""$time_local" client=$remote_addr method=$request_method request="$request" request_length=$request_length status=$status bytes_sent=$bytes_sent body_bytes_sent=$body_bytes_sent referer=$http_referer user_agent="$http_user_agent" upstream_addr=$upstream_addr upstream_status=$upstream_status request_time=$request_time upstream_response_time=$upstream_response_time"#;

    let mut consumers = Consumers::new();
    let parser = Parser::new(format).unwrap();

    // let mut ua: HashSet<String> = HashSet::new();
    let paths = [PathBuf::from("/Users/nulldata/Documents/projects/rust/ngx_laser/ngx_laser/test_data/def/")];
    let mut file_finder = FileFinder::new(&paths, &parser);
    let dates = file_finder.dated_files().unwrap();
    dates.into_iter()
        .map(|x| x.1)
        .flat_map(|x| LogFile::new(&x, &parser).into_iter())
        .for_each(|x| {
            consumers.eat_line(x);
        });

    // println!("Count IP: {}", ua.len());
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}