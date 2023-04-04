#include <stdlib.h>
#include <stdbool.h>

// amazing strategy intead of
// using void* and casting
typedef struct sofia_app application;
#define NUA_MAGIC_T   application

#include <sofia-sip/nua.h>
#include <sofia-sip/su.h>
#include <sofia-sip/su_tag_inline.h>
#include "sofia_app.h"


struct sofia_app {
  su_home_t home;
  su_root_t *root;
  nua_t *nua;
  sofia_app_handle_incoming_cb *handle_incoming;
  void *handle_incoming_user_data;

  char *bindurl;
};


static sofia_app_tags_t sofia_app_tl(sofia_app_t *app, tagi_t const tags[]) {
  sofia_app_tags_t head = (sofia_app_tag_t*)su_alloc(&app->home, sizeof(sofia_app_tag_t));
  sofia_app_tag_t *next = NULL;
  for(; tags; tags = tl_next(tags)) {
    if (!next) {
      next = head;
      next->next = NULL;
    } else {
      next->next = (sofia_app_tag_t *)su_alloc(&app->home, sizeof(sofia_app_tag_t));
      next = next->next;
      next->next = NULL;
    }
    
    //TAKED: su_taglist.c
    tag_type_t tt = TAG_TYPE_OF(tags);
    next->ns = su_strdup(&app->home, tt->tt_ns ? tt->tt_ns : "");
    next->name = su_strdup(&app->home, tt->tt_name ? tt->tt_name : "null");

    char buffer[4096];
    buffer[0] = '\0';
    if (tt->tt_snprintf)
      tt->tt_snprintf(tags, buffer, sizeof(buffer));
    else
      snprintf(buffer, sizeof(buffer), "%llx", (long)tags->t_value);
    next->value = su_strdup(&app->home, buffer);
  }

  return head;
}

void sofia_app_nua_callback(
                            nua_event_t event,
                            int status,
                            char const *phrase,
                            nua_t *nua,
                            nua_magic_t *app,
                            nua_handle_t *nh,
                            nua_hmagic_t *hmagic,
                            sip_t const *sip,
                            tagi_t tags[])
{

  if (app->handle_incoming) {
    char const *event_name = nua_event_name(event);
    sofia_app_tag_t *exported_tags = sofia_app_tl(app, tags);

    app->handle_incoming(event, event_name, status, phrase, exported_tags, app->handle_incoming_user_data);
    su_free(&app->home, exported_tags);
  }
}

int sofia_app_check() {
  su_home_t home;
  su_init();
  su_home_init(&home);
  su_home_deinit(&home);
  su_deinit();
  return 0;
}

// manipulation
void sofia_app_iterate(sofia_app_t *app, long interval_us) {
  su_root_sleep(app->root, interval_us);
}

// allocation
sofia_app_t *sofia_app_create() {
  sofia_app_t *app = (sofia_app_t*) malloc(sizeof(sofia_app_t));
  app->bindurl = NULL;
  app->handle_incoming = NULL;
  return app;
}

bool sofia_app_init(sofia_app_t *app,
                    const char *bindhost,
                    int bindport,
                    sofia_app_handle_incoming_cb *handle_incoming,
                    void *handle_incoming_user_data) {
  su_init();
  su_home_init(&app->home);
  app->root = su_root_create(app);
  if (!app->root)
    return false;

  app->handle_incoming = handle_incoming;
  app->handle_incoming_user_data = handle_incoming_user_data;
  app->bindurl = su_sprintf(&app->home, "sip:%s:%d", bindhost, bindport);
  app->nua = nua_create(app->root,
                        sofia_app_nua_callback,
                        app,
                        NUTAG_URL(app->bindurl),
                        TAG_NULL());
  if (!app->nua)
    return false;

  nua_set_params(app->nua, TAG_NULL());

  return true;
}

void sofia_app_deinit(sofia_app_t *app) {
  if (app->nua) {
    nua_shutdown(app->nua);
    // we wait for event shutdown
    su_root_sleep(app->root, 100);
    nua_destroy(app->nua);
  }

  su_root_destroy(app->root);
  app->root = NULL;

  su_home_deinit(&app->home);
  su_deinit();
}

void sofia_app_destroy(sofia_app_t **app) {
  if (*app) {
    free(*app);
    *app = NULL;
  }
}
