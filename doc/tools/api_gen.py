import re
import sys

comment_stripper = re.compile(r' *\* ?(> ?)?(.*)$')
define_matcher = re.compile(r'#define\s+([a-zA-Z0-9_]+)([^\\\n]|\\\n|\\.)+')
typedef_matcher = re.compile(r'^(?:.|\n)*?([a-zA-Z0-9_]+);$')
variant_matcher = re.compile(r'\s*([a-zA-Z0-9_]+)(?:\s*=\s*([-0-9]+))?\s*(?:,|$)')
function_matcher = re.compile(r'\s*([a-zA-Z0-9_ ]+[ &*]+)([a-zA-Z0-9_]+)\s*\(')
arg_matcher = re.compile(r'\s*([a-zA-Z0-9_ ]+[ &*]+)([a-zA-Z0-9_]+)\s*')
fn_ptr_matcher = re.compile(r'\s*([a-zA-Z0-9_ ]+[ &*]+)\(\*([a-zA-Z0-9_]+)\)\(((?:.|\n)*)\)')

def split_doc(text):
    for block in text.split('\n/**\n')[1:]:
        doc, block = block.split('\n */\n', maxsplit=1)
        doc = '\n'.join((comment_stripper.match(line).group(2) for line in doc.split('\n')))
        yield doc, block

def parse_arg_list(args):
    if args.strip() == 'void':
        return []
    level = 0
    prev = 0
    arg_list = []
    for i, c in enumerate(args):
        if c == '(':
            level += 1
        elif c == ')':
            level -= 1
        elif c == ',' and level == 0:
            arg_list.append(args[prev:i].strip())
            prev = i + 1
    arg_list.append(args[prev:].strip())

    parsed_arg_list = []
    for arg in arg_list:
        match = arg_matcher.match(arg)
        if match:
            parsed_arg_list.append((match.group(1), match.group(2)))
            continue

        match = fn_ptr_matcher.match(arg)
        if match:
            parsed_arg_list.append((match.group(1), match.group(2), parse_arg_list(match.group(3))))
            continue

        if arg.strip() == '...':
            parsed_arg_list.append(('...',))
            continue

        print(arg)
        raise ValueError('failed to parse argument')

    return parsed_arg_list

def format_arg(parsed_arg):
    if len(parsed_arg) == 3:
        output = format_arg_list(parsed_arg[2])
        output[0] = '{}(*{}){}'.format(parsed_arg[0], parsed_arg[1], output[0])
        return output
    else:
        return [''.join(parsed_arg[:2])]

def format_arg_list(parsed_arg_list):
    if len(parsed_arg_list) == 0:
        return ['(void)']
    elif len(parsed_arg_list) == 1:
        output = format_arg(parsed_arg_list[0])
        output[0] = '(' + output[0]
        output[-1] = output[-1] + ')'
        return output
    else:
        output = []
        for arg in parsed_arg_list:
            if output:
                output[-1] += ','
            output.extend(('    ' + line for line in format_arg(arg)))
        output.insert(0, '(')
        output.append(')')
        return output

def parse_header_file(fname):

    with open(fname, 'r') as f:
        header = f.read()

    entities = []

    for doc, block in split_doc(header):
        if block.startswith('#define'):
            match = define_matcher.match(block)
            if not match:
                print(block)
                raise ValueError('failed to parse #define')
            block = match.group(0)
            name = match.group(1)

            if name.upper() == name:
                entities.append(('define', name, block, doc))
            else:
                entities.append(('function', name, block, doc))

        elif block.startswith('typedef'):
            level = 0
            enumerator = enumerate(block)
            start = None
            end = None
            for i, c in enumerator:
                if c == '{':
                    if level == 0:
                        start = i
                    level += 1
                elif c == '}':
                    level -= 1
                    if level == 0:
                        i += 1
                        end = i
                        break
            for i, c in enumerator:
                if c == ';':
                    i += 1
                    break
            block = block[:i]

            match = typedef_matcher.match(block)
            if not match:
                print(block)
                raise ValueError('failed to parse typedef')
            name = match.group(1)

            if start is not None and end is not None:
                inner = block[start+1:end-1]
                block = block[:start] + '{ ... }' + block[end:]

                inner = inner.replace('\n  ', '\n')

                variants = []
                for vdoc, vblock in split_doc(inner):
                    vmatch = variant_matcher.match(vblock)
                    vname = vmatch.group(1)
                    vvalue = vmatch.group(2)
                    variants.append((vname, vvalue, vdoc))

                entities.append(('enum', name, block, doc, variants))

            else:
                entities.append(('typedef', name, block, doc))

        elif block.startswith('namespace'):
            pass

        else:
            level = 0
            enumerator = enumerate(block)
            for i, c in enumerator:
                if c == '(':
                    level += 1
                elif c == ')':
                    level -= 1
                    if level == 0:
                        i += 1
                        break
            block = block[:i] + ';'

            match = function_matcher.match(block)
            if not match:
                print(block)
                raise ValueError('failed to parse function')
            return_type = match.group(1).replace('inline', '').replace('  ', ' ').lstrip()
            name = match.group(2)

            arguments = block.split('(', maxsplit=1)[1][:-2]
            arguments = parse_arg_list(arguments)

            block = return_type + name + '\n'.join(format_arg_list(arguments))

            entities.append(('function', name, block, doc))

    return entities

def entities_to_markdown(entities):
    output = []

    for etyp, name, block, doc, *args in entities:

        brief, *doc = doc.split('\n\n', maxsplit=1)
        if doc:
            doc = doc[0]
        else:
            doc = None

        fmt = {
            'function': '<details><summary><font color="green"><tt>{}()</tt></font>',
            'enum': '<details><summary><font color="purple"><tt>{}</tt></font>',
            'typedef': '<details><summary><font color="purple"><tt>{}</tt></font>',
            'define': '<details><summary><font color="red"><tt>{}</tt></font>',
        }[etyp]
        fmt += '<div style="margin-left: 16px">\n\n{}\n\n</div></summary><div style="margin-left: 16px">'
        output.append(fmt.format(name, brief))
        output.append('')
        output.append('```C')
        output.append(block)
        output.append('```')
        if doc:
            output.append('')
            output.append(doc)
        if etyp == 'enum':
            output.append('')
            output.append('Variants:')
            output.append('')
            output.append('<p>')
            for name, value, doc in args[0]:
                code = name
                if value is not None:
                    code += ' = ' + value
                output.append('<details><summary><font color="blue"><tt>{}</tt></font></summary><div style="margin-left: 16px">\n{}\n\n</div></details>'.format(code, doc.strip()))
            output.append('</p>')

        output.append('</div></details>')

    return '\n'.join(output)

if __name__ == '__main__':

    if len(sys.argv) < 4:
        print('usage: python3 {} <header.h> <mod.rs> <template.apisrc.md> ...'.format(sys.argv[0]))
        sys.exit(1)

    entities = parse_header_file(sys.argv[1])
    undocumented = set((entity[1] for entity in entities))

    with open(sys.argv[2], 'r') as f:
        lines = f.read().split('\n')
    in_md_header = True
    module_doc = []
    for line in lines:
        if not line.startswith('//!'):
            continue
        line = line[4:]
        if line.startswith('#'):
            in_md_header = False
            line = '#' + line
        if in_md_header:
            continue
        module_doc.append(line)
    module_doc = '\n'.join(module_doc)

    for fname in sorted(sys.argv[3:]):
        if '.apisrc.' not in fname:
            raise ValueError('source filename must contain \'.apisrc.\' so destination filename can be derived')

        with open(fname, 'r') as f:
            template = f.read()

        while True:
            match = re.search(r'@@@c_api_gen(?:\s+((?:(?!@@@).)*))?@@@', template)
            if not match:
                break
            span = match.span()

            name_regex = match.group(1)
            if name_regex is None:
                filtered_entities = entities
            else:
                name_regex = re.compile(name_regex)
                filtered_entities = filter(lambda entity: name_regex.search(entity[1]), entities)

            filtered_entities = [entity for entity in filtered_entities if entity[1] in undocumented]

            for entity in filtered_entities:
                undocumented.remove(entity[1])

            template = template[:span[0]] + entities_to_markdown(filtered_entities) + template[span[1]:]

        template = template.replace('@@@rust_module_doc@@@', module_doc)

        if '@@@c_api_gen_ref@@@' in template:
            template = template.replace('@@@c_api_gen_ref@@@', entities_to_markdown(sorted(entities, key=lambda e: e[1])))

        with open(fname.replace('.apisrc.', '.apigen.'), 'w') as f:
            f.write(template)

    if undocumented:
        print('These entities are not documented anywhere: ')
        for name in sorted(undocumented):
            print(' -', name)
        print('Returning error code')
        sys.exit(1)
