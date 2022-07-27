TODO: src/init.rs:init is using Vec because I got a stack overflow using [S;N].
Is the original `std::array` on the stack or heap?

Either way we can get a speedup by eliminating compiler padding, as suggested in the lab intro.
