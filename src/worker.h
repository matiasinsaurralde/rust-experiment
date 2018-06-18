#ifndef WORKER_H
#define WORKER_H
#ifdef __cplusplus
extern "C" {
#endif

struct worker_s;
typedef struct worker_s worker;

worker* worker_new();
void worker_set_rust_object(worker*, void*);

void trigger_callback(worker*, int);

void rust_callback(void*, int);

#ifdef __cplusplus
}  // extern "C"
#endif
#endif  // WORKER_H
