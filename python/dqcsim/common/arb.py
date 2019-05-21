"""Contains a class wrapper for `ArbData` objects."""

import copy
import dqcsim._dqcsim as raw
from dqcsim.common.handle import Handle
import cbor

def _check_json(ob):
    try:
        cbor.dumps(ob)
    except ValueError:
        raise TypeError("Invalid JSON/CBOR object: {!r}".format(ob))

class ArbData(object):
    """Represents an ArbData object.

    ArbData objects are used to transfer user-specified data from one DQCsim
    plugin to another. They are composed of a CBOR-like object (superset of
    JSON) and a list of binary strings. The Python interface to DQCsim wraps
    ArbData objects in a class that behaves like a list and a dict at the same
    time; the list containing the binary strings and the dict containing the
    toplevel JSON/CBOR object entries. The class enforces that the binary
    string list entries implement Python's buffer protocol (bytes, bytearray,
    etc.) and that the JSON/CBOR entries are serializable as such using the
    cbor library.
    """

    def __init__(self, *args, **kwargs):
        """Constructs an ArbData object.

        The positional arguments are used to construct the binary argument
        list. They must therefore be binary strings or buffers. The keyword
        arguments are used to construct the JSON data. For instance:

            ArbData(b"test1", b"test2", answer=42)

        constructs an ArbData object with JSON `{"answer": 42}` and arguments
        `[b"test1", b"test2"]`. You can also pass an `ArbData` object as the
        sole argument, in which case a copy will be made.
        """
        super().__init__()
        if len(args) == 1 and not kwargs and isinstance(args[0], ArbData):
            self._args = copy.deepcopy(args[0]._args)
            self._json = copy.deepcopy(args[0]._json)
        else:
            for arg in args:
                memoryview(arg)
            self._args = list(args)
            _check_json(kwargs)
            self._json = kwargs

    def __bool__(self):
        """Returns whether there is non-default data in this ArbData object."""
        return bool(self._args) or bool(self._json)

    def __len__(self):
        """Returns the number of binary string arguments."""
        return len(self._args)

    def __getitem__(self, key):
        """Returns the binary string at the given index if key is numeric, or
        the JSON sub-object associated with the given key if key is a
        string."""
        if isinstance(key, str):
            return self._json[key]
        else:
            return self._args[key]

    def __setitem__(self, key, value):
        """Sets the binary string at the given index if key is numeric, or
        sets the JSON sub-object associated with the given key if key is a
        string."""
        if isinstance(key, str):
            _check_json(value)
            self._json[key] = value
        else:
            memoryview(value)
            self._args[key] = value

    def __delitem__(self, key):
        """Deletes the binary string at the given index if key is numeric, or
        deletes the JSON sub-object associated with the given key if key is a
        string."""
        if isinstance(key, str):
            del self._json[key]
        else:
            del self._args[key]

    def __contains__(self, item):
        """If key is a string, tests existence of the key in the toplevel JSON
        object. If it is a binary string, tests if it is one of the binary
        string arguments."""
        if isinstance(item, str):
            return item in self._json
        else:
            return item in self._args

    def __iter__(self):
        """Iterates over the binary arguments."""
        for arg in self._args:
            yield arg

    def append(self, value):
        """Appends a binary string to the list."""
        memoryview(value)
        self._args.append(value)

    def insert(self, index, value):
        """Inserts a binary string into the list."""
        memoryview(value)
        self._args.insert(index, value)

    def extend(self, it):
        """Extends the binary strings with the given iterator."""
        for value in it:
            self.append(value)

    def keys(self):
        """Iterates over the JSON object entry keys."""
        return self._json.keys()

    def values(self):
        """Iterates over the JSON object values."""
        return self._json.values()

    def items(self):
        """Iterates over the JSON object items."""
        return self._json.items()

    def clear_args(self):
        """Clears the binary argument list."""
        self._args = []

    def clear_json(self):
        """Clears the JSON data."""
        self._json = {}

    def clear(self):
        """Resets the ArbData object."""
        self.clear_args()
        self.clear_json()

    def __eq__(self, other):
        if isinstance(other, ArbData):
            return self._args == other._args and self._json == other._json
        return False

    @classmethod
    def _from_raw(cls, handle): #@
        """Constructs an ArbData object from a raw API handle."""
        # Load CBOR.
        with handle as hndl:
            cb = bytearray(256)
            cbl = raw.dqcs_arb_cbor_get(hndl, cb)
            if cbl > 256:
                cb = bytearray(cbl)
                raw.dqcs_arb_cbor_get(hndl, cb)
            kwargs = cbor.loads(cb)

            # Load binary arguments.
            args = []
            for i in range(raw.dqcs_arb_len(hndl)):
                arg = bytearray(256)
                argl = raw.dqcs_arb_get_raw(hndl, i, arg)
                if argl > 256:
                    arg = bytearray(argl)
                    raw.dqcs_arb_get_raw(hndl, i, arg)
                args.append(bytes(arg[:argl]))

        return ArbData(*args, **kwargs)

    def _to_raw(self, handle=None):
        """Makes an API handle for this ArbData object."""
        if handle is None:
            handle = Handle(raw.dqcs_arb_new())
        else:
            raw.dqcs_arb_clear(int(handle))
        with handle as hndl:
            raw.dqcs_arb_cbor_set(hndl, cbor.dumps(self._json))
            for arg in self._args:
                raw.dqcs_arb_push_raw(hndl, arg)
        return handle

    def __repr__(self):
        e = []
        for arg in self._args:
            e.append(repr(arg))
        for key, value in sorted(self._json.items()):
            e.append("{!s}={!r}".format(key, value))
        return "ArbData({})".format(', '.join(e))

    __str__ = __repr__

