use ngx_laser::DateTime;
use glassbench::*;

fn bench_parse_lines(bench: &mut Bench) {
    let dt = Vec::from([
        "02/Feb/2020:03:59:00 +0500",
        "15/May/2022:11:00:01 +0300",
        "01/Dec/2021:23:28:00 +0300",
        "22/Jun/2023:03:22:51 +0300",
        "23/Jun/2023:03:18:29 +0300",
        "23/Jun/2023:03:18:37 +0300",
    ]);

    bench.task("parse datetime", |task| {
        let mut idx = 0;
        task.iter(|| {
            let _ = DateTime::from_nginx(&dt[idx]).unwrap();
            idx += 1;
            if idx >= dt.len() {
                idx = 0;
            }
        });
    });

    // bench.task("parse chrono dt", |task| {
    //     let mut idx = 0;
    //     task.iter(|| {
    //         let d = chrono::DateTime::parse_from_str(&dt[idx], "%d/%b/%Y:%H:%M:%S %z").unwrap();
    //         idx += 1;
    //         if idx >= dt.len() {
    //             idx = 0;
    //         }
    //     });
    // });
}

glassbench!("Parse log lines", bench_parse_lines,);
