
class BoolSprite {
    var value: Bool;
    public function new(value) { this.value=value; }
}

class IntSprite {
    var value: Int;
    public function new(value) { this.value=value; }
}

class FloatSprite {
    var value: Float;
    public function new(value) { this.value=value; }
}

class StringSprite {
    var value: String;
    public function new(value) { this.value=value; }
}

class ListSprite {
    var value: Array<Sprite>;
    public function new(value) { this.value=value; }
}

enum Command {
    Self(out: Int);
    Parent(out: Int, target: Int);
    //Root(out: Int);  // Could be useful. Could use root.libs.x, to access a loaded library. Not sure about the lib thing.
    Literal(out: Int, value: Dynamic);
    ReadStash(out: Int, target: Int, name: Int);
    SetStash(target: Int, name: Int, value: Int);
    AddChild(target: Int, name: Int, child: Int);
    RemoveChild(target: Int, name: Int);  // Do we need RemoveChild?
    GetChild(target: Int, name: Int);
    Goto(target: Int, timeline_name: Int, frame_num: Int);  // frame_num is a register, of course. 
    MakeSprite(out: Int, name: Int, args: Int);  // args points to the first of n consecutive input regs
}
class Frame {
    public var commands: Array<Command>;
    public function new() {}
}
class Timeline {  // Like Flash's Scene
    public var name: String;
    public var frames: Array<Frame>;
    public function new() {}
}
class MadeTemplate {
    public var param_names: Array<String>;
    //public var timelines: Map<String, Timeline>;
    public var timelines: Array<Timeline>;
    public function new() {}
}
class MadeSprite {
    public var template: MadeTemplate;
    public var parent: Null<Sprite>;
    public var children: Array<Sprite>;
    //var args: Array<Sprite>;  // Do we need to keep the args around?
    public var stash: Map<String, Sprite>;
    public var timeline_num: Int;
    public var frame_num: Int;
    public function new() {}
}

enum Sprite {
    SBool(value: BoolSprite);
    SInt(value: IntSprite);
    SFloat(value: FloatSprite);
    SString(value: StringSprite);
    SList(value: ListSprite);
    SMade(value: MadeSprite);
}

// rp is regsiter pointer: the reg we consider to be '0'.
function eval_made_sprite(self: MadeSprite, regs: Array<Sprite>, rp: Int) {
    var template = self.template;
    var timeline = template.timelines[self.timeline_num];
    var frame = timeline.frames[];
    var cmds = frame.commands;
    for (cmd in cmds) {
        switch (cmd) {
            case Self(out:Int): {
                regs[rp + out] = self;
            }
            case Parent(out:Int): {
                regs[rp + out] = self.parent;
            }
            case Literal(out:Int, value:Dynamic): {
                if (value is Bool) regs[rp + out] = SBool(cast(value, Bool));
                else if (value is Int) regs[rp + out] = SInt(cast(value, Int));
                else if (value is Float) regs[rp + out] = SFloat(cast(value, Float));
                else if (value is String) regs[rp + out] = SString(cast(value, String));
                else {  }  // ERROR: How to handle?
            }
            case ReadStash(r_out:Int, r_target:Int, r_name:Int): {
                var d_name = regs[rp + r_name];
                //**WRONG: Need to do a pattern match instead of cast.
                switch (d_name) {
                    SString(value: StringSprite) {
                        //**TODO
                    }
                    _: {}  // ERROR
                }
                var name = cast(name, String);  //ERROR if not string
                var t = regs[rp + r_target];
                switch (t) {
                    case SMade(target: MadeSprite): {
                        regs[rp + r_out] = target.stash[name];
                    }
                    _: {} // ERROR. Only made has a stash.
                }
            }
            case SetStash(r_target:Int, r_name:Int, r_value:Int): {
                var d_name = regs[rp + r_name];
                var name = cast(name, String);  //ERROR if not string
                //var d_value = regs[rp + r_value];
                var t = regs[rp + r_target];
                switch (t) {
                    case SMade(target: MadeSprite): {
                        target.stash[name] = regs[rp + r_value];
                    }
                    _: {} // ERROR. Only made has a stash.
                }
            }
            case AddChild: {}
            case RemoveChild: {}
            case GetChild: {}
            case Goto: {}
            case MakeSprite: {}
        }
    }
}

// What are the things that sprites must do?
// And what can we do with sprites?

// Construct
// Change state
//  - How? I can send a message? Name? Name and params? Name, frame num params?

// All sprites have an initial state.
// Additional states take the same sprite as input.
// Whether or not the needed state is available is decided at runtime. At least for prototyping.

// In flash, the states and keyframes are the same thing.
// I expect to use states often.
// I think keyframes will be much less common.

// Should I force a state change every frame?
// It would be ok to transition to the same state, but the thing will be reconstructed each frame (not free).
// This would keep things "constructive" / immediate-mode.
// And, in the future, I may consider how to make things "reactive" so that we don't need to do work for things that aren't changing.

// Each frame, the root sprite gets an "update" message/event.
// The sprite needs to know what state it transitions to given an update event.
// It may transition to the same state.
// The old state is an input to the new state, along with any data attached to the update event.
// I suppose, a sprite does not need to change state if it does not receive a message.
// A sprite does not necessarily need to send the update message to its children.

// How does this differ from Smalltalk?
// Objects in smalltalk receive messages, which may mutate the data in the object.
// These messages are just methods.
// The object may fully "reconstruct" itself if it wants, but it does not have to.
// Is this model strictly better than my state machine model?

// What is nice about the state machine model?
// Since everything must be reconstructed, each bit of data corresponds to an "instruction".
// In smalltalk, there is a bit of a disconnect between data and instructions.

// However, if we imagine getting the current sprite-state as input to the new state,
// we cannot think of the input state as coming from a particular state, if we want to be able to transition to a state from any other state.
// Ie., a state may only have one previous state. This seems like a big limitation.
// If we want to transition to a state frome multiple other states, we would need the construction to be from a variant of prior states.

// Alternatively, we may not have the new state take the old state as input.
// Each state may take a list of params as input.
// Each state must specify how it transitions to other states, given an event.
// So, it does not pass itself, but may pass its data.
// When we specify the data for the new state, can we transform it with an expr? Or should it be passed in in its original state?
// Allowing it to be transformed by an expr sounds like a good idea, but it might be interesting to try doing without it.

// I could just accept defeat and separate instructions and data.
// Let each state declare the data it expects, and have each state transition construct the expected data.

// I think I like the state-construction from a variant idea.
// Let's give that a shot.
// Imagine a "code" tree on the left.
// The roots are the different state-variants, and the lists contained in the roots are the construction instructions.

// Sprite has states.
// States have a set of constructors (how the state gets made).
// A constructor may work for multiple different input states, assuming the states have the inputs needed.
// The constructor also specifies what event it is a response too, and what data is expected to come along with the event.

//==========================

// If I force every state change to be a reconstruction,
// then I don't really need a GC.
// I can allow some large objects to be alloc'd still, and it would
// probably be reasonable to just use ref counting for these.

// I would need to prevent referencing an object from the old state.
// However, I could move an object from the old state to the new state.
// This would copy the memory, then I could make a ref to the mem
// in the new location.

// This mem mangagement idea is interesting, but seems dangerously
// likely to not work well. Prob just stick to js GC for now.

//==========================

// How flash works

// Create a sprite.
// Add it to one other sprite.
// It responds to events.
// "Main" event is onEnterFrame.

// I probably should make my lang like this.
// Reduce how much I need to invent.

// Treat sprites in a smalltalk object kind of way.
// Maybe self-like.

// Then, we need to specify what data a sprite will have.

// We need to define functions that can construct the sprite.

// We want to be able to visualize both an object as well
// as the process of constructing of an object.

// If we call a method on an object, and it changes the objects
// state, we should be able to visualize that as well.
// So, all methods may have a visualization attached.

//===========================

// In flash
// when you create a sprite (I'm using this to mean movieclip),
// a new class is created that extends Sprite.
// Each frame might have its own onEnterFrame.
// So, that function

// A movieclip has scenes (like states)
// A scene has frames.


// Have sprites, scenes, frames.
// Sprite may have data shared between scenes.
// Scenes may be parameterized?
//   - At first, may not be necessary. Could just have implicit
//     assumption that some data is set on the sprite before
//     we transition to the new scene.
// Sprite must declare what data it has. This will make it easier
// to refer to that data from a gui.
// Some data may be declared to be an input. It must be given
// on construction.

// A user may choose to use the initial scene as a constructor.
// The sprite may transition to the update scene after construction.

// When a frame enters, should anything from the previous frame
// persist into the next frame? Or is each frame constructed from
// scratch?
// I think, in flash, objects persist between frames unless they
// have been added or removed as children.
// Maybe, we can have each frame constructed from scratch, but
// we can store a child in the data and display it on each frame.


// For now, do not expect sprites to execute within the editor.
// I can worry about such things in a future version.

// ==========

// A sprite may either loop or stop at the last frame.
// If it stops, then the last frame is executed over and over.
// This means, the last frame must be an input to itself.

// The last frame of a sprite may loop back to any frame, including the
// first frame or even itself.
// Which ever frame it loops back to must be able to take its current state
// as "input".
// Ie., if a sprite loops back to frame 2, then the state of the sprite
// after both the first and the last frame must be the same, and that state
// is the same as the input to frame 2.
// By "state", I mean the children attached to the current sprite.
// So, if frame 2 expects to have a child called "hero" (it may 
// have some required type in the future too) when it begins, then 
// both frame 1 and n must have "hero" after execution.
// So, frames are in some sense parameterized, and the arguments to
// these parameters are implicitly feed in from "source frames".
// - In the initial prototype, I may not need to staticly check that 
//   this holds. It could be dynamically checked.
// - I might need to require that "hero" always refers to the same child.
//   - If I am editing frame 2, then how do I display the "hero" child?
//     Is it based off of frame 1 or n?
//     If I require that "hero" is always the same thing, then I can just
//     display the one "hero".

// Maybe, children can be "stashed".
// The stash is the sprite's "data".
// On each new frame,
// the stashed sprite may be added as a child.
// When it is added as a child, it moves to the next frame.

// ===========

// Problem,
// To properly display a parameterized object, you need to know the
// arguments.
// Previously, I imagined these aruments being editable in the editor.
// But, if the input is complex enough, this may be very difficult.

// Now,
// If a sprite has retained state, when editing a frame, I must also
// imagine each retained object as input to the frame.

//============

// How do I make/represent an instance of a sprite?

// What differs between a sprite and an instance of one?
// - Parent, children, args, stash

// When editing a sprite template, we are always also editing
// a concrete sprite.

// ======

// A sprite cannot contain itself,
// but it can contain a different instance of itself.
// This might be dangerous. Infinite recursion and memory use.
// Need a recoverable "stack overflow".

// ============

// Make interpreter that does not know about sprites.
// It has a stack for temp storage,
// and it can process pointers/refs to heap storage.

// Then, how do we hook this interpreter up to a sprite interpreter?

// Why not just compile everything directly to js?
// Could this possibly simplify things?

// ---

// Have template for all built in sprite types and for the made type.
// Each has a 'to_js' function.
// Each template needs to implement certain funcs, like goto, read and
// set stash, etc.

// For now, have all sprite types implement the same interface.
// Even bool and int. Even if it is heavy and slow.
// Later, I can optimize the heavy stuff away when I know it is ok 
// to do so.
// For example, if an int isn't visualized, we can throw away the visual
// stuff.

// Concern, do we really want to recompile the js just because 
// a sprite was toggled from vis to not-vis?
// We could store both versions.
// But, if something inside the sprite is toggled, everything 
// containing it will need to be recompiled.
// I might want to try to decoupling render logic from logic-logic
// so that the execution can happen quickly whether or not rendering
// happens.
// - Actually, it should be fine to have just two versions of each
//   sprite. The fast and the slow.
//   The fast is just execution. Optimized.
//   The slow includes all editor stuff. Like vis. Unoptimized,
//   so it can be understood/debugged.
//   Toggling a child's vis should not require a recompile of the 
//   parent.
  

// ============

// To be renderer agnostic, let the sprites produce "render commands"
// that are later collected and passed to the renderer.
// Like clay, the c layout engine.

//===============

// Sprite looks like
// {
//   template: ref,
//   //parent: ref,
//   //children: [],
//   stash: {},
//   current_timeline: int,
//   current_frame: int,
//   //editor: {},  // Should editor data be here or in the stash?
//   // What else?
// }

// Template looks like
// {
//   timelines: [],
// }

// Timeline
// {
//   frames: [],
// }

// Frame
// {
//   commands: [],
//   editor_fn: ref,
//   optimized_fn: ref,
// }

// =======

// I'm not convinced I need to store parent and children explicitly.
// The stash is data that a sprite has access to.
// The children are the data that is "active" for the frame.
// However, the concept of active here is really one of UI and graphics.
//  - No, actually, only the active stuff gets updated for the frame.
//    Regardless, the execution of a parent implicitly controls which
//    things are active or updated for the frame. We don't seems to need
//    an explicit list of active things, for now.
//    If someone wants such a list, they could build it and keep it in the
//    stash.
//  - I can think of the stash as children. And the commands determine
//    which children are active. Also, there may be active "children"
//    that exist for the single frame but are not persistent due to
//    not being in the stash.
//     - If we don't store the non-stashed children in any way, then we
//       can't do a pass where we collect stuff for rendering.
//       Storing children might be important for the editor. Might not
//       need to store the parent. If the editor stuff is off, then
//       we don't need children, probably.

// =========

// If I add two ints, the result is an "add" sprite with a child that
// is the result of the addition.
// It is a little unfortunate that we have to extract the result
// from the add sprite, if we want it.
// Should I consider having functions that produce sprites, instead of
// being a sprite?

// =========

// Why do I insist on thinking of things like flash?
// I could just have structs, enums, and functions.
// I can allow the user to create visualizations for each.
// After all, operations need to be visualized.

// The nice thing about the flash idea is that the frames give
// a decent way to implement init and update functions for an object.
// I also like the built in state machine stuff.
// If we don't want an automatic update, we can have the timeline
// stop on the final frame instead of looping.
// How exactly do movieclip updates work in flash for objects that are
// not on the active frame?

// I'm thinking of making a language much more like rust.
// This is a departure from flash.

// =========

// Back to a flash-like.
// But make it slightly more rusty.
// Let's make the stash just be the list of children.
// So, if something is in the stash, it is a child.
// And, a sprite may only be a child of a single other sprite.
// This is like single ownership in rust. What if we completely disallow
// multi-ownership? No shared_ptr.
// Now, how do children work? What if we want a child to only be on
// one frame?

// ==========

// If objects have no view, but constructions have a view,
// then I don't have to think of, for example, an int having a view.
// Instead, when creating an int (as a 'source node'), that manner of
// creating an int may have a view within the editor.
// For example, 'int from textbox' or 'int from slider'.
// I don't love that this forces us to use a single 'view' per int.
// However, if all such views have the same interface (no inputs, one int
// as output), then we should be able to trivially swap one view for
// another.
// The views may have different state, however. For example, a slider 
// may have a location, size, and range. The textbox may have location
// and size, but no range.
// We may want to preserve as much similar state as possible when switching
// views. Unfortunately, it may not be easy to know which state can
// reasonably transfer to a new view.
// And, what if I wanted two views at the same time?
// I'm sure I've thought this before, but the views/controls really
// should be separate from the data.

// Should I extend this idea to general sprites?
// Any time you define a sprite, you should define at least one view for it.
// And time you instantiate a sprite, you probably want to instantiate
// at least one view for it.
// Is this tedious?

// Example, a sprite may or may not have a timeline.
// If it does, and you want to edit it, you must add a timeline editor
// view to wherever you plan to edit it.

// The view sprites need to take a ref to whatever they are a view of.
// Things are only passed by ref by default?
// Let the compiler optimize when it can determine this isn't necessary?

// ============

// Think less of rust and more of js.
// I have objects, functions, arrays, prototype/class(struct).
//  - js uses call by sharing.

// Define a lib of classes and functions.
// Declare what of a class is a member vs static.
// Classes do not have editor only data, probably.
// Functions may have editor only data though, so we may have
// two versions of each function.
// A function may include visualizations.
// This means that functions must be reified in some way?
//  - How? Why? Think

// ============

// A program is made of source data and transformations.
// We need views/controls of the source data and we need
// views of the transformations.
//  - We might also want a view of result/intermediate data, but
//    transformations could cover that.
//  - If we think of functions/transformations as data, then viewing
//    a transformation is like viewing the source code.
//     - We often will want to view a specific instance of the source
//       given particular input values.

// For js,
// An object is just a list of untyped fields we expect to have.
// The constructor should expect each of the field as input.
// So, construction is like a rust struct.
// We may specify simpler free functions for construction.
// A function is a block of code.
// We must list untyped fields we expect as input.
// A block has a list of transformations.
// Each transformation has a name for its output.
// The final transformation may be unnamed. Its output is the output
// of the current block.

// A block is itself an object. Would it be simpler if it wasn't?
// If we want to create a view for a transformation, we need to be
// able to pass the transformation, as an argument, to the constructor
// for its view.
// For this, it would help if the transformation were an object.
//  - We can also think about this as, to make a good view for a 
//    transformation, we want to have the intermediate data available.
//    Ie., when evaluating a function, we want to store all temp/local
//    variables.
//  - As implementation, when evaluating a function, we can return two 
//    things: 1) the expected return value, 2) a structure containing
//    all data used during the evaluation. This is "editor data".
//    If some evaluation/data is expensive, the function can be marked as
//    "hidden", which switches to the optimized function that only returns
//    the expected return value.
//     - A function call that takes editor data as an argument is an
//       "editor only" function call. It may produce editor data like
//       render commands.

// So, when a block is evaluated, you back two values, the return value
// and the editor data.

// I like this design for a programming ide where I just want to write
// one function.
// What if I want graphical programming with a notion of updating/time.

// **** If I had this idea fully working, I could try to design the
//      graphical programming environment inside of it.
//      Would it be possible to design this idea without an inherent
//      notion of updates? How will I handle inputs?
//      I need a notion of init and update. Not necessarily of frames
//      and graphics.
//      Init can just be free functions.
//      Update probably should be attached to objects.
//      If an "editor object" inside of a function can update, then
//      the function must also be persistent and updatable.
//      So, I am back to everything being objects/sprites.
//      I don't necessarily need frames, but every object must have
//      an update function.
//     - Do I need functions? I just have sprite constructors and
//       sprite updaters?
//        - A constructor takes some inputs and sets some fields
//          on it self.
//        - I also want sprite types, because all sprites of the 
//          same type should have the same data fields, including
//          the same updater.
//         - So a sprite type may have multiple constructors, but
//           one updater.
//         - Why have just one special function, the updater?
//           What if I want it to take params?
//           What if I want different updaters for different circumstances?
//           I would just have methods. Bringing me back to a smalltalk design.
//           It seems that I really want to implement smalltalk.
//           I should have a root object that is initialized and repeatedly has
//           its update method called.

// If I do away with the concept of return value, I could again let the 
// sprites be my functions.

// I could specify editor data as public/private to indicate whether or not
// its view should be visible outside of itself.
// For example, when editing a movieclip, you probably want a view of its
// timeline. But, when outside of the movieclip, you probably don't want
// to see its timeline.

// Example:
// MySprite {
//   has timeline
//   has private editor timeline view (taking timeline)
  
// }

//=============

// What are the elements of graphical programming?
// Points, vectors, frames, time, lines, curves, circles, triangles, rectangles,
// Text, images
// Grab, drag, collision
// Lists/collections
// Composite objects, Recursive objects?

// Render values as text on canvas.
// Render control flow stmts as text or other simple graphical primitives.
//  - Not sure how these will be interacted with, but I'll figure it out when I need it.

//==============

// Make an Ada like lang?
// Is it a good idea to copy Ada concepts?
// Where does the graphical stuff come in?