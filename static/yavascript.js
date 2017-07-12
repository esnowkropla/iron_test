var author = document.getElementById("name");
var summary = document.getElementById("summary");
var contents = document.getElementById("contents");

var button = document.getElementById("submit");
button.onclick = function() {
	var request = new XMLHttpRequest();
	request.open('POST', "http://localhost:3000/post");
	request.setRequestHeader("Content-type", "application/json");
	request.onload = function() {
		alert(request.responseText);
	};

	var send = "{ \"author_handle\": \"" + author.value + "\", \"summary\": \"" + summary.value + "\", \"content\": \"" + contents.value+ "\" }";
	request.send(send);
}

function ping() {
	var request = new XMLHttpRequest();
	request.open('GET', "http://localhost:3000/feed");
	request.setRequestHeader("Content-type", "application/json");
	request.onload = function() {
		alert(request.responseText);
	};
	request.send();
	setTimeout(ping, 5000);
}
//ping();
