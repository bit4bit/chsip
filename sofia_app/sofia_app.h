#pragma once

#include <stdbool.h>

typedef struct sofia_app sofia_app_t;

struct sofia_app_details {
  sofia_app_t *app;
  char *bindurl;
};
typedef struct sofia_app_details sofia_app_details_t;

int sofia_app_check();

sofia_app_details_t *sofia_app_details_create(sofia_app_t *app);
void sofia_app_details_set_bindhost(sofia_app_details_t *app, const char *host, int port);

sofia_app_t *sofia_app_create();
bool sofia_app_init(sofia_app_t *app, sofia_app_details_t *details);
void sofia_app_iterate(sofia_app_t *app, long interval_us);
void sofia_app_deinit(sofia_app_t *app);
void sofia_app_destroy(sofia_app_t **app);
