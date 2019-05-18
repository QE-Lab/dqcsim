# Error handling

Almost all API calls can fail, for instance because an invalid handle is
supplied. Since C does not support any kind of exceptions, such failures are
reported through the return value. Which value is used to indicate an error
depends on the return type; refer to the data type section for more
information. However, this value only indicates *something* went wrong, not
*what* went wrong. The following function can be used for that.

@@@c_api_gen ^dqcs_error_get$@@@

The mechanism for reporting errors to DQCsim from within a callback is the
same, but reversed: you first set the error string yourself, and then return
the value that indicates that an error occurred. You can set the error as
follows.

@@@c_api_gen ^dqcs_error_set$@@@

