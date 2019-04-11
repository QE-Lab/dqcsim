import dqcsim._dqcsim as raw

class Handle(object):
    """Wrapper for DQCsim API handles that automatically deallocates the
    underlying handle when it is garbage collected."""
    def __init__(self, handle):
        self._handle = handle

    def __bool__(self):
        return self._handle is not None and self._handle > 0

    def __int__(self):
        if self._handle is None:
            raise ValueError("Using explicitly taken handle!")
        return self._handle

    def take(self):
        if self._handle is None:
            raise ValueError("Using explicitly taken handle!")
        handle = self._handle
        self._handle = None
        return handle

    def get_type(self):
        if self._handle is None:
            raise ValueError("Using explicitly taken handle!")
        return raw.dqcs_handle_type(self._handle)

    def is_type(self, typ):
        return self.get_type() == typ

    def __enter__(self):
        return int(self)

    def __exit__(self, type, value, tb):
        pass

    def __del__(self):
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

