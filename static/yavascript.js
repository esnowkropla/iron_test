var author = document.getElementById("name");
var summary = document.getElementById("summary");
var contents = document.getElementById("contents");

alert("wat");

var button = document.getElementById("submit");
button.onclick = function() {
	var request = new XMLHttpRequest();
	request.open('POST', "http://localhost:3000/post");
	request.setRequestHeader("Content-type", "application/json");
	request.onload = function() {
		alert(request.responseText);
	}

	console.log("author: " + author.value);
	console.log("summary: " + summary.value);
	console.log("contents: " + contents.value);
	var send = "{ \"author_handle\": \"" + author.value + "\", \"summary\": \"" + summary.value + "\", \"content\": \"" + contents.value+ "\" }";
	request.send(send);
}
