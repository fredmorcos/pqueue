// Build using:
//
//   c++ -Wall -Wextra target/debug/libmylib.a -o tests/test_cxx
//
//   c++ -Wall -Wextra target/release/libmylib.a -o tests/test_cxx
//
// Running:
//
//   ./tests/test_auto
//
// With Valgrind (to show that really memory has been freed properly):
//
//   valgrind ./tests/test_auto

#include <array>
#include <cstdint>
#include <iostream>
#include <pqueue/src/lib.rs.h>
#include <rust/cxx.h>

int main() {
  std::array<const uint8_t, 5> array{4, 2, 5, 1, 2};
  rust::Slice<const uint8_t> elements{array.data(), array.size()};
  rust::Box<PQueueU8> pq = pqueue_u8_new(elements);

  pq->push(17);

  for (int i = 0; i < 10; i++) {
    try {
      const uint8_t value = pq->pop();
      std::cout << std::to_string(value) << std::endl;
    } catch (const rust::Error &e) {
      std::cerr << "pop error: " << e.what() << std::endl;
    }
  }
}
