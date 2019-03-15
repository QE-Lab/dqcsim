%module dqcshost
%{
#include "dqcshost.h"
%}

%include gen/c/dqcshost.h

// Mark all functions that return a newly allocated object with newobject, so
// swig deallocates them after making its own copy.
%newobject dqcs_arb_get_str;
%newobject dqcs_arb_json_get_str;
%newobject dqcs_arb_pop_str;
%newobject dqcs_handle_dump;
