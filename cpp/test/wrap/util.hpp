#pragma once

#define EXPECT_ERROR(x, msg)                    \
  try {                                         \
    x;                                          \
    FAIL() << "Expected std::runtime_error";    \
  }                                             \
  catch(std::runtime_error const & err) {       \
    EXPECT_EQ(err.what(), std::string(msg));    \
  }                                             \
  catch(...) {                                  \
    FAIL() << "Expected std::runtime_error";    \
  }

#define EXPECT_INV_ARG(x, msg)                  \
  try {                                         \
    x;                                          \
    FAIL() << "Expected std::invalid_argument"; \
  }                                             \
  catch(std::invalid_argument const & err) {    \
    EXPECT_EQ(err.what(), std::string(msg));    \
  }                                             \
  catch(...) {                                  \
    FAIL() << "Expected std::invalid_argument"; \
  }
