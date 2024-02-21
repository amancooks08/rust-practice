#![allow(dead_code)]
pub fn run() {
    // Enums are types which have a few definite values.
    // You can define an enum by creating a new data type.
    // Enums are useful when you need to restrict a value to a set of possible values.
    // The enum keyword allows you to define a type by enumerating its possible variants.
    // Here's an example of an enum called WebEvent:
    enum WebEvent {
        // An enum may either be unit-like
        PageLoad,
        PageUnload,
        // like tuple structs
        KeyPress(char),
        Paste(String),
        // or like structures
        Click { x: i64, y: i64 },
    }

    // Here we have defined a WebEvent enum with four variants.
    // The first two, PageLoad and PageUnload, are unit variants.
    // The next two, KeyPress and Paste, are tuple variants.
    // The last, Click, is a structure variant.

    // We can create instances of each of these like this:
    let pressed = WebEvent::KeyPress('X');
    // `pressed` is a WebEvent::KeyPress variant containing 'x'

    let pasted = WebEvent::Paste("my text".to_owned());
    // `pasted` is a WebEvent::Paste variant containing the string "my text"

    let click = WebEvent::Click { x: 20, y: 80 };
    // `click` is a WebEvent::Click variant containing x and y values

    let _load = WebEvent::PageLoad;
    // `load` is a WebEvent::PageLoad variant

    let _unload = WebEvent::PageUnload;
    // `unload` is a WebEvent::PageUnload variant

    // We can also define methods on enums.
    // Here is an example of a method named inspect that we could define on WebEvent:
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    inspect(pressed);
    inspect(pasted);
    inspect(click);
}