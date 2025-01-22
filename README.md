# Memalloc

`memalloc` is a Rust library that provides efficient heap memory allocation functionality. It allows users to dynamically allocate and deallocate memory, similar to C's `malloc`.

## Features

The library offers the following methods for memory management:

- **`memorize(size: usize) -> *mut u8`**  
  Allocates memory to hold `size` bytes and returns the starting address of the usable memory region.

- **`forget(address: *mut u8)`**  
  Frees the memory allocated at the provided address.

- **`rememorize(address: *mut u8, size: usize) -> *mut u8`**  
  Moves the contents of the given memory address to a new region with larger capacity, freeing the old block and returning the address of the newly allocated memory.

---

## Technical Details

`memalloc` is designed to maintain performance comparable to C's `malloc`, employing efficient algorithms to manage memory effectively.

### Current Features

- **Efficient System Calls:**  
  All new memory blocks are allocated using the `mmap` system call, ensuring proper memory alignment. Any sub-blocks within allocated regions also retain alignment properties.

- **Block Management:**  
  Memory blocks are maintained in two separate linked lists:  
  - **Allocated List:** Tracks all currently used memory blocks.  
  - **Free List:** Holds previously allocated blocks for potential reuse. Memory is returned to the OS only if it is the most recently allocated block; otherwise, it is retained in the free list to minimize system calls.

- **Thread Safety:**  
  All operations are safeguarded by synchronization mechanisms (e.g., mutexes), ensuring stability in both single-threaded and multi-threaded environments.

- **Block Splitting:**  
  When a free block is reused, an attempt is made to split it if the requested size allows. The requested memory is allocated as an aligned block, while the remaining portion is stored in the free list for future use.

- **Block Merging:**  
  Upon freeing a block, adjacent free blocks are merged if possible, reducing fragmentation and improving memory reuse efficiency.

---

## Planned Improvements

- **Memory Pooling:**  
  Future releases will introduce memory pooling to reduce the frequency of system calls. Instead of allocating memory on each request, the allocator will request larger chunks from the OS and manage them internally using splitting and merging mechanisms. Pool sizes will adapt dynamically based on allocation patterns.

- **C Language Integration:**  
  Plans include providing C bindings and comprehensive tests in C environments to ensure seamless compatibility and ease of use across different programming languages.

---

## Installation

To install and use `memalloc` in your Rust project:

1. Clone the repository:

   ```bash
   git clone https://github.com/AlexandruCostea/memalloc.git
   cd memalloc
   ```

2. Build the project:
  ```bash
    cargo build --release
  ```

3. Use the library in your Rust program by adding the following to your Cargo.toml:
   ```
   [dependencies]
   memalloc = { path = "/path/to/memalloc" }
   ```
