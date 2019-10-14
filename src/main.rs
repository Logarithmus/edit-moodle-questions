
use std::io::Write;
use curl::easy::Easy;
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
    const LOGIN_URL: &str = "https://lms2.bsuir.by/login/index.php";
	const TEST_URL: &[&str] = &[
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2732%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2774%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2775%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2776%2C21753"
		];
	let questions = auth_cookie(TEST_URL[0], "MoodleSession=blre5nckqdudlsidlt5b8c8pp3")?;
	println!("ПДД: {:?}", questions.find("Дарья"));
	let gear_regex = Regex::new("<a title=\"Редактировать\".*?href=\"(.*?)\".*?>").unwrap();
	gear_regex.captures_iter(&questions).map(|s| s[1].to_owned());
	Ok(())
}