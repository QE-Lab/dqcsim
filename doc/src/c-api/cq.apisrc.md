# ArbCmd queues

Some interfaces allow multiple commands to be specified. This is done through
command queue objects.

## Constructing a command queue

To construct a command queue, create a handle using `dqcs_cq_new()`, and then
push `ArbCmd` objects into it one by one using `dqcs_cq_push()`. To keep the
API simple, it is not possible to insert by index or override previously added
commands

@@@c_api_gen ^dqcs_cq_new$@@@
@@@c_api_gen ^dqcs_cq_push$@@@

## Iterating over a command queue

Command queues can be iterated over as follows (note, only once!):

```C
dqcs_handle_t queue = ...;
for (; dqcs_cq_len(queue) > 0; dqcs_cq_next(queue)) {
    ...
}
```

Within the loop body, the `queue` variable can be used as if it's an `ArbCmd`
handle. The iteration happens in the same order in which `dqcs_cq_push()` was
called.

@@@c_api_gen ^dqcs_cq_len$@@@
@@@c_api_gen ^dqcs_cq_next$@@@
