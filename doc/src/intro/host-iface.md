# The host interface

The host interface, also known as simulator or accelerator interface, connects
the host (or DQCsim command-line interface) to the frontend. Especially from
the perspective of the host, it intends to be as generic as possible –
sufficiently generic, even, to allow for drop-in replacements with a real
quantum computer control system once it becomes available. The following
graphic shows the functions and callbacks used to form the interface on either
side.

<p style="text-align: center"><img src="host-iface.svg" /></p>

## Algorithm execution

It is assumed that the quantum accelerator can only execute one algorithm at a
time; that is, it is single-threaded. However, multiple algorithms can be run
sequentially within the context of a simulation. It's also possible to control
multiple parallel quantum accelerators from a single program by simply
initializing DQCsim multiple times from different threads,

The host starts an algorithm by calling `start()`. This function takes an
`ArbData` as an argument, which may for instance be used to select which
algorithm to run for microarchitectures that allow multiple to be loaded in
memory at the same time. This call is asynchronous; that is, it requests that
the accelerator starts running, but does not wait for it to complete. Instead,
this waiting has to be done explicitly through a call to wait `wait()`. This
allows the quantum accelerator to run in parallel to the classical logic in
the host program, even though the quantum accelerator itself is
single-threaded.

Algorithm execution is modeled by means of a single callback on the frontend
side. This callback takes the `ArbData` passed to `start()` as an argument. It
also returns an `ArbData`; this response is returned to the host through the
return value of `wait()`.

## Communication

While both the host and the quantum accelerator are running in parallel, they
can communicate with each other through two queues, one in either direction.
These queues carry `ArbData` objects as packets. The `send()` function
asynchronously pushes a message into the respective queue, while the `recv()`
returns a message from the queue. `recv()` will block until a message is
available.

## Host arbs

In addition to the above, the host can send `ArbCmd`s to the accelerator. These
operate like synchronous remote procedure calls, taking an `ArbCmd` as argument
and sending an `ArbData` or error message in response.

This mechanism can for instance be used to model device memory access, or to
query implementation-specific information.
