#include <sofia-sip/su.h>
#include <sofia-sip/su_alloc.h>

#include "sofia_app.h"

int sofia_app_check() {
  su_home_t home;
  su_init();
  su_home_init(&home);
  su_home_deinit(&home);
  su_deinit();
  return 0;
}
