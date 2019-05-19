"""Contains a class wrapper for `ArbCmd` objects."""

import dqcsim._dqcsim as raw
from dqcsim.common.arb import ArbData
from dqcsim.common.handle import Handle
import re

_ident_re = re.compile(r'[a-zA-Z0-9_]+')

class ArbCmd(ArbData):
    """Represents an ArbCmd object.

    ArbData objects are used to transfer user-specified instructions from one
    DQCsim plugin to another. They consist of two identifying strings and an
    ArbData argument. In Python, the contained ArbData argument is represented
    using subclassing (so all ArbData operations also work on an ArbCmd).

    The two identifying strings are called the interface identifier and the
    operation identifier. Both strings must match exactly (case-sensitive) when
    matching an incoming ArbCmd against a handler. The difference between the
    two is that if the interface identifier is unrecognized the ArbCmd should
    be treated as no-op, whereas an error should be raised when the interface
    identifier is matched but the operation identifier is not.
    """

    def __init__(self, *args, **kwargs):
        """Constructs an ArbCmd object.

        The first two positional arguments are the interface identifier and the
        operation identifier. They must be valid identifiers, i.e. matching the
        regex /[a-zA-Z_0-9]+/. The remaining positional arguments and the
        keyword arguments are used to construct the `ArbData` argument.
        Alternatively, another `ArbCmd` object can be specified as the sole
        argument to make a copy.
        """
        super().__init__()
        if len(args) == 1 and not kwargs and isinstance(args[0], ArbCmd):
            super().__init__(args[0])
            self._iface = args[0]._iface
            self._oper = args[0]._oper
        elif len(args) >= 2:
            iface, oper, *args = args
            iface = str(iface)
            if not _ident_re.match(iface):
                raise ValueError('iface is not a valid identifier: {!r}'.format(iface))
            oper = str(oper)
            if not _ident_re.match(oper):
                raise ValueError('oper is not a valid identifier: {!r}'.format(oper))
            super().__init__(*args, **kwargs)
            self._iface = iface
            self._oper = oper
        else:
            raise TypeError('Invalid arguments passed to ArbCmd constructor')

    @property
    def iface(self): #@
        """The interface identifier."""
        return self._iface

    @property
    def oper(self): #@
        """The operation identifier."""
        return self._oper

    def __eq__(self, other):
        if isinstance(other, ArbCmd):
            return super().__eq__(other) and self._iface == other._iface and self._oper == other._oper
        return False

    @classmethod
    def _from_raw(cls, handle): #@
        """Constructs an ArbCmd object from a raw API handle."""
        arg = ArbData._from_raw(handle)
        with handle as hndl:
            cmd = ArbCmd(raw.dqcs_cmd_iface_get(hndl), raw.dqcs_cmd_oper_get(hndl))
        cmd._args = arg._args
        cmd._json = arg._json
        return cmd

    def _to_raw(self):
        """Makes an API handle for this ArbCmd object."""
        handle = Handle(raw.dqcs_cmd_new(self._iface, self._oper))
        super()._to_raw(handle)
        return handle

    def __repr__(self):
        e = [repr(self._iface), repr(self._oper)]
        for arg in self._args:
            e.append(repr(arg))
        for key, value in sorted(self._json.items()):
            e.append("{!s}={!r}".format(key, value))
        return "ArbCmd({})".format(', '.join(e))

    __str__ = __repr__

