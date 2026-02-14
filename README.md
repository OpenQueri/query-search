# Query Search Engine (Mini Project)

For word stemming (reducing words to their root), this project uses the **Snowball / libstemmer_c** library.  
The C++ source code has been included directly in this project for convenience when using it from Rust via FFI.  

❗ Note: The stemmer code is **not our own**, we are just wrapping it for use in our pipeline.  

Original Snowball project: [https://snowballstem.org/](https://snowballstem.org/)

---

## Usage

- Russian, Ukrainian, and English are supported via libstemmer_c.
- Each thread can have its own stemmer instance.
- For multithreaded search, use one stemmer per thread, or a single worker thread with a task queue.
