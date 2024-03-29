
/*
선형 큐 
-  삽입과 삭제 연산이 선입선출 FIFO로 이뤄지는 자료구조 입니다.
-  스택과 다르게 먼저 들어온 데이터가 먼저 나갑니다.
-  그래서 삽입과 삭제가 양방향에서 이루어집니다.

rear && front

When implementing a queue, Front and Rear always refer to the indices immediately in the front and rear. Initially, both Front and Rear exist at 0 or -1 (to distinguish an empty queue). When adding data, Rear increases gradually, and when removing data, Front moves forward gradually. You can easily understand this concept by looking at the illustration above.

Adding a new value is done at the rear of the queue, and removal is done at the front of the queue.

Rear: The area in the queue that points to the last data.

Front: The area in the queue that points to the frontmost data.

The drawback of a linear queue is that when an element is removed, the space is left empty, and it requires the cumbersome task of shifting the data.
 */



struct Queue {
    arr: Vec<i32>,  // Queue elements stored in a vector
    capacity: usize,  // Maximum capacity of the queue
    front: usize,  // Index pointing to the front element (if present)
    rear: usize,   // Index pointing to the rear element (if present)
    count: usize,  // Current number of elements in the queue
}

impl Queue {
    //큐 생성
    fn new(size: usize) -> Self {
        Queue {
            arr: vec![0; size],
            capacity: size,
            front: 0,
            rear: size - 1,
            count: 0,
        }
    }
 // Insert a new element at the rear of the queue.
// When trying to add data to the queue, check if the queue is full before proceeding.
    fn enqueue(&mut self, item: i32) {
        // Check for queue overflow
        if self.is_full() {
            println!("Overflow\nProgram Terminated");
            std::process::exit(1);
        }

        println!("Inserting {}", item);

        // Increment the rear index and wrap around if necessary
        self.rear = (self.rear + 1) % self.capacity;
        self.arr[self.rear] = item;
        self.count += 1;
    }
// Remove and return the front element of the queue.
fn dequeue(&mut self) -> Option<i32> {
        // Check for queue underflow
        if self.is_empty() {
            println!("Underflow\nProgram Terminated");
            std::process::exit(1);
        }

        let x = self.arr[self.front];
        println!("Removing {}", x);

        // Increment the front index and wrap around if necessary
        self.front = (self.front + 1) % self.capacity;
        self.count -= 1;

        Some(x)
    }  
// Return the front element of the queue.
fn peek(&self) -> Option<i32> {
        if self.is_empty() {
            println!("Underflow\nProgram Terminated");
            std::process::exit(1);
        }

        Some(self.arr[self.front])
    }
   // Total number of elements in the queue.
   fn size(&self) -> usize {
        self.count
    }
   // Check if the queue is empty.
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
   // Check if the queue is full.
    fn is_full(&self) -> bool {
        self.size() == self.capacity
    }
}

pub fn main() {
    // Create a queue with a capacity of 5
    let mut q = Queue::new(5);
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);

    println!("The front element is {:?}", q.peek());
    q.dequeue();

    q.enqueue(4);
    println!("The front element is {:?}", q.peek());

    println!("The queue size is {}", q.size());

    q.dequeue();
    q.dequeue();
    q.dequeue();

    if q.is_empty() {
        println!("The queue is empty");
    } else {
        println!("The queue is not empty");
    }
}






