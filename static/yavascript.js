var author = document.getElementById("name");
var summary = document.getElementById("summary");
var contents = document.getElementById("contents");
var display = document.getElementById("display");

var button = document.getElementById("submit");
button.onclick = function() {
	var request = new XMLHttpRequest();
	request.open('POST', "http://localhost:3000/post");
	request.setRequestHeader("Content-type", "application/json");
	request.onload = function() {
		alert(request.responseText);
		summary.value = "";
		contents.value = "";
	};

	var send = "{ \"author_handle\": \"" + author.value + "\", \"summary\": \"" + summary.value + "\", \"content\": \"" + contents.value+ "\" }";
	request.send(send);
}

function ping() {
	var request = new XMLHttpRequest();
	request.open('GET', "http://localhost:3000/feed");
	request.setRequestHeader("Content-type", "application/json");
	request.onload = function() {
		fill_feed(request.responseText);
	};
	request.send();
	setTimeout(ping, 1000);
}
ping();

function fill_feed(responseText) {
	var list = JSON.parse(responseText);
	list.reverse();
	
	clear_display();
	list.forEach(function(item, index, array) {
		add_to_display(item);
	});
}

function add_to_display(item) {
	var para = document.createElement("p");
	var name = document.createTextNode(item.author_handle + ": ");
	var link = document.createElement("a");
	var linkText = document.createTextNode(item.summary);
	link.appendChild(linkText);
	link.href = "http://localhost:3000/post/" + item.uuid;

	para.appendChild(name);
	para.appendChild(link);

	display.appendChild(para);
}

function clear_display() {
	var elements = display.getElementsByTagName("p");
	while (elements.length > 0) {
		var node = elements[0];
		display.removeChild(node);
		node.nodeValue = "";
	}
}
