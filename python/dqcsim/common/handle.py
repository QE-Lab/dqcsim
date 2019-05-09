import dqcsim._dqcsim as raw

class Handle(object):
    """Wrapper for DQCsim API handles that automatically deallocates the
    underlying handle when it is garbage collected."""
    def __init__(self, handle=0):
        super().__init__()
        self._handle = int(handle)

    def __bool__(self):
        """Returns whether the handle is valid."""
        return self._handle is not None and self._handle > 0

    def __int__(self):
        """Returns the handle as an integer.

        NOTE: it's dangerous to use this! If you the int() call used the last
        reference to this Handle, which happens when the Handle is created in
        the same expression where it is converted, the API handle will be
        deleted by __del__ as soon as int() returns! Use the "with" syntax
        instead to make sure that this cannot happen."""
        if self._handle is None:
            raise ValueError("Using explicitly taken handle!")
        return self._handle

    def take(self):
        """Retakes ownership of the underlying API handle. The integer handle
        is returned, and __del__ won't try to delete it."""
        if self._handle is None:
            raise ValueError("Using explicitly taken handle!")
        handle = self._handle
        self._handle = None
        return handle

    def get_type(self):
        """Returns the type code for the handle from the API."""
        if self._handle is None:
            raise ValueError("Using explicitly taken handle!")
        return raw.dqcs_handle_type(self._handle)

    def is_type(self, typ):
        """Checks this handle's type code."""
        return self.get_type() == typ

    def __enter__(self):
        """`with` syntax for borrowing the underlying API handle.

        Syntax:

            with Handle(...) as h:
                raw.dqcs_...(..., h, ...)

        This ensures that the Handle wrapper cannot deleted by Python until the
        `with` block is completed, because the `with` block takes a reference
        to it to be able to call `__exit__`. It's also a convenient way to get
        the integer handle, so you don't have to write `int(h)` everywhere."""
        return int(self)

    def __exit__(self, type, value, tb):
        pass

    def __del__(self):
        """Tries it's best to delete the underlying API handle, if any. It's
        fine if the user or the API itself has already done this (it's a small
        performance hit, but that's all)."""
        if hasattr(self, '_handle'):
            if self:
                try:
                    raw.dqcs_handle_delete(self._handle)
                except RuntimeError:
                    # We're probably being called from the generational garbage
                    # collector (a different thread). Nothing we can do about it.
                    # We have to leak the handle.
                    pass

    def __repr__(self):
        return "Handle({})".format(self._handle)

    def __str__(self):
        if self:
            try:
                return raw.dqcs_handle_dump(self._handle)
            except RuntimeError:
                return "Handle({}?)".format(self._handle)
        else:
            return "Handle({})".format(self._handle)

