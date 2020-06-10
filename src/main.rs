use serialport::prelude::*;
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "ffb", about = "Communicate with Open FFBoard.")]
struct Opt {
    #[structopt(short, long)]
    /// List available serial ports
    list: bool,
    #[structopt(long = "log", parse(from_os_str))]
    log_file: Option<std::path::PathBuf>,
    //    #[structopt(short, long, required_unless = "list")]
    #[structopt(short, long, default_value, required_unless = "list")]
    port: String,
}

struct StmBoard {
    port: Box<serialport::SerialPort>,
}

//fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() {
    let opt = Opt::from_args();
    // println!("{:#?}", opt);
    // let args = Opt::from_args();
    // println!("{:#?}", args);

    if opt.list == true {
        list_ports();
        //     Ok(());
    }

    let exit_code = match connect(&opt.port) {
        Ok(p) => {
            let mut board = StmBoard { port: p };
            1
        }
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    };

    std::process::exit(exit_code);
}

fn connect(port_name: &str) -> Result<Box<dyn serialport::SerialPort>, Box<dyn Error>> {
    let s = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1),
    };

    let port = serialport::open_with_settings(port_name, &s)
        .map_err(|ref e| format!("Port '{}' not available: {}", &port_name, e))?;
    println!("Connected to {}", &port_name);
    Ok(port)
}

fn list_ports() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}
