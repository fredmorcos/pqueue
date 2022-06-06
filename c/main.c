// Build using:
//
//   cc -Wall -Wextra -L../target/debug -lpqueue main.c -o pqueue_test
//
//   cc -Wall -Wextra -L../target/release -lpqueue main.c -o pqueue_test
//
// Running:
//
//   LD_LIBRARY_PATH=../target/debug ./pqueue_test
//
//   LD_LIBRARY_PATH=../target/release ./pqueue_test
//
// With Valgrind (show correct memory access and management):
//
//   LD_LIBRARY_PATH=../target/debug valgrind ./pqueue_test
//
//   LD_LIBRARY_PATH=../target/release valgrind ./pqueue_test

#include "pqueue.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  const uint8_t elements[5] = {4, 2, 5, 1, 2};
  struct PQueueU8 *pq = pqueue_u8_new(elements, 5);
  if (pq == NULL) {
    fprintf(stderr, "failed to create queue\n");
    exit(EXIT_FAILURE);
  }

  enum PQueueU8Status stat = pqueue_u8_push(pq, 17);
  if (stat != PQueueU8Status_Success) {
    fprintf(stderr, "push failed: invalid argument\n");
  }

  for (int i = 0; i < 10; i++) {
    struct PQueueU8Value ret = pqueue_u8_pop(pq);
    switch (ret.status) {
    case PQueueU8Status_Success:
      printf("%d\n", ret.value);
      break;
    case PQueueU8Status_Empty:
      printf("Nothing\n");
      break;
    case PQueueU8Status_InvalidArgument:
      fprintf(stderr, "pop failed: invalid argument\n");
      exit(EXIT_FAILURE);
    }
  }

  pqueue_u8_free(pq);
}
