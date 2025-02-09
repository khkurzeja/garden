import js.Browser;

class Main 
{
    var canvas: js.html.CanvasElement;
    var ctx: js.html.CanvasRenderingContext2D;

    function new() {
        canvas = Browser.document.createCanvasElement();
        canvas.width = Browser.window.innerWidth;
        canvas.height = Browser.window.innerHeight;
        //canvas.width = Browser.document.documentElement.clientWidth;
        //canvas.height = Browser.document.documentElement.clientHeight;
		Browser.document.body.appendChild(canvas);

        ctx = canvas.getContext2d();

        Browser.window.requestAnimationFrame(update);
    }

    function update(time_ms: Float) {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        ctx.fillStyle = "rgb(200 0 0)";
        ctx.fillRect(100 + Math.sin(time_ms * .01) * 100, 10, 50, 50);
        
        Browser.window.requestAnimationFrame(update);
    }

    static function main() { new Main(); }
}