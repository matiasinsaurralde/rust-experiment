
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <string>
#include "worker.h"

struct worker_s {
  void* rust_object;
};

worker* worker_new() {
    worker* w = new (worker);
    printf("worker_new: worker = %p\n", w);
    return w;
};

void worker_set_rust_object(worker* w, void* rust_object) {
    printf("worker_set_rust_object: worker = %p rust_object = %p\n", w, rust_object);
    w->rust_object = rust_object;
};

void trigger_callback(worker* w, int n) {
    printf("trigger_callback: worker = %p rust_object = %p\n", w, w->rust_object);
    rust_callback(w->rust_object, n);
};