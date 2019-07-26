# whitespace-rs

[whitespace language](https://en.wikipedia.org/wiki/Whitespace_%28programming_language%29) interpreter, in Rust.

It's an [esoteric](https://en.wikipedia.org/wiki/Esoteric_programming_language) stack-based language with 24 instructions that only use `tab`, `space` and `line feed`. Think assembly, but with a smaller instruction set, and with unreadable characters.

Unfortunately the original language page is dead, but it still lives on [archive.org](https://web.archive.org/web/20150426193527/http://compsoc.dur.ac.uk/whitespace/tutorial.php). A copy of the original Haskell implementation is copied inside the `docs/WSpace` directory

# Todo

 - [ ] write all the instructions in the virtual machine along with tests

      // stack
      - [x] Push(i32)
      - [x] Duplicate
      - [ ] CopyNth(i32)
      - [x] Swap
      - [x] Discard
      - [ ] Slide(i32)
      // arithmetic
      - [x] Add
      - [x] Sub
      - [x] Mul
      - [x] Div
      - [x] Mod
      // heap
      - [x] Store
      - [x] Retrieve
      // flow control
      - [x] SetLabel(String)
      - [x] CallSubroutine(String)
      - [x] Jump(String)
      - [x] JZero(String)
      - [x] JNeg(String)
      - [x] EndOfSubroutine
      - [x] EndOfProgram
      // I/O -> need a better implementation that allows for tests
      - [-] PrintChar
      - [-] PrintInt
      - [ ] ReadChar
      - [ ] ReadInt

 - [ ] make the VM able to use different streams for I/O (easier testing)
 - [ ] parser: fix the crash due to out of bounds exceptions
 - [ ] automated tests for every critical branch of the parser
 - [ ] add some property-based tests in the parser
 - [ ] automated tests for every critical branch of the virtual machine
 - [ ] ensure every example program can run
 - [ ] use a more strict data type in the parser (in order to only allow space/tabs/lf at compile time)
 - [ ] simplify/shorten parse_instruction, many things are repeated there
 - [ ] functionnal tests for the parser that use real programs
 - [ ] write some documentation
 - [ ] cleanup the copies and unwrap as much as possible
 - [x] cleanup this mess with mutable/immutable borrows in `run_instruction`
 - [ ] implement arbitrary precision integers ?
 - [ ] turns this into a compiler because why not ?

# License

The examples in the examples directory, the tutorial.html and some elements of documentation come straight from the original haskell implementation that can be downloaded on the wayback machine.

This rust implementation, in the `src` directory, is MIT licensed.

