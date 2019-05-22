import dqcsim._dqcsim as raw
from dqcsim.common.handle import Handle
from dqcsim.common.meas import Measurement

class MeasurementSet(object):
    @classmethod
    def _from_raw(cls, handle): #@
        with handle as hndl:
            measurements = []
            while raw.dqcs_mset_len(hndl) > 0:
                measurements.append(Measurement._from_raw(Handle(raw.dqcs_mset_take_any(hndl))))
        measurements.sort(key=lambda x: x.qubit)
        return measurements

    @classmethod
    def _to_raw(cls, *measurements): #@
        if len(measurements) == 1 and not isinstance(measurements[0], Measurement):
            measurements = measurements[0]
        handle = Handle(raw.dqcs_mset_new())
        qubits = set()
        with handle as hndl:
            for measurement in measurements:
                if not isinstance(measurement, Measurement):
                    raise TypeError("Expected Measurement object")
                qubit = measurement.qubit
                if qubit in qubits:
                    raise ValueError("Multiple measurement defined for qubit {}".format(qubit))
                qubits.add(qubit)
                with measurement._to_raw() as meas:
                    raw.dqcs_mset_set(hndl, meas)
        return handle
