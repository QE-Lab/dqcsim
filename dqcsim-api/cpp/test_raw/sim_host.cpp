#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"
#include "util.h"

using namespace dqcsim;

// Sanity-check.
TEST(sim_host, sanity) {
  SIM_HEADER;
  SIM_CONSTRUCT;
  SIM_FOOTER;
}

typedef struct {
  const char *signature;
  int counter;
} user_data_t;

dqcs_handle_t run_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t args) {
  char *s;

  ((user_data_t*)user_data)->counter++;

  // Handle error command.
  if (dqcs_arb_len(args) == 2) {
    s = dqcs_arb_get_str(args, 0);
    int cmp = strcmp(s, "return_error");
    if (s) free(s);
    if (!cmp) {
      s = dqcs_arb_get_str(args, 1);
      dqcs_handle_delete(args);
      dqcs_error_set(s);
      if (s) free(s);
      return 0;
    }
  }

  // Handle recv/send pair.
  if (dqcs_arb_len(args) == 2) {
    s = dqcs_arb_get_str(args, 0);
    int cmp = strcmp(s, "recv_send");
    if (s) free(s);
    if (!cmp) {
      dqcs_handle_t data = dqcs_plugin_recv(state);
      if (!data) {
        dqcs_handle_delete(args);
        return 0;
      }
      ((user_data_t*)user_data)->counter++;
      s = dqcs_arb_get_str(args, 1);
      dqcs_arb_push_str(data, s);
      if (s) free(s);
      if (dqcs_plugin_send(state, data) != dqcs_return_t::DQCS_SUCCESS) {
        dqcs_handle_delete(args);
        dqcs_handle_delete(data);
        return 0;
      }
    }
  }

  // Normal return: return the received ArbData with our user_data pointer
  // appended to it as a string.
  dqcs_arb_push_str(args, ((user_data_t*)user_data)->signature);
  return args;
}

// Start followed by wait, accelerator finishes.
TEST(sim_host, start_wait) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;
  MAKE_ARB(a, "{}", "a");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  a = dqcs_sim_wait(sim);
  CHECK_ARB(a, "{}", "a", "run() was here");

  SIM_FOOTER;

  EXPECT_EQ(ud.counter, 1);
}

// Start followed by wait, accelerator blocks (deadlock).
TEST(sim_host, start_wait_block) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;
  MAKE_ARB(a, "{}", "recv_send", "marker");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_sim_wait(sim), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Deadlock: accelerator is blocked on recv() while we are expecting it to return");

  SIM_FOOTER;

  EXPECT_EQ(ud.counter, 1);
}

// Start followed by another start.
TEST(sim_host, double_start) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;
  MAKE_ARB(a, "{}", "a");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  MAKE_ARB(a, "{}", "b");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: accelerator is already running; call wait() first");
  dqcs_handle_delete(a);

  a = dqcs_sim_wait(sim);
  CHECK_ARB(a, "{}", "a", "run() was here");

  SIM_FOOTER;

  EXPECT_EQ(ud.counter, 1);
}

// Start without wait, accelerator finishes.
TEST(sim_host, start_only) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;
  MAKE_ARB(a, "{}", "a");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  SIM_FOOTER;

  EXPECT_EQ(ud.counter, 1);
}

// Start without wait, accelerator blocks.
TEST(sim_host, start_only_block) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;
  MAKE_ARB(a, "{}", "recv_send", "marker");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  SIM_FOOTER;

  EXPECT_EQ(ud.counter, 1);
}

// Wait without start.
TEST(sim_host, wait_only) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  EXPECT_EQ(dqcs_sim_wait(sim), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: accelerator is not running; call start() first");

  SIM_FOOTER;

  EXPECT_EQ(ud.counter, 0);
}

// Double wait.
TEST(sim_host, start_wait_wait) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;
  MAKE_ARB(a, "{}", "a");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  a = dqcs_sim_wait(sim);
  CHECK_ARB(a, "{}", "a", "run() was here");

  EXPECT_EQ(dqcs_sim_wait(sim), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: accelerator is not running; call start() first");

  SIM_FOOTER;

  EXPECT_EQ(ud.counter, 1);
}

// Recv without start.
TEST(sim_host, recv_only) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  EXPECT_EQ(dqcs_sim_recv(sim), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Deadlock: recv() called while queue is empty and accelerator is idle");

  SIM_FOOTER;

  EXPECT_EQ(ud.counter, 0);
}

// Start followed by recv without matching send; deadlock.
TEST(sim_host, start_recv) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;
  MAKE_ARB(a, "{}", "a");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_sim_recv(sim), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Deadlock: accelerator exited before sending data");

  SIM_FOOTER;

  EXPECT_EQ(ud.counter, 1);
}

// Start, send, recv, wait.
TEST(sim_host, start_send_recv_wait) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;

  MAKE_ARB(a, "{}", "recv_send", "a");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  MAKE_ARB(a, "{}", "x");
  EXPECT_EQ(dqcs_sim_send(sim, a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(ud.counter, 0);

  a = dqcs_sim_recv(sim);
  CHECK_ARB(a, "{}", "x", "a");

  EXPECT_EQ(ud.counter, 2);

  a = dqcs_sim_wait(sim);
  CHECK_ARB(a, "{}", "recv_send", "a", "run() was here");

  SIM_FOOTER;
}

// Start, send, recv, wait. Additional yields.
TEST(sim_host, start_send_recv_wait_plus_yields) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;

  MAKE_ARB(a, "{}", "recv_send", "a");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(ud.counter, 0);

  EXPECT_EQ(dqcs_sim_yield(sim), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(ud.counter, 1);

  EXPECT_EQ(dqcs_sim_yield(sim), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(ud.counter, 1);

  MAKE_ARB(a, "{}", "x");
  EXPECT_EQ(dqcs_sim_send(sim, a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(ud.counter, 1);

  EXPECT_EQ(dqcs_sim_yield(sim), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(ud.counter, 2);

  EXPECT_EQ(dqcs_sim_yield(sim), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(ud.counter, 2);

  a = dqcs_sim_recv(sim);
  CHECK_ARB(a, "{}", "x", "a");

  a = dqcs_sim_wait(sim);
  CHECK_ARB(a, "{}", "recv_send", "a", "run() was here");

  SIM_FOOTER;
}

// Additional checks w.r.t. the message queues.
TEST(sim_host, message_queues) {
  SIM_HEADER;
  user_data_t ud = {"run() was here", 0};
  dqcs_pdef_set_run_cb(front, run_cb, NULL, &ud);
  SIM_CONSTRUCT;

  dqcs_handle_t a;

  MAKE_ARB(a, "{}", "x");
  EXPECT_EQ(dqcs_sim_send(sim, a), dqcs_return_t::DQCS_SUCCESS);

  MAKE_ARB(a, "{}", "y");
  EXPECT_EQ(dqcs_sim_send(sim, a), dqcs_return_t::DQCS_SUCCESS);

  MAKE_ARB(a, "{}", "z");
  EXPECT_EQ(dqcs_sim_send(sim, a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(ud.counter, 0);

  MAKE_ARB(a, "{}", "recv_send", "a");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(ud.counter, 0);

  a = dqcs_sim_wait(sim);
  CHECK_ARB(a, "{}", "recv_send", "a", "run() was here");

  EXPECT_EQ(ud.counter, 2);

  MAKE_ARB(a, "{}", "recv_send", "b");
  EXPECT_EQ(dqcs_sim_start(sim, a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(ud.counter, 2);

  a = dqcs_sim_recv(sim);
  CHECK_ARB(a, "{}", "x", "a");

  EXPECT_EQ(ud.counter, 2);

  a = dqcs_sim_recv(sim);
  CHECK_ARB(a, "{}", "y", "b");

  EXPECT_EQ(ud.counter, 4);

  a = dqcs_sim_wait(sim);
  CHECK_ARB(a, "{}", "recv_send", "b", "run() was here");

  EXPECT_EQ(ud.counter, 4);

  EXPECT_EQ(dqcs_sim_recv(sim), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Deadlock: recv() called while queue is empty and accelerator is idle");

  SIM_FOOTER;
}
