import js.Browser;

class Main 
{
    static function main() {
        var button = Browser.document.createButtonElement();
        button.textContent = "Click me!";
        button.onclick = (event) -> Browser.alert("Haxe is great");
        Browser.document.body.appendChild(button);
    }
}