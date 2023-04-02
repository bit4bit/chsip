#include <stdlib.h>
#include <stdbool.h>

// amazing strategy intead of
// using void* and casting
typedef struct sofia_app application;
#define NUA_MAGIC_T   application

#include <sofia-sip/nua.h>
#include <sofia-sip/su.h>

#include "sofia_app.h"

struct sofia_app {
  su_home_t home;
  su_root_t *root;
  nua_t *nua;
};


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

  printf("event %d: %03d %s\n", event, status, phrase);
  tl_print(stdout, "", tags);
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

sofia_app_details_t *sofia_app_details_create(sofia_app_t *app)
{
  sofia_app_details_t *details = (sofia_app_details_t *) su_alloc(&app->home, sizeof(sofia_app_details_t));
  details->bindurl = su_sprintf(&app->home, "sip:localhost:5060");
  return details;
}

void sofia_app_details_set_bindhost(sofia_app_details_t *details, const char *host, int port) {
  details->bindurl = su_sprintf(&details->app->home, "sip:%s:%d", host, port);
}

// allocation
sofia_app_t *sofia_app_create() {
  return (sofia_app_t*) malloc(sizeof(sofia_app_t));
}

bool sofia_app_init(sofia_app_t *app, sofia_app_details_t *details) {
  su_init();
  su_home_init(&app->home);
  app->root = su_root_create(app);
  if (!app->root)
    return false;

  const char *bindurl = "sip:localhost:5080";
  app->nua = nua_create(app->root,
                        sofia_app_nua_callback,
                        app,
                        NUTAG_URL(bindurl),
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

