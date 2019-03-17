
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
        args.extend(m.group(3).strip().split(', '))
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
    if (!Py_IsInitialized()) return ({cb_ret}){cb_ret_fail};
    PyGILState_STATE gstate;
    gstate = PyGILState_Ensure();

    PyObject *ret_obj = PyObject_CallFunction((PyObject*)user, "{cb_arg_fmts}"{cb_arg_refs});
    if (!ret_obj) goto pyerr;
    long long ret_long = PyLong_AsLongLong(ret_obj);
    Py_XDECREF(ret_obj);
    if (ret_long == -1 && PyErr_Occurred()) goto pyerr;

    PyGILState_Release(gstate);
    return ({cb_ret})ret_long;
pyerr:
    Py_XDECREF(ret_obj);

    PyObject *exc_type = NULL, *exc_value = NULL, *exc_tb = NULL;
    PyErr_Fetch(&exc_type, &exc_value, &exc_tb);
    PyObject* str_exc_value = PyObject_Str(exc_value);
    PyObject* py_str = PyUnicode_AsEncodedString(str_exc_value, "utf-8", "Error ~");
    const char *c_str = PyBytes_AS_STRING(py_str);
    Py_XDECREF(str_exc_value);
    Py_XDECREF(py_str);
    Py_XDECREF(exc_type);
    Py_XDECREF(exc_value);
    Py_XDECREF(exc_tb);
    dqcs_set_error(c_str);

    PyGILState_Release(gstate);
    return ({cb_ret}){cb_ret_fail};
}}

{signature} {{
    if (!PyCallable_Check(callable)) {{
        PyErr_SetString(PyExc_RuntimeError, "The specified callback is not callable");
    }}
    Py_INCREF(callable);
    return {name}({user_args}{name}_handler, dqcs_swig_callback_cleanup, (void*)callable);
}}
%}}

%exception {name}_pyfun {{
    $action
    if (PyErr_Occurred()) {{
        SWIG_fail;
    }}
    if (result) {{
        const char *s = dqcs_explain();
        if (!s) s = "Unknown error";
        PyErr_SetString(PyExc_RuntimeError, s);
        SWIG_fail;
    }}
}}

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

output = ['''\
%module dqcsim

%include <pybuffer.i>
%include exception.i

%inline %{
#include "dqcsim.h"
typedef long unsigned int size_t;
typedef long signed int ssize_t;
typedef signed int int32_t;
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

            # RULE: all functions returning "const char *" except for
            # dqcs_explain return an owned string that must be freed by SWIG.
            if ret_typ == 'const char *' and name != 'dqcs_explain':
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
                and args[-3][0][1] == 'void*'
                and args[-2][0] == ['void', 'void*']
                and args[-1][0] == 'void *'
            ):
                line += '\n' + gen_callback_installer(name, args)

        except:
            print('While parsing the following line as a function...')
            print()
            print(line)
            print()
            raise

    output.append(line)

with open(sys.argv[2], 'w') as f:
    f.write('\n\n'.join(output) + '\n')
