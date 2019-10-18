# Automatic questions editing in Moodle
A small tool that allows you to edit multiple questions automatically (for example, add/remove answers numbering, etc.).

It works as follows:
1) GET the list of questions in HTML format
2) extract question data from the HTML form (yeah, "parsing HTML with regex is a bad idea" yada yada yada..., but who cares?)
3) modify form data obtained on previous step in some way
4) make POST request with modified data

It took me several days of reverse engineering. I can't count how many times stupid Moodle refused to accept my requests and just responsed with the same unchanged question page. NO ERRORS AT ALL!!!
Fortunately, now it works!
