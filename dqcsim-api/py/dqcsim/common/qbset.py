import dqcsim._dqcsim as raw
from dqcsim.common.handle import Handle

class QbSet(object):
    @classmethod
    def from_raw(cls, handle):
        with handle as hndl:
            qubits = []
            try:
                while True:
                    qubits.append(raw.dqcs_qbset_pop(hndl))
            except RuntimeError:
                pass
        return qubits

    @classmethod
    def to_raw(cls, *qubits):
        if len(qubits) == 1 and not isinstance(qubits[0], int):
            qubits = qubits[0]
        handle = Handle(raw.dqcs_qbset_new())
        with handle as hndl:
            for qubit in qubits:
                raw.dqcs_qbset_push(hndl, qubit)
        return handle
