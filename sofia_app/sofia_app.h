#pragma once

#include <stdbool.h>

struct sofia_app_tag {
  char *ns;
  char *name;
  char *value;
  struct sofia_app_tag *next;
};
typedef struct sofia_app_tag sofia_app_tag_t;

typedef sofia_app_tag_t *sofia_app_tags_t;

typedef struct sofia_app sofia_app_t;

typedef void sofia_app_handle_incoming_cb(
                                          int event,
                                          char const *event_name,
                                          int status,
                                          char const *phrase,
                                          sofia_app_tag_t const *tags,
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
