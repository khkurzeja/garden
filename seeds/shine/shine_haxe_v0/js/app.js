// Generated by Haxe 4.3.6
(function ($global) { "use strict";
var Main = function() { };
Main.main = function() {
	var button = window.document.createElement("button");
	button.textContent = "Click me!";
	button.onclick = function(event) {
		window.alert("Haxe is great");
	};
	window.document.body.appendChild(button);
};
var haxe_iterators_ArrayIterator = function(array) {
	this.current = 0;
	this.array = array;
};
haxe_iterators_ArrayIterator.prototype = {
	hasNext: function() {
		return this.current < this.array.length;
	}
	,next: function() {
		return this.array[this.current++];
	}
};
Main.main();
})({});
