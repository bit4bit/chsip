#pragma once

#include <stdbool.h>

typedef struct sofia_app sofia_app_t;

typedef void sofia_app_handle_incoming_cb(
                                          int event,
                                          int status,
                                          char const *phrase,
                                          void *user_data
                                          );
int sofia_app_check();

sofia_app_t *sofia_app_create();
bool sofia_app_init(
                    sofia_app_t *app,
                    const char *bindhost,
                    int bindport,
                    sofia_app_handle_incoming_cb *handle_incoming,
                    void *handle_incoming_user_data
                    );
void sofia_app_iterate(sofia_app_t *app, long interval_us);
void sofia_app_deinit(sofia_app_t *app);
void sofia_app_destroy(sofia_app_t **app);
