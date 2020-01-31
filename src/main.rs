use docopt::Docopt;
use serde::Deserialize;

static USAGE: &str = "
codesender

Usage:
    codesender <code> [options]

Options:
    -p <pin>     The GPIO Pin to send the code on (default 17)
    -l <length>  The length of the pulse (default 187)
";
#[derive(Deserialize)]
struct Args {
    pub arg_code: usize,
    pub flag_pin: Option<u8>,
    pub flag_length: Option<u64>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.deserialize())
                        .unwrap_or_else(|e| e.exit());

    let code = args.arg_code;
    let pin = args.flag_pin.unwrap_or(17);
    let len = args.flag_length.unwrap_or(187);
    for _ in 0..10 {
        codesender::send(code, pin, len).expect("Failed to send code");
    }
}