
class BoolType {
    //var value: Bool;
    //public function new(value) { this.value=value; }
}

class IntType {
    //var value: Int;
    //public function new(value) { this.value=value; }
}

class FloatType {
    //var value: Float;
    //public function new(value) { this.value=value; }
}

class StringType {
    //var value: String;
    //public function new(value) { this.value=value; }
}

class ListType {
    //var value: Array<ShineType>;
    //public function new(value) { this.value=value; }
}

class BuiltInType {

    public function new() {}
}

class UserType {
    public function new() {}
}

enum ShineType {
    TBuiltIn(value: BuiltInType);
    TUser(value: UserType);
}

enum ShineFunction {
    FBuiltIn;
    FUser;
}

class ClassicGraphics {
    public function new() {}
}

enum Graphics {
    GClassic;
}

// Like flash, or smalltalk morph.
class Sprite {
    var parent: Sprite;
    var children: Array<Sprite>;
    var name: String;
    public function new() {
        parent = null;
        children = null;
        name = "";
    }
    function initChildren() { if (children==null) { children=[]; } }
    //function reparent(newParent:Sprite) { if (parent!=null) { parent.removeChild(this); } newParent.addChild(this); }
    public function addChild(child:Sprite) { initChildren(); if (child.parent==null) { children.push(child); } }
    public function removeChild(child:Sprite) { children.remove(child); child.parent=null; }
    public function getChildAt(index:Int) { return children[index]; }
    public function getChild(name:String) { for (child in children) { if (child.name == name) return child; } return null; }
    public function update() { if (children!=null) for (child in children) child.update(); }
    public function draw(canvas:js.html.CanvasRenderingContext2D) {}
    //public function onMouseDown(){}
    //public function onMouseUp(){}
    //public function onMouseMove(){}

    //**TODO: Events (maybe don't do event listners. When a sprite handles an event, call the same function on all children, and let child respond or not.)
}

class BoolSprite extends Sprite {
    public var value: Bool;
    public function new() { super(); value = false; }
}
class IntSprite extends Sprite {
    public var value: Int;
    public function new() { super(); value = 0; }
}
class FloatSprite extends Sprite {
    public var value: Float;
    public function new() { super(); value = 0.0; }
}
class StringSprite extends Sprite {
    public var value: String;
    public function new() { super(); value = ""; }
}
class Point2 extends Sprite {
    public function new() {
        super();
    }
}
class Rectangle extends Sprite {
    public function new() {
        super();
    }
}
class Circle extends Sprite {
    public function new() {
        super();
    }
    public override function draw(canvas:js.html.CanvasRenderingContext2D) {
        var center = getChild("center");
        var radius = getChild("radius");
        center.update();
        radius.update();
        var x = center.getChild("x");
        var y = center.getChild("y");
        canvas.ellipse(x.value, y.value, radius.value, radius.value, .0, .0, .0);
    }
}

class Editor {
    var types: Map<String, ShineType>;
    var functions: Map<String, ShineFunction>;
    
    var input_values: Map<String, Dynamic>;
    var values: Map<String, Dynamic>;

    public function new() {}
}