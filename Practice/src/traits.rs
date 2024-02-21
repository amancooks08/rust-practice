pub fn run() {
    /*
        Since we have come this far learning Rust, it is time we talk 
        about some conventional stuff. Yess I'm talking about OOPs.
        Rust has a powerful feature called Traits which is similar to
        interfaces in other languages. It allows you to define a set of
        methods that a type must implement. You can use traits to define
        shared behavior in an abstract way.
        But there are some differences between traits and interfaces.
        Let's see how we can define a trait and implement it for a struct.
        Reference the code after this function for that.
     */

    // Let's create a Dog struct and implement the Animal trait for it.
    let dog = Dog {
        name: String::from("Entertainment"),
        
    };

    // Now we can call the methods from the Animal trait on the Dog struct.
    println!("{}", dog.sound());
    if dog.can_fly() {
        println!("{} can fly!", dog.name);
    } else {
        println!("{} can't fly!", dog.name);
    }
}

// First we define a struct, hope you recall how to do that.
struct Dog {
    name: String,
}

// Then we define a trait using the trait keyword.
trait Animal {
    // Inside the trait we define the method signatures.
    // These are the methods that the type implementing this trait must have.
    fn sound(&self) -> String;
    fn can_fly(&self) -> bool;

    /*
        You can also define a method in a trait with a default implementation.
        This is useful when you want to provide a default behavior for the method.
        The type implementing the trait can override the default implementation.
        Or can use the default implementation if it doesn't want to override it.
     */
}

// Now we implement the trait for the struct.
impl Animal for Dog {
    // We define the methods from the trait for the struct.
    fn sound(&self) -> String {
        format!("{} says Woof Woof!", self.name)
    }

    fn can_fly(&self) -> bool {
        false
    }
}