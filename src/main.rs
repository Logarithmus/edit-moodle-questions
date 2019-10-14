use curl::FormError;
use std::io::Write;
use curl::easy::{Easy, Form};
use std::error::Error;
use regex::Regex;

#[macro_use(lazy_static)]
extern crate lazy_static;

fn send_request(easy: &mut Easy) -> Result<String, Box<dyn Error>> {
	let mut response = Vec::<u8>::new();
    {
	    let mut transfer = easy.transfer();
	    transfer.write_function(|html| {
	    	response.write_all(html).map(|_| Ok(html.len())).unwrap()
	    })?;
	    transfer.perform()?;
	}
    let response = String::from_utf8(response)?;
    Ok(response)
}

fn parse_logintoken(login_page: &str) -> Option<&str> {
	lazy_static!{
		static ref LOGINTOKEN: Regex = Regex::new("<.*?logintoken.*?value=\"(.*?)\">").unwrap();}
    LOGINTOKEN.captures(&login_page).and_then(|cap| cap.get(1)).map(|m| m.as_str())
}

fn build_form(logintoken: &str, username: &str, password: &str) -> Result<Form, FormError> {
	let mut form = Form::new();
	form.part("anchor").contents(b"").add()?;
	form.part("logintoken").contents(logintoken.as_bytes()).add()?;
	form.part("username").contents(username.as_bytes()).add()?;
	form.part("password").contents(password.as_bytes()).add()?;
	Ok(form)
}

fn auth_form(auth_url: &str, form: Form) -> Result<String, Box<dyn Error>> {
	let mut easy = Easy::new();
	easy.url(auth_url)?;
	easy.useragent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/77.0.3865.90 Chrome/77.0.3865.90 Safari/537.36")?;
	easy.follow_location(true)?;
	easy.verbose(true)?;
	easy.httppost(form)?;
	easy.post(true)?;
	easy.cookie_jar("cookies.txt")?;
	let _redir_response = send_request(&mut easy)?;
	easy.post(false)?;
	easy.get(true)?;
	easy.url("https://lms2.bsuir.by")?;
	let response = send_request(&mut easy)?;
	Ok(response)
}

fn auth_cookie(auth_url: &str, cookie: &str) -> Result<String, Box<dyn Error>> {
	let mut easy = Easy::new();
	easy.url(auth_url)?;
	easy.useragent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/77.0.3865.90 Chrome/77.0.3865.90 Safari/537.36")?;
	easy.follow_location(true)?;
	//easy.verbose(true)?;
	easy.cookie(cookie)?;
	let response = send_request(&mut easy)?;
	Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
	//let username = &std::env::args().nth(1).expect("username not found");
    //let password = &std::env::args().nth(2).expect("password not found");

    //let (username, password) = ("71380043", "B26forever");
    const LOGIN_URL: &str = "https://lms2.bsuir.by/login/index.php";
	const TEST_URL: &[&str] = &[
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2732%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2774%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2775%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2776%2C21753"
		];
 //    let mut easy = Easy::new();
 //    easy.url(LOGIN_URL)?;
 //    let login_page = send_request(&mut easy)?;
 //    let logintoken = parse_logintoken(&login_page).expect("logintoken not found");
 //    println!("{}", logintoken);
	let after_auth = auth_cookie(TEST_URL[0], "MoodleSession=blre5nckqdudlsidlt5b8c8pp3")?;
	println!("ПДД: {:?}", after_auth.find("Дарья"));
	

	Ok(())
}