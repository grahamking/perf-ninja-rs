In the C++ version `qsort` is a library function in glibc, which is dynamically linked. The compiler can't inline `compare` into it. The lab asks you to figure out how to fix this.

Rust however is statically linked, and `compare` is inlined already. Nothing really to do here, other than appreciate static linking.
