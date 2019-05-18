# Memory management

For the most part, the API will handle memory allocation for you through
the handle system. However, it is usually up to you to free memory back up.
Failing to do this will usually not cause errors, but will adversely affect
memory usage and performance.

To prevent such memory leaks, pay close attention to the documentation of the
API calls you make. Most importantly, strings returned by DQCsim almost
always have to be deallocated by you through `free()`. The only exception to
that is `dqcs_error_get()`. You should also make sure that you delete handles
that you no longer need through `dqcs_handle_delete()`, though most of the
time DQCsim does this for you when you use a handle.
