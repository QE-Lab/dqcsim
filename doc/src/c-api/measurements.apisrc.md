# Measurements

Measurements are returned in response to measurement gates. While the
measurement result for a qubit is normally just a 0 or a 1 (or some other
boolean convention), DQCsim allows additional information to be attached,
and also allows an "undefined" state to model measurement failure.
Measurement objects/handles are used to encapsulate this. Furthermore, since
a single function call may have to return multiple measurements, measurement
sets are defined as well.

## Contents

 - [Singular measurements](meas.apigen.md)
 - [Measurement sets](mset.apigen.md)
