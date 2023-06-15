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

    let mut my_vector = VectorAlex::new();
    assert_eq!(my_vector.len(), 0);
    assert_eq!(my_vector.capacity(), 0);
    my_vector.push(5);
    assert_eq!(my_vector.get_item(0), 5);
    assert_eq!(my_vector.len(), 1);
    assert_eq!(my_vector.capacity(), 4);

    let upper_lim = 10000;
    for i in 1..upper_lim {
        println!("Index is {}", i);
        my_vector.push(i);
        assert_eq!(my_vector.get_item(i), i);
        assert_eq!(my_vector.len(), i + 1);
    }
  //  assert_eq!(my_vector.capacity(), 1024);

    for i in 1..upper_lim {
        println!("Index is {}", i);
        let value = my_vector.get_item(i);
        println!("value of index {} is {}", i, value);
    }
    for i in 1..upper_lim {
        println!("Index is {}", i);
        assert_eq!(my_vector.get_item(i), i);
    }
    // drop(my_vector);
}

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};


struct VectorAlex<T> {
    len: usize,
    cap: usize,
    items_1: *mut u8,
    phantom: std::marker::PhantomData<T>,

}

impl<T> Drop for VectorAlex<T>{
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.cap).unwrap();
       unsafe{ dealloc(self.items_1, layout) }
    }
}
impl<T: Default + Copy + Clone> VectorAlex<T> {
    pub fn new() -> VectorAlex<T> {
        VectorAlex {
            len: 0,
            cap: 0,
            items_1: std::ptr::null_mut(),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn len(&self) -> usize
    {
        self.len
    }
    pub fn capacity(&self) -> usize
    {
        self.cap
    }
    pub fn push(&mut self, item: T)
    {
        if self.len() == self.capacity() {
            self.reallloc();
        }

        self.set_item(item, self.len);
        self.len += 1;
    }
    fn set_item(&mut self, item: T, index: usize) {
        if std::ptr::null_mut() != self.items_1 {
            Self::set_item_ptr(self.items_1, item, index)
        }
    }
    fn set_item_ptr(ptr: *mut u8, item: T, index: usize) {
        unsafe {

            (ptr.offset((index * 8) as isize) as *mut T).write(item);
        };
    }
    pub fn get_item_ptr(ptr: *mut u8, index: usize) -> T {
        unsafe {
            (ptr.offset((index * 8) as isize) as *mut T).read()
        }
    }
    pub fn get_item(&self, index: usize) -> T {
        if std::ptr::null_mut() != self.items_1 {
            return Self::get_item_ptr(self.items_1, index);
        }
        Default::default()
    }
    fn reallloc(&mut self) {
        if std::ptr::null_mut() != self.items_1 {
            let next_cap = self.cap * 2;
            let layout = Layout::array::<T>(next_cap).unwrap();
            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            self.cap = next_cap;
            // copy prev arr
            for i in 0..self.len {
                let prev_val = Self::get_item_ptr(self.items_1, i);
                Self::set_item_ptr(ptr, prev_val, i);
            }
            unsafe {dealloc(self.items_1,  layout) };
            self.items_1 = ptr;
        } else {
            self.cap = 4;
            let layout = Layout::array::<T>(self.cap).unwrap();

            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            self.items_1 = ptr;
        }
    }
}




