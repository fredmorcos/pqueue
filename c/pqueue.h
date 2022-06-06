#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum PQueueU8Status {
  Success = 0,
  Empty = 1,
  InvalidArgument = -1,
} PQueueU8Status;

typedef struct PQueueU8 PQueueU8;

typedef struct PQueueU8Value {
  enum PQueueU8Status status;
  uint8_t value;
} PQueueU8Value;

struct PQueueU8 *pqueue_u8_new(const uint8_t *elements, uintptr_t len);

enum PQueueU8Status pqueue_u8_push(struct PQueueU8 *pqueue, uint8_t element);

struct PQueueU8Value pqueue_u8_pop(struct PQueueU8 *pqueue);

void pqueue_u8_free(struct PQueueU8 *_pqueue);
