
import dqcsim

a = dqcsim.dqcs_qbset_new()
dqcsim.dqcs_qbset_push(a, 2)

b = dqcsim.dqcs_gate_new_unitary(a, 0, [0, 1, 1, 0])
if b:
    print(dqcsim.dqcs_handle_dump(b))
else:
    print(dqcsim.dqcs_error_get())

print(dqcsim.dqcs_gate_matrix(b))
