use std::net::{TcpListener, TcpStream};

pub fn init(port: &String) ->Result<TcpListener, String>{
	let addr = String::from("127.0.0.1:");

	match TcpListener::bind(addr + port) {
		Ok(listener) => Ok(listener), 
		Err(_) => Err(String::from("fail to bind")), 
	}
}

use std::fs::File;

pub struct HttpResponse {
	pub filetype: HttpType, 
	pub fhandler: File, 
	pub socket_handler: TcpStream, 
}

use std::io::{Read, Write};
use std::io::Error;

impl HttpResponse {
	pub fn new(prefix: &String, filename: &str, socket_handler: TcpStream) ->Result<HttpResponse, Error> {
		let filetype = HttpType::from_name(filename);

		let mut fullpath = prefix.clone();
		fullpath.push_str(filename);

		println!("{}", fullpath);
		match File::open(fullpath) {
			Ok(fhandler) => Ok(HttpResponse{
				filetype, 
				fhandler, 
				socket_handler, 
			}), 
			Err(e) => {
				HttpResponse::send_failmsg(socket_handler).unwrap();
				Err(e)
			}, 
		}
	}

	pub fn solve(&mut self) ->Result<(), Error> {
		let filesize = self.fhandler
				.metadata()
				.unwrap()
				.len() as usize;		// use the metadata to get file size 

		// send the HTTP packet header 
		HttpResponse::send_head(&mut self.socket_handler, 
				&self.filetype, filesize)?;

		// send the file 
		let mut sent_size = 0usize;
		let mut buf = [0u8; 1024];

		while sent_size < filesize {
			self.fhandler.read(&mut buf)?;
			sent_size += self.socket_handler.write(&buf)?;
		}

		Ok(()) 
	}

	fn send_head(socket: &mut TcpStream, filetype: &HttpType, filesize: usize) ->Result<(), Error> {
		let status = if let HttpType::ERR = filetype {
			"404 Not Found"
		} else { "200 Ok"};

		socket.write_fmt(format_args!(
			"HTTP/1.1 {}\r\n\
			Content-Type: {}; charset=utf-8\r\n\
			Content-Length: {}\r\n\r\n", 
			status, filetype.as_str(), filesize
		))?;
		socket.flush()?;

		Ok(())
	}

	fn send_failmsg(mut socket: TcpStream) ->Result<(), Error> {
		let fail_html = "\
			<!DOCTYPE html>\r\n\
			<html><head>\r\n\
			<title>Page not found</title>\r\n\
			</head><body>\r\n\
			<h1>Not Found</h1>\r\n\
			<p>The requested URL is not found on this server</p>\r\n\
			</body></html>\r\n\
		";

		let size = fail_html.len();
		HttpResponse::send_head(&mut socket, &HttpType::ERR, size)?;
		socket.write(fail_html.as_bytes())?;
		socket.flush()?;

		Ok(())
	}
}

pub enum HttpType {
	ERR, 
	HTML, 
	JPG, 
	PNG, 
	GIF, 
	CSS, 
}

impl HttpType {
	pub fn from_name(filename: &str) ->HttpType {
		if filename.ends_with(".html") {
			HttpType::HTML 
		}
		else if filename.ends_with(".jpg") {
			HttpType::JPG
		}
		else if filename.ends_with(".png") {
			HttpType::PNG
		}
		else if filename.ends_with(".gif") {
			HttpType::GIF
		}
		else if filename.ends_with(".css") {
			HttpType::CSS 
		}
		else {
			HttpType::ERR 
		}
	}

	pub fn as_str(&self) ->&'static str {
		match self {
			HttpType::ERR => "", 
			HttpType::HTML => "text/html", 
			HttpType::JPG => "image/jpg", 
			HttpType::PNG => "image/png", 
			HttpType::GIF => "image/gif", 
			HttpType::CSS => "text/css", 
		}
	}
}

// Move TcpStream in instead of borrowing it.
// That's because we won't be using the TcpStream obj after 
// we resolve it. 
pub fn resolve_request(prefix: &String, mut incoming: TcpStream) ->Option<HttpResponse>{
	let mut buf = [0; 1024];

	// If an error happens, return None, so that melantha does 
	// nothing about the TCP connection. 
	if let Err(_) = incoming.read(&mut buf) {
		return None;
	}

	let buf = buf;		// now it should not be mutable 
	let mut filename = String::new();

	// search for required filename 
	let mut i: usize = 0;
	while 1020 > i {
		if b'G' == buf[i] && b'E' == buf[i+1] && b'T' == buf[i+2] && b' ' == buf[i+3] {
			i += 4;
			while b' ' != buf[i] {
				filename.push(buf[i] as char);
				i += 1;
			}

			return match HttpResponse::new(
				prefix, 
				&filename, 
				incoming
			) {
				Ok(res) => Some(res), 
				Err(_) => None, // error handled during `new`
			};
		}
		i += 1;
	}

	None
}
