use std::{collections::{HashMap, VecDeque}};

struct Colour(u8, u8, u8);

const HIT_MASK: u8 = 0b0011;
const GHOST_MASK: u8 = 0b0100;

// Should be templated with the payload type
struct CacheObject {
    ctrl: u8, // 0-2: hit counter, 2: ghost?
    data: Option<Colour> // Makes struct a neat 4 bytes.
}

struct S3Cache {
    small_capacity: usize,
    main_capacity: usize,
    small: VecDeque<u64>,
    main: VecDeque<u64>,
    ghost: VecDeque<u64>,
    data: HashMap<u64, CacheObject> // Includes ghost queue info, it's data is just None
}

impl S3Cache {
    // pub
    fn new(capacity: usize) -> Self {
        let small_capacity = capacity >> 3;
        let main_capacity = capacity - small_capacity;
        Self {
            small_capacity: small_capacity,
            main_capacity: main_capacity,
            small: VecDeque::with_capacity(small_capacity), // 8x smaller, paper recommends 10x
            main: VecDeque::with_capacity(main_capacity),
            ghost: VecDeque::with_capacity(main_capacity),
            data: HashMap::with_capacity(capacity + main_capacity) // stores the ghost data too
        }
    }

    fn read(&mut self, id: u64) -> Option<&Colour> {
        let Some(obj) = self.data.get_mut(&id) else {
            // Doesn't exist: insert to small
            return None;
        };
        
        if obj.ctrl & GHOST_MASK != 0 { // Should name the bitmasks as constants
            // Is in the ghost queue: insert to main
            return None;
        }

        if obj.ctrl & HIT_MASK < 3 { // Should replace this with a named constant
            obj.ctrl += 1;
        }

        obj.data.as_ref()
    }

    fn insert(&mut self, id: u64) -> Result<&CacheObject, ()> {
        // eviction loop to make room? Seems like we would only need to remove one value
        let Some(obj) = self.data.get_mut(&id) else {
            self.small.push_front(id);
            return Ok(self.data
                .entry(id)
                .or_insert(
                    CacheObject { ctrl: 0, data: None }
                )
            );
        };

        if obj.ctrl & GHOST_MASK == 0 {
            return Err(());
        }

        // Does not need to be removed from Ghost queue, we just check the id when we evict from
        // ghost and only count it if it the item has the flag set
        self.main.push_front(id);
        Ok(self.data.)
    }
}

struct Node {
    id: u64,
    data: S3Cache
    // S3 Queue struct
}

fn main() {
    println!("Hello, world!");

}
