# whitespace-rs

[whitespace language](https://en.wikipedia.org/wiki/Whitespace_%28programming_language%29) interpreter, in Rust.

It's an [esoteric](https://en.wikipedia.org/wiki/Esoteric_programming_language) stack-based language with 24 instructions that only use `tab`, `space` and `line feed`. Think assembly, but with a smaller instruction set, and with unreadable characters.

Unfortunately the original language page is dead, but it still lives on [archive.org](https://web.archive.org/web/20150426193527/http://compsoc.dur.ac.uk/whitespace/tutorial.php). A copy of the original Haskell implementation is copied inside the `docs/WSpace` directory

# Goal

 - to have a fully functionnal whitespace interpreter that can run all the available programs
 - self sufficient: no external dependencies for parsing and virtual machine

Ideally, it would deal with arbitrary sized integer (that's what the original Haskell implementation feature), but right now only regular i32 are used.

# Usage

It can run a whitespace program :

      $ cargo run --bin interpreter -- examples/fact.ws

It can turn a whitespace source file into a rust source file that can the be compiled and run (actually it's more of a transpiler):

      $ cargo run --bin compiler -- examples/count2.ws src/bin/count.rs && cargo run --bin count

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

 - [ ] make the VM able to use different streams for I/O (in order to ease testing)
 - [ ] implement arbitrary precision integers ?
 - [x] turns this into a compiler because why not ? (by generating a rust source file that uses its own virtual machine ?)

## side stuff that would be nice to cleanup

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
 - [x] cleanup this mies and unwrap as much as possible
 - [ ] test for empty program (or more generally, program with no end of program. Checking for reachability is… not a solved problem though)

# License

The examples in the examples directory, the tutorial.html and some elements of documentation come straight from the original haskell implementation that can be downloaded on the wayback machine.

This rust implementation, in the `src` directory, is MIT licensed.

