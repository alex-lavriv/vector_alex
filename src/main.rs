fn main() {
    println!("Hello, world!");
    // let mut vector = vec![];
    //
    // for _ in 1..1000 {
    //     println!("len: {}, capacity {}", vector.len(), vector.capacity());
    //     vector.push(1);
    //     assert_eq!(vector[0], 1);
    // }
    // for _ in 1..1000 {
    //     println!("len: {}, capacity {}", vector.len(), vector.capacity());
    //
    //     vector.pop();
    // }
    // vector.shrink_to_fit();
    // println!("len: {}, capacity {}", vector.len(), vector.capacity());

    let mut my_vector = vector_alex::new();
    assert_eq!(my_vector.len(), 0);
    assert_eq!(my_vector.capacity(), 0);
    my_vector.push(5);
    assert_eq!(my_vector.len(), 1);
    assert_eq!(my_vector.capacity(), 4);


}
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};

struct vector_alex<T>{
    len: usize,
    items: Option<Box<[T]>>
}

impl <T> vector_alex <T> {
    pub fn new() -> vector_alex<T> {
        vector_alex{len: 0, items: None}
    }
    pub fn len(&self) -> usize
    {
        self.len
    }
    pub fn capacity(&self) -> usize
    {
        if let Some(items) = &self.items{
            items.len()
        }else{
            0
        }
    }
    pub fn push(&mut self, item: T)
    {

        if self.len() == self.capacity(){
            self.reallloc();
        }

        if let Some(items) = &mut self.items{
            items[self.len] = item;
        }
    }
    fn reallloc(&mut self){
        unsafe {
            let layout = Layout::array(4).unwrap();
            let ptr = alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }

            *(ptr as *mut u16) = 42;
            assert_eq!(*(ptr as *mut u16), 42);

            dealloc(ptr, layout);
        }
    }


}


