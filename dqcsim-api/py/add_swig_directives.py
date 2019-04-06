
import sys
import re
from itertools import tee

def split_type(s):
    m = re.match(r'([\w ]+ \**)(\w+)', s)
    if m:
        return m.group(1).strip(), m.group(2).strip()
    m = re.match(r'(\w+)\s*\(\*(\w+)\)\(([\w*, ]+)\)', s)
    if m:
        args = [m.group(1).strip()]
        args.extend(map(lambda x: split_type(x)[0], m.group(3).strip().split(', ')))
        if len(args) == 2 and args[1] == ['void']:
            del args[-1]
        return args, m.group(2).strip()
    raise Exception('Cannot parse type + name argument: \'%s\'', s)

def pairwise(iterable):
    a, b = tee(iterable)
    next(b, None)
    return zip(a, b)

def arg_type_to_py(typ):
    if typ == 'const char*' or typ == 'const char *' or typ == 'char*' or typ == 'char *':
        return 's', 'const char*'
    else:
        return 'L', 'long long'

def gen_callback_installer(name, args):
    user_args = list(map(lambda x: x[0], args[:-3]))

    cb_ret = args[-3][0][0]
    cb_ret_fail = '-1'
    if cb_ret == 'dqcs_return_t':
        cb_ret_fail = 'DQCS_FAILURE'
    elif cb_ret == 'dqcs_handle_t' or cb_ret == 'dqcs_qubit_t':
        cb_ret_fail = '0'

    cb_args = args[-3][0][2:]

    sig_args = list(map(lambda x: '{1} arg{0}'.format(*x), enumerate(user_args)))
    sig_args.append('PyObject *callable')
    signature = 'dqcs_return_t {0}_pyfun({1})'.format(name, ', '.join(sig_args))

    return '''\
%{{
{cb_ret} {name}_handler(void *user{cb_args}) {{
    // Claim GIL.
    if (!Py_IsInitialized()) return ({cb_ret}){cb_ret_fail};
    PyGILState_STATE gstate;
    gstate = PyGILState_Ensure();

    // Call user function.
    if (!PyCallable_Check((PyObject*)user)) {{
        dqcs_error_set("Callback object is not callable!");
        return ({cb_ret}){cb_ret_fail};
    }}
    PyObject *ret_obj = PyObject_CallFunction((PyObject*)user, "{cb_arg_fmts}"{cb_arg_refs});

    // Catch exception from user code.
    if (ret_obj == NULL) goto pyerr;

    // Convert result. Interpret None as 0, otherwise value must be an integer.
    long long ret_long = 0;
    if (ret_obj != Py_None) {{
        ret_long = PyLong_AsLongLong(ret_obj);
    }}

    // Regardless of whether the object was None or whatever, we need to
    // release our reference to the return value.
    Py_XDECREF(ret_obj);

    // Catch return value conversion errors.
    if (ret_long == -1 && PyErr_Occurred()) goto pyerr;

    // Release GIL.
    PyGILState_Release(gstate);
    return ({cb_ret})ret_long;

pyerr:
    {{
        int ok = 0;

        // Claim ownership of the Python exception data. We only care about the
        // value, so we can immediately release the rest.
        PyObject *exc_type = NULL, *exc_value = NULL, *exc_tb = NULL;
        PyErr_Fetch(&exc_type, &exc_value, &exc_tb);
        Py_XDECREF(exc_type);
        Py_XDECREF(exc_tb);

        // Parse the exception value.
        if (exc_value != NULL) {{

            // Convert to Python string.
            PyObject* str_exc_value = PyObject_Str(exc_value);
            Py_XDECREF(exc_value);
            if (str_exc_value != NULL) {{

                // Encode Python string into a binary string with UTF-8 encoding.
                PyObject* py_str = PyUnicode_AsEncodedString(str_exc_value, "utf-8", "ignore");
                Py_XDECREF(str_exc_value);
                if (py_str != NULL) {{

                    // Save the string. Note that the string will get dealloc'd
                    // when the reference is released, so we need to call
                    // dqcs_error_set() before then (it makes a copy).
                    const char *c_str = PyBytes_AS_STRING(py_str);
                    if (c_str != NULL) {{
                        dqcs_error_set(c_str);
                        ok = 1;
                    }}
                    Py_XDECREF(py_str);
                }}
            }}
        }}

        if (!ok) {{
            // We failed to parse the exception value. Set a generic error instead.
            dqcs_error_set("Unknown error");

            // Make sure to leave Python's exception marker empty.
            PyErr_Fetch(&exc_type, &exc_value, &exc_tb);
            Py_XDECREF(exc_type);
            Py_XDECREF(exc_value);
            Py_XDECREF(exc_tb);
        }}

        // Release GIL.
        PyGILState_Release(gstate);
        return ({cb_ret}){cb_ret_fail};
    }}
}}

{signature} {{
    // Callbacks are usually called from different threads, so make sure that
    // Python's thread system is active. This function can safely be called
    // multiple times according to the docs, so it's better to be safe than
    // sorry here.
    PyEval_InitThreads();

    // Make sure the object is callable.
    if (!PyCallable_Check(callable)) {{
        dqcs_error_set("The specified callback is not callable");
        return DQCS_FAILURE;
    }}

    // Right now we only have a borrowed reference to the callable. We save the
    // callable, so we need to take ownership of our own reference. This will
    // be cleaned up using dqcs_swig_callback_cleanup(), which is ALWAYS called
    // exactly once, regardless of whether installing the callback succeeds or
    // fails.
    Py_INCREF(callable);

    // Since we don't need to do anything else after installing the callback,
    // we can return its result immediately.
    return {name}({user_args}{name}_handler, dqcs_swig_callback_cleanup, (void*)callable);
}}
%}}

{signature};
'''.format(
    signature = signature,
    name = name,
    user_args = ''.join(['arg%d, ' % i for i in range(len(user_args))]),
    cb_ret = cb_ret,
    cb_ret_fail = cb_ret_fail,
    cb_args = ''.join(map(', {0[1]} arg{0[0]}'.format, enumerate(cb_args))),
    cb_arg_refs = ''.join(map(lambda x: ', ({1})arg{0}'.format(x[0], arg_type_to_py(x[1])[1]), enumerate(cb_args))),
    cb_arg_fmts = ''.join(map(lambda x: arg_type_to_py(x)[0], cb_args)),
)


if len(sys.argv) != 3:
    print('Usage: %s <infile> <outfile>' % sys.argv[0])
    sys.exit(1)

with open(sys.argv[1], 'r') as f:
    data = f.read()

# TODO:
#%exception {name}_pyfun {{
    #$action
    #if (PyErr_Occurred()) {{
        #SWIG_fail;
    #}}
    #if (result) {{
        #const char *s = dqcs_error_get();
        #if (!s) s = "Unknown error";
        #PyErr_SetString(PyExc_RuntimeError, s);
        #SWIG_fail;
    #}}
#}}

output = ['''\
%module(threads="1") dqcsim
%nothread;

%include <pybuffer.i>
%include exception.i
%include stdint.i

%inline %{
#include "dqcsim.h"
typedef unsigned long size_t;
typedef signed long ssize_t;
%}

%{
void dqcs_swig_callback_cleanup(void *user) {
    if (!Py_IsInitialized()) return;
    PyGILState_STATE gstate;
    gstate = PyGILState_Ensure();
    Py_XDECREF((PyObject*)user);
    PyGILState_Release(gstate);
}
%}

%typemap(out) double* dqcs_gate_matrix {
  if ($1 == NULL) {
    Py_INCREF(Py_None);
    $result = Py_None;
  } else {
    size_t len = dqcs_gate_matrix_len(arg1);
    $result = PyList_New(len);
    for (size_t i = 0; i < len; i++) {
      PyObject *o = PyComplex_FromDoubles((double) $1[i * 2], (double) $1[i * 2 + 1]);
      PyList_SetItem($result,i,o);
    }
    free($1);
  }
}

%typemap(in) (const double *matrix, size_t matrix_len) {
  if (!PySequence_Check($input)) {
    PyErr_SetString(PyExc_ValueError, "Expected a sequence");
    SWIG_fail;
  }
  $2 = PySequence_Length($input);
  $1 = calloc($2 * 2, sizeof(double));
  if ($1 == NULL) {
    PyErr_SetString(PyExc_ValueError, "Failed to allocate memory");
    SWIG_fail;
  }
  for (size_t i = 0; i < $2; i++) {
    PyObject *o = PySequence_GetItem($input, i);
    if (PyNumber_Check(o)) {
      Py_complex pc = PyComplex_AsCComplex(o);
      $1[i * 2] = pc.real;
      $1[i * 2 + 1] = pc.imag;
    } else {
      PyErr_SetString(PyExc_ValueError, "Sequence elements must be numbers");
      SWIG_fail;
    }
  }
}

%typemap(freearg) (const double *matrix, size_t matrix_len) {
  if ($1 != NULL) {
    free($1);
  }
}

%typemap(in) dqcs_plugin_state_t = long long;

''']

for line in data.split('\n\n'):
    line = line.strip()
    if not line.startswith('typedef ') and not line.startswith('#include '):
        # Assuming this is a function now...
        try:
            name, args = line.split('(', maxsplit=1)
            args, _ = args.rsplit(')', maxsplit=1)
            if args == 'void':
                args = []
            else:
                arg_list = ['']
                depth = 0
                for c in args:
                    if c == '(':
                        depth += 1
                    elif c == ')':
                        depth -= 1
                    elif depth == 0 and c == ',':
                        arg_list.append('')
                        continue
                    arg_list[-1] += c
                args = list(map(split_type, map(str.strip, arg_list)))
            ret_typ, name = split_type(name)

            # RULE: all functions returning "char *" must be freed by SWIG.
            if ret_typ == 'char *':
                line = '%%newobject %s;\n%s' % (name, line)

            for a, b in pairwise(args):
                a_type, a_name = a
                b_type, b_name = b

                # RULE: a pair of consecutive arguments following the pattern
                # "void *<NAME>, size_t <NAME>_size" represents a mutable
                # bytebuffer.
                if a_type == 'void *' and b_type == 'size_t' and b_name == a_name + '_size':
                    line = '%%pybuffer_mutable_binary(%s, %s);\n%s' % (' '.join(a), ' '.join(b), line)

                # RULE: a pair of consecutive arguments following the pattern
                # "const void *<NAME>, size_t <NAME>_size" represents a const
                # bytebuffer.
                if a_type == 'const void *' and b_type == 'size_t' and b_name == a_name + '_size':
                    line = '%%pybuffer_binary(%s, %s);\n%s' % (' '.join(a), ' '.join(b), line)

            # RULE: all functions with:
            #  - dqcs_return_t return type;
            #  - three or more arguments;
            #  - the third-to-last argument being a function pointer taking a
            #    void* as first argument;
            #  - the second-to-last argument being a function pointer taking
            #    only a void* as argument that returns void;
            #  - the last argument being a void*;
            # are treated as callback installers. These functions can not be
            # ergonomically generated by SWIG, because SWIG does not support
            # script-side callbacks. We'll have to generate them here. Oh boy.
            # We currently only support the following callback return types:
            #  - dqcs_return_t; DQCS_FAILURE indicates failure
            #  - dqcs_handle_t; 0 indicates failure
            #  - dqcs_qubit_t; 0 indicates failure
            #  - anything else is assumed to be castable from long long and is
            #    interpreted as such; -1 failure value.
            # And we only support the following callback argument types:
            #  - (const) char*: interpreted as string (input only)
            #  - anything else: assumed to be castable to long long and
            #    interpreted as such
            if (
                ret_typ == 'dqcs_return_t'
                and len(args) >= 3
                and type(args[-3][0]) == list
                and len(args[-3][0]) >= 2
                and args[-3][0][1] == 'void *'
                and args[-2][0] == ['void', 'void *']
                and args[-1][0] == 'void *'
            ):
                line += '\n' + gen_callback_installer(name, args)

            # RULE: the following functions:
            #  - dqcs_accel_*
            #  - dqcs_log_*
            #  - dqcs_plugin_*
            #  - dqcs_sim_*
            #  - dqcs_handle_delete
            # can block waiting for other threads that can call back into
            # Python, therefore needing a %thread directive to make SWIG
            # release the GIL before calling them. We need to be particularly
            # broad about this because even a Rust logging macro may do this
            # when the crossbeam channel to the log thread fills up and a
            # Python log callback is installed!
            if name.split('_')[1] in ['sim', 'accel', 'plugin', 'log'] or name == 'dqcs_handle_delete':
                line = '%%thread %s;\n%s' % (name, line)

        except:
            print('While parsing the following line as a function...')
            print()
            print(line)
            print()
            raise

    output.append(line)

with open(sys.argv[2], 'w') as f:
    f.write('\n\n'.join(output) + '\n')
