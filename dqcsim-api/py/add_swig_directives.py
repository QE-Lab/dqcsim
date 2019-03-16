
import sys
import re
from itertools import tee

def split_type(s):
    m = re.match(r'([\w ]+ \**)(\w+)', s)
    return m.group(1).strip(), m.group(2).strip()

def pairwise(iterable):
    a, b = tee(iterable)
    next(b, None)
    return zip(a, b)


if len(sys.argv) != 3:
    print('Usage: %s <infile> <outfile>' % sys.argv[0])
    sys.exit(1)

with open(sys.argv[1], 'r') as f:
    data = f.read()

output = ['''\
%module dqcsim

%include <pybuffer.i>

%inline %{
#include "dqcsim.h"
typedef long unsigned int size_t;
typedef long signed int ssize_t;
%}\
''']

for line in data.split('\n\n'):
    line = line.strip()
    if not line.startswith('typedef ') and not line.startswith('#include '):
        # Assuming this is a function now...
        try:
            name, args = line.split('(')
            args, _ = args.split(')')
            if args == 'void':
                args = []
            else:
                args = list(map(split_type, args.split(', ')))
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

        except:
            print("While parsing the following line as a function...")
            print()
            print(line)
            print()
            raise

    output.append(line)

with open(sys.argv[2], 'w') as f:
    f.write('\n\n'.join(output) + '\n')
