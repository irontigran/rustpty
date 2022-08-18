use nix::pty;
use std::fs::File;
use std::io::Write;
use std::net;
use std::os::unix::io::FromRawFd;
use std::process::{Child, Command, Stdio};

#[derive(Debug)]
pub struct Pty {
    process: Child,
    fd: i32,
}

fn main() {
    unsafe {
        let ForkptyResult { mfd, proc } = pty::forkpty(None, None).expect("forkpty");
    }
    if proc.is_parent() {
        // do parent stuff
        // set up socket
        // relay data between socket and master pty
        let listener = net::TcpListener::bind("127.0.0.1:2000").expect("bind");   
        let (socket, _) = listener.accept().expect("accept");
    } else {
        // exec shell with stdin/out/err hooked up to slave pty
    }


    let ends = openpty(None, None).expect("openpty failed");
    let master = ends.master;
    let slave = ends.slave;

    let mut builder = Command::new(process);
    builder.stdin(unsafe { Stdio::from_raw_fd(slave) });
    builder.stdout(unsafe { Stdio::from_raw_fd(slave) });
    builder.stderr(unsafe { Stdio::from_raw_fd(slave) });

    match builder.spawn() {
        Ok(process) => {
            let pty = Pty {
                process,
                fd: master,
            };

            pty
        }
        Err(e) => {
            panic!("Failed to create pty: {}", e);
        }
    }
    let shell = "/bin/bash";

    let pty = create_pty(shell);
    println!("{:?}", pty);

    let mut output = unsafe { File::from_raw_fd(pty.fd) };
    write!(output, "touch /tmp/itworks\n");
    output.flush();

    std::thread::sleep_ms(1000);

    println!("{}", pty.process.id());
}

