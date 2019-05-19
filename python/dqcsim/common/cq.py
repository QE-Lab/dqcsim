import dqcsim._dqcsim as raw
from dqcsim.common.handle import Handle
from dqcsim.common.cmd import ArbCmd

class ArbCmdQueue(object):
    @classmethod
    def _from_raw(cls, handle): #@
        with handle as hndl:
            cmds = []
            while raw.dqcs_cq_len(hndl):
                cmds.append(ArbCmd._from_raw(handle))
                raw.dqcs_cq_next(hndl)
        return cmds

    @classmethod
    def _to_raw(cls, *cmds): #@
        if len(cmds) == 1 and not isinstance(cmds[0], ArbCmd):
            cmds = cmds[0]
        handle = Handle(raw.dqcs_cq_new())
        with handle as hndl:
            for arg in cmds:
                if not isinstance(arg, ArbCmd):
                    raise TypeError("Expected an ArbCmd, got {}".format(type(arg)))
                with arg._to_raw() as a:
                    raw.dqcs_cq_push(hndl, a)
        return handle

