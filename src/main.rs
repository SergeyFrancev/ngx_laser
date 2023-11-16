#[macro_use]
extern crate cli_log;
fn main() -> Result<(), ngx_laser::NgxLaserError> {
    init_cli_log!("ngx-laser");
    debug!("hello");
    let _ = ngx_laser::run()?;
    info!("bye");
    Ok(())
}
