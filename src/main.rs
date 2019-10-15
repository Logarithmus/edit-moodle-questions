
use curl::easy::Form;
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

fn get_question_form(html: &str) -> Result<Form, curl::FormError> {
	lazy_static! {
		static ref NOANSWERS: Regex = Regex::new(r#"(?s)name="noanswers".*?value="(.*?)""#).unwrap();
		static ref ID: Regex = Regex::new(r#"(?s)name="id".*?value="(.*?)""#).unwrap();
		static ref COURSEID: Regex = Regex::new(r#"(?s)name="courseid".*?value="(.*?)""#).unwrap();
		static ref SESSKEY: Regex = Regex::new(r#"(?s)name="sesskey".*?value="(.*?)""#).unwrap();
		static ref USECURRENTCAT: Regex = Regex::new(r#"(?s)name="usecurrentcat".*?value="(.*?)""#).unwrap();
		static ref NAME: Regex = Regex::new(r#"(?s)name="name".*?value="(.*?)""#).unwrap();
		static ref QUESTIONTEXT: Regex = Regex::new(r#"(?s)name="questiontext\[text\]".*?>(.*?)<"#).unwrap();
		static ref DEFAULTMARK: Regex = Regex::new(r#"(?s)name="defaultmark".*?value="(.*?)""#).unwrap();
		static ref SINGLE: Regex = Regex::new(r#"(?s)name="single".*?value="(.*?)""#).unwrap();
		static ref SHUFFLEANSWERS: Regex = Regex::new(r#"(?s)name="shuffleanswers".*?value="(.*?)""#).unwrap();
		static ref ANSWERNUMBERING: Regex = Regex::new(r#"(?s)name="answernumbering".*?value="(.*?)""#).unwrap();
		static ref ANSWER_TEXT: Regex = Regex::new(r#"(?s)name="answer\[.*?\]\[text\]".*?>(.*?)<"#).unwrap();
		static ref FRACTION: Regex = Regex::new(r#"(?s)name="fraction\[.*?\]".*?value="(.*?)".*?selected"#).unwrap();
		static ref PENALTY: Regex = Regex::new(r#"(?s)name="penalty".*?value="(.*?)".*?selected"#).unwrap();
	}
	let mut form = Form::new();
	form.part("noanswers").contents(NOANSWERS.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("id").contents(ID.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("courseid").contents(COURSEID.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("sesskey").contents(SESSKEY.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("usecurrentcat").contents(USECURRENTCAT.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("name").contents(NAME.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("questiontext").contents(QUESTIONTEXT.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("defaultmark").contents(DEFAULTMARK.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("single").contents(SINGLE.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("shuffleanswers").contents(SHUFFLEANSWERS.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("answernumbering").contents(ANSWERNUMBERING.captures(html).unwrap()[1].as_bytes()).add()?;
	for (i, cap) in ANSWER_TEXT.captures_iter(html).enumerate() {
		form.part(&format!("answer[{}][text]", i)).contents(cap[1].as_bytes()).add()?;
	}
	for (i, cap) in NOANSWERS.captures_iter(html).enumerate() {
		form.part(&format!("fraction[{}]", i)).contents(cap[1].as_bytes()).add()?;
	}
	form.part("penalty").contents(NOANSWERS.captures(html).unwrap()[1].as_bytes()).add()?;
	Ok(form)
}

fn parse_gear_url(gear_url: &str) -> String {
	lazy_static! {
		static ref COURSEID: Regex = Regex::new("[^a-z](courseid=.*?(?:&|$))").unwrap();
		static ref ID: Regex = Regex::new("[^a-z](id=.*?(?:&|$))").unwrap();
	}
	let mut s = String::from("https://lms2.bsuir.by/question/question.php?");
	s.push_str(&COURSEID.captures(gear_url).unwrap()[1]);
	s.push_str(&ID.captures(gear_url).unwrap()[1]);
	s
}

fn main() -> Result<(), Box<dyn Error>> {
    const LOGIN_URL: &str = "https://lms2.bsuir.by/login/index.php";
	const TEST_URL: &[&str] = &[
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2732%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2774%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2775%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2776%2C21753"
		];
	const COOKIE: &str = "MoodleSession=8r1iheds5bdiu3ef8ni960ifnh";
	let questions = auth_cookie(TEST_URL[0], COOKIE)?;
	println!("ПДД: {:?}", questions.find("Дарья"));
	let gear_regex = Regex::new(r#"<a title="Редактировать".*?href="(.*?)".*?>"#).unwrap();
	let questions = gear_regex.captures_iter(&questions).map(|s| s[1].to_owned());
	let mut easy = Easy::new();
	for q in questions.take(1) {
		let url = parse_gear_url(&q);
		println!("{}\n\n\n", url);
		let html = auth_cookie(&url, COOKIE)?;
		let form = get_question_form(&html)?;
		easy.url(&url)?;
		easy.httppost(form)?;
		let response = auth_cookie(&url, COOKIE)?;
		println!("{}", &response);
	}
	Ok(())
}