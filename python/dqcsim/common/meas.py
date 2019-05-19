"""Contains a class wrapper for `Measurement` objects."""

import dqcsim._dqcsim as raw
from dqcsim.common.arb import ArbData
from dqcsim.common.handle import Handle
import re

_ident_re = re.compile(r'[a-zA-Z0-9_]+')

class Measurement(ArbData):
    """Represents a measurement object.

    Measurement objects consist of an integer referencing the qubit that was
    measured and a measurement value. The value can be 0, 1, or None
    (= undefined).

    Measurements also have an attached `ArbData` object. This is modelled in
    Python through inheritance.
    """

    def __init__(self, qubit, value, *args, **kwargs):
        """Constructs a Measurement object.

        The first two positional arguments are the qubit reference and the
        measurement value. The remaining positional arguments and the keyword
        arguments are used to construct the attached `ArbData` object.
        """
        super().__init__(*args, **kwargs)
        self.qubit = qubit
        self.value = value

    @property
    def qubit(self): #@
        """The qubit associated with this measurement."""
        return self.__qubit

    @qubit.setter
    def qubit(self, qubit): #@
        qubit = int(qubit)
        if qubit < 1:
            raise ValueError('invalid qubit reference: {!r}'.format(qubit))
        self.__qubit = qubit

    @property
    def value(self): #@
        """The measurement value; either 0, 1, or None (= undefined)."""
        return self.__value

    @value.setter
    def value(self, value): #@
        if value is not None:
            value = int(bool(value))
        self.__value = value

    def __eq__(self, other):
        if isinstance(other, Measurement):
            return super().__eq__(other) and self.qubit == other.qubit and self.value == other.value
        return False

    @classmethod
    def _from_raw(cls, handle): #@
        """Constructs a measurement object from a raw API handle."""
        arg = ArbData._from_raw(handle)
        with handle as hndl:
            value = raw.dqcs_meas_value_get(hndl)
            if value == raw.DQCS_MEAS_UNDEFINED:
                value = None
            elif value == raw.DQCS_MEAS_ZERO:
                value = 0
            elif value == raw.DQCS_MEAS_ONE:
                value = 1
            else:
                assert(False)
            meas = Measurement(raw.dqcs_meas_qubit_get(hndl), value)
        meas._args = arg._args
        meas._json = arg._json
        return meas

    def _to_raw(self):
        """Makes an API handle for this measurement object."""
        value = self.value
        if value is None:
            value = raw.DQCS_MEAS_UNDEFINED
        elif value == 0:
            value = raw.DQCS_MEAS_ZERO
        elif value == 1:
            value = raw.DQCS_MEAS_ONE
        else:
            assert(False)
        handle = Handle(raw.dqcs_meas_new(self.qubit, value))
        super()._to_raw(handle)
        return handle

    def __repr__(self):
        e = [repr(self.qubit), repr(self.value)]
        for arg in self._args:
            e.append(repr(arg))
        for key, value in sorted(self._json.items()):
            e.append("{!s}={!r}".format(key, value))
        return "Measurement({})".format(', '.join(e))

    __str__ = __repr__

