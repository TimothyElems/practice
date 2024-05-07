# String v &str (string slice)

_In Rust, String and &str represent two different ways to work with strings:_
**String:**
- Owns the data: A String is a growable, owned string type. It holds the actual UTF-8 encoded data of the string on the heap.
- Mutable: You can modify the characters within a String.
- Less efficient for passing: Copying a String involves copying the entire data, which can be inefficient for frequently passed arguments.

**&str (pronounced "string slice")**
- Reference to existing data: An &str is a borrowed reference (immutable slice) to a piece of UTF-8 encoded data. It doesn't own the data itself, but points to an existing string somewhere in memory.
- Immutable: You cannot modify the characters within an &str.
More efficient for passing: Passing an &str is just passing a reference, so it's more efficient for function arguments and similar cases.

Here's an analogy:
- Imagine String as a book you own. You can write in it, add new pages, or throw it away (free the memory).
- Imagine &str as borrowing a friend's book. You can read it, but you can't write in it or change it.

When to use which:
- Use String when you need to own and modify the string data. This includes building strings at runtime, storing user input, or passing data between threads (since each thread needs its own copy).
- Use &str when you only need a read-only view of an existing string. This is common for function arguments, string literals, and working with slices of a larger string.

Additional points:
- Rust automatically converts between String and &str when necessary. For example, you can pass a String to a function expecting an &str argument.
- String implements several methods for manipulating the string data, while &str has limited functionality due to its immutability.
