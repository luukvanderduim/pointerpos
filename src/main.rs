use std::time::Duration;
use xcb;

fn main() {
    let zzz: Duration = std::time::Duration::from_millis(20);

    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let root_id = screen.root();

    // let pointer_coords = xcb::xproto::Point::new(x: 0, y: 0);

    loop {
        let pointercookie = xcb::xproto::query_pointer(&conn, root_id);
        std::thread::sleep(zzz);

        match pointercookie.get_reply() {
            Ok(r) => {
                print!("Coordinates: {},{}  \r", r.root_x(), r.root_y());
            }
            Err(_) => {
                panic!("could not get coordinates of pointer");
            }
        };
    }
}
