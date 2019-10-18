
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

fn get_with_cookie(url: &str, cookie: &str) -> Result<String, Box<dyn Error>> {
	let mut easy = Easy::new();
	easy.url(url)?;
	easy.useragent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/77.0.3865.90 Chrome/77.0.3865.90 Safari/537.36")?;
	easy.follow_location(true)?;
	//easy.verbose(true)?;
	easy.cookie(cookie)?;
	let response = send_request(&mut easy)?;
	Ok(response)
}

fn post_with_cookie(url: &str, cookie: &str, form: Form) -> Result<String, Box<dyn Error>> {
	let mut easy = Easy::new();
	easy.url(url)?;
	easy.useragent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/77.0.3865.90 Chrome/77.0.3865.90 Safari/537.36")?;
	easy.follow_location(true)?;
	//easy.verbose(true)?;
	easy.cookie(cookie)?;
	easy.httppost(form)?;
	let response = send_request(&mut easy)?;
	Ok(response)
}

fn get_question_form(html: &str) -> Result<Form, curl::FormError> {
	lazy_static! {
		static ref NOANSWERS: Regex = Regex::new(r#"(?s)name="noanswers".*?value="(.*?)""#).unwrap();
		static ref ID: Regex = Regex::new(r#"(?s)name="id".*?value="(.*?)""#).unwrap();
		static ref COURSEID: Regex = Regex::new(r#"(?s)name="courseid".*?value="(.*?)""#).unwrap();
		static ref SESSKEY: Regex = Regex::new(r#"(?s)name="sesskey".*?value="(.*?)""#).unwrap();
		static ref NAME: Regex = Regex::new(r#"(?s)name="name".*?value="(.*?)""#).unwrap();
		static ref QUESTION_TEXT: Regex = Regex::new(r#"(?s)name="questiontext\[text\]".*?>(.*?)<"#).unwrap();
		static ref QUESTION_FORMAT: Regex = Regex::new(r#"(?s)name="questiontext\[format\]".*?value="(.*?)""#).unwrap();
		static ref DEFAULTMARK: Regex = Regex::new(r#"(?s)name="defaultmark".*?value="(.*?)""#).unwrap();
		static ref SINGLE: Regex = Regex::new(r#"(?s)name="single".*?value="(.*?)""#).unwrap();
		static ref ANSWERNUMBERING: Regex = Regex::new(r#"(?s)name="answernumbering".*?value="(.*?)""#).unwrap();
		static ref ANSWER_TEXT: Regex = Regex::new(r#"(?s)name="answer\[[0-9]+\]\[text\]".*?>(.*?)<"#).unwrap();
		static ref ANSWER_FORMAT: Regex = Regex::new(r#"(?s)name="answer\[[0-9]+\]\[format\]".*?value="(.*?)""#).unwrap();
		static ref FRACTION: Regex = Regex::new(r#"(?s)name="fraction\[[0-9]+\]".*?value="([0-9|\.]+)"\s+selected"#).unwrap();
		static ref PENALTY: Regex = Regex::new(r#"(?s)name="penalty".*?value="(.*?)".*?selected"#).unwrap();

	}
	let mut form = Form::new();
	
	form.part("id").contents(ID.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("courseid").contents(COURSEID.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("sesskey").contents(SESSKEY.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("name").contents(NAME.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("questiontext[text]").contents(QUESTION_TEXT.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("questiontext[format]").contents(QUESTION_FORMAT.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("defaultmark").contents(DEFAULTMARK.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("single").contents(b"1").add()?;
	form.part("answernumbering").contents(b"none").add()?;
	form.part("noanswers").contents(NOANSWERS.captures(html).unwrap()[1].as_bytes()).add()?;
	form.part("qtype").contents(b"multichoice").add()?;
	form.part("_qf__qtype_multichoice_edit_form").contents(b"1").add()?;
	form.part("usecurrentcat").contents(b"1").add()?;
	form.part("shuffleanswers").contents(b"1").add()?;
	form.part("penalty").contents(b"0.3333333").add()?;


	for (i, cap) in ANSWER_TEXT.captures_iter(html).enumerate() {
	 	form.part(&format!("answer[{}][text]", i)).contents(cap[1].as_bytes()).add()?;
 }
	 for (i, cap) in ANSWER_FORMAT.captures_iter(html).enumerate() {
	 	form.part(&format!("answer[{}][format]", i)).contents(cap[1].as_bytes()).add()?;
	 }
	 for (i, cap) in FRACTION.captures_iter(html).enumerate() {
	 	form.part(&format!("fraction[{}]", i)).contents(cap[1].as_bytes()).add()?;
	 }
	 
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
    //const LOGIN_URL: &str = "https://lms2.bsuir.by/login/index.php";
    const QUSTION_URL: &str = "https://lms2.bsuir.by/question/question.php";
	const TEST_URL: [&str; 6] =
	[
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2732%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2774%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2775%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2776%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2801%2C21753",
		"https://lms2.bsuir.by/question/edit.php?courseid=374&category=2802%2C21753",
	];
	const COOKIE: &str = "MoodleSession=rmhhds59riv707ng3qfin2q542";
	for test in TEST_URL.iter().skip(5) {
		println!("Test: {}", test);
		let questions = get_with_cookie(test, COOKIE)?;
		println!("ПДД: {:?}", questions.find("Дарья"));
		let gear_regex = Regex::new(r#"<a title="Редактировать".*?href="(.*?)".*?>"#).unwrap();
		let questions = gear_regex.captures_iter(&questions).map(|s| s[1].to_owned());
		for q in questions {
			let url = parse_gear_url(&q);
			println!("{}", url);
			let html = get_with_cookie(&url, COOKIE)?;
			let form = get_question_form(&html)?;
			let response = post_with_cookie(QUSTION_URL, COOKIE, form)?;
			println!("{}", response.find("Редактировать вопросы").is_some());
		}
	}
	Ok(())
}