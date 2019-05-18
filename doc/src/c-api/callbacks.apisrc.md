# Callbacks

In some places you can pass callbacks to the API. This is particularly
important for defining plugins: the callbacks ultimately define all your
plugin's functionality!

Depending on the callback, it may be called from a different thread than
the one you configured it with. This is clearly documented along with the
callback setter function and normally doesn't cause problems, but you should
keep it in mind.

In order to support closures in higher-level languages, all callback
setters take an optional cleanup callback and a `void*` to a piece of user
data. The cleanup callback is intended for cleaning up this user data if
necessary; it is called when DQCsim drops all references to the primary
callback, so it is guaranteed that the primary callback is never called
again when the cleanup. It is also guaranteed that the cleanup callback
is executed exactly once (unless the process dies spectacularly, in which
case it may not be called). However, very few guarantees are made about
which thread the cleanup callback is called from! If you use it, make sure
that it is thread-safe.
