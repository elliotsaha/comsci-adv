struct Backpack {
    color: String,
    size: String,
    items: Vec<String>,
    open: bool,
}

trait BackpackTrait {
    fn new(color: &str, size: &str) -> Self;
    // set the object’s open property to true. Provides appropriate message to the user.
    fn open_bag(&mut self);
    // set the object’s open property to false. Provides appropriate message to the user.
    fn close_bag(&mut self);
    // if the backpack is open, add the item argument to the backpacks items array.  Provide an appropriate message to the user.
    fn put_in(&mut self, item: &str);
    // if the backpack is open and the item argument is in the backpack, take out the item argument from the backpack.  Provide an appropriate message to the user.
    fn take_out(&mut self, item: &str);
}

impl BackpackTrait for Backpack {
    fn new(color: &str, size: &str) -> Self {
        Backpack {
            color: String::from(color),
            size: String::from(size),
            items: Vec::new(),
            open: false,
        }
    }
    fn open_bag(&mut self) {
        self.open = true;
        println!("Bag is now open");
    }
    fn close_bag(&mut self) {
        self.open = false;
        println!("Bag is now closed");
    }
    fn put_in(&mut self, item: &str) {
        if !self.open {
            println!("Please open bag first before putting anything in");
            return;
        }

        self.items.push(String::from(item));
        println!(
            "Added item, these are the contents of your backpack: {:?}",
            self.items
        )
    }
    fn take_out(&mut self, item: &str) {
        if !self.open {
            println!("Please open bag first before taking anything out");
            return;
        }

        for (pos, i) in self.items.clone().iter_mut().enumerate() {
            if i == item {
                // O(1) operation
                self.items.swap_remove(pos);
            }
        }

        println!(
            "Removed item, these are the contents of your backpack: {:?}",
            self.items
        )
    }
}

fn main() {
    let mut backpack1 = Backpack::new("blue", "medium");
    let _backpack2 = Backpack::new("red", "medium");
    let _backpack3 = Backpack::new("green", "large");

    backpack1.open_bag();
    backpack1.put_in("Lunch");
    backpack1.put_in("Jacket");
    backpack1.close_bag();
    backpack1.open_bag();
    backpack1.take_out("Jacket");
    backpack1.close_bag();
}
