/*
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}*/


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h
        }
    }
}

/*
fn main()
{
    let rect = Rectangle {
        width: 0,
        height: 0
    };

    println!("{}", rect.area());

    let other_rect = Rectangle{width: 1, height: 1};

    let max_rect = rect.max(other_rect);
}*/

fn main()
{
    let rect = Rectangle {
        width: 0,
        height: 0
    };
    //error:cannt borrow 'rect' as mutables, as it is not declare as mutable
    //rect.set_width(0);//note: set_width method expect 'w' permission, but rect don't have
    //because rust default variable is immutable
    //except specify use the 'mut' keyword

    //compiling
    let mut rect = Rectangle {
        width: 0,
        height: 0
    };
    rect.set_width(1);

    let rect_ref =&rect; //rect borrowed a immutable refernece
    //error
//    rect_ref.set_width(2); //because rect_ref is a immutable reference
    //so it error

    let rect = Rectangle {
        width: 0,
        height: 0
    };

    let other_rect = Rectangle {
        width: 1,
        height: 1
    };

    let max_rect = rect.max(other_rect); //note:max(self, Self) -> Self {}
    //so rect and other_rect will move into the self and the Self taht the max of paramter
    //so after this line, rect and other_rect will be invalid, they are moved

//    println!("{}", rect.area());//error: value borrowed here after move
    impl Rectangle {
        fn set_to_max(&mut self, other: Rectangle) {
            *self = self.max(other); //on a refernece call a method
            //because assign need owner permission, but a reference don't have owner permission?
            //here need to explain 
            //TODO
            
            //so ot is error
        }
    }    
}

