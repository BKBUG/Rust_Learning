/* 
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn max(&mut self, other: Self)->Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h
        }
    }

    fn set_to_max(&mut self, other:Rectangle) {
        let max = self.max(other);
        *self = max;
    }
}

fn main()
{
    let mut rect = Rectangle {width:0, height: 1};
    let other_rect = Rectangle{width: 1, height: 0};

    rect.set_to_max(other_rect);
}
*/

/*
#[derive(Copy,Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn max(self, other:Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h
        }
    }

    fn set_to_max(&mut self, other: Self) {
        *self = self.max(other);//that self.max(other) desugars to Rectangle::max(*self, other)
        //The dereference *self does not require ownership over *self if Rectangle is copyable
    }
}*/


struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    fn set_to_max(&mut self, other: Self) {
        let max = self.max(other);
        drop(*self);//because max signature is max(self, other:Self)
        //self moved to self that is max parameter

        *self = max;//When we overwrite *self, Rust will implicitly drop the data previously in *self
    }
}

fn main()
{
    let mut r1 = Rectangle {
        width: 9,
        height: 9,
        name: String::from("r1") //the data located heap
    };

    let r2 = Rectangle {
        width: 16,
        height: 16,
        name: String::from("r2")
    };
    r1.set_to_max(r2);
}