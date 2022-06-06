#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

struct PQueueU8;

enum PQueueU8Status {
  PQueueU8Status_Success = 0,
  PQueueU8Status_Empty = 1,
  PQueueU8Status_InvalidArgument = -1,
};

struct PQueueU8Value {
  enum PQueueU8Status status;
  uint8_t value;
};

struct PQueueU8 *pqueue_u8_new(const uint8_t *const elements, const size_t len);
enum PQueueU8Status pqueue_u8_push(struct PQueueU8 *const pqueue, const uint8_t element);
struct PQueueU8Value pqueue_u8_pop(struct PQueueU8 *const pqueue);
void pqueue_u8_free(struct PQueueU8 *pqueue);
