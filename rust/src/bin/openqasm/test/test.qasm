OPENQASM 2.0;

gate u2(phi,lambda) q { U(pi/2,phi,lambda) q; }
gate h a { u2(0,pi) a; }

opaque zz(a, b, c) q;

qreg q[2];
creg c[2];

//measure q -> c;
//h q;
//barrier q;

h q;
measure q -> c;

//zz(1,2,3) q[1];
//zz q[0];

if (c==1) u2(pi, pi) q;
measure q -> c;

// CX q[0], q[1];




//gate u3(theta,phi,lambda) a {
//  U(theta,phi,lambda) a;
//}
//
//gate i a {
//  u3(0, 0, 0) a;
//}
//
//gate special a,b {
//  i a;
//  i b;
//}
//
//gate u2(phi,lambda) q { U(pi/2,phi,lambda) q; }
//
//gate h a {
//  u2(0,pi) a;
//}
//
//qreg q[2];
//creg c[1];
//
//reset q;
//
//h q;
//u3(0, 0, -pi/4) q;
//
//u2(0,pi) q[0];
//u3(0, 0, 0) q[0];
//
//i q[0];
//U(0, pi, pi) q[0];
//U(pi / 2, 0, pi) q[0];
//
//h q[0];
//CX q[0], q[1];
//
//measure q[0] -> c[0];
