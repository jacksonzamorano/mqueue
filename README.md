# MQueue
## Simple FIFO queue that supports mutating in iteration.

## General Usage
This is the simplest way to use a MQueue.
```rust
    let mut queue = WorkQueue::<i32>::create();
    let mut sum = 0;
    queue.add(1);
    queue.add(2);
    queue.add(3);

    while let Some(data) = queue.next_value() {
        sum += data;
    }

    assert_eq!(sum, 6);
```
The loop will continue to give items until it reaches the end. Note that the alternative function `next_ref` may be used instead to obtain a reference to the object, instead of a `clone` of it.
As you can see, items may be enqueued while in the loop. No matter whether you use `next_ref` or `next_value`, the queue may be modified while in a loop.
```rust
    let mut queue = WorkQueue::<i32>::create();
    let mut sum = 0;
    queue.add(1);
    queue.add(2);
    queue.add(3);

    while let Some(data) = queue.next_ref() {
        sum += data;
        if sum == 6 {
            queue.add(4);
        }
    }
```
## Cleaning
MQueue works by keeping a current index to your current item. That means that elements remain in memory until manually cleared with `clean()`. Manually clean the queue when you're done with it or at any point where it makes sense to. This operation is O(n), while accessing current or next is O(1). You can get how many items that are consumed but not cleaned by calling `dirty_count()`.

## Remaining
You can access the number of remaining items by calling `queue.remaining()`. This returns an `usize`.
