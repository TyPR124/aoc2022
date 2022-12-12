@_default:
    just --list --unsorted --justfile {{justfile()}}

# Create and open the source file for the given day. Refuses to overwrite an existing file.
begin day:
    #!/usr/bin/env python3

    from os import path
    from subprocess import run

    day = {{day}}
    template_path = 'day.template'
    target_path = f'src/bin/day{day:02}.rs'

    if path.exists(target_path):
        print(f'File {target_path} already exists. Refusing to continue.')
        exit(1)
    
    with open(template_path, 'r') as f:
        template = f.read()
    
    template = template.replace('day01', f'day{day:02}')
    template = template.replace('Day01', f'Day{day:02}')
    template = template.replace('Day::number(1)', f'Day::number({day})')

    with open(target_path, 'w') as f:
        f.write(template)
    
    run(['code', target_path, '--reuse-window'])

# Adds a new test for the given day by creating a blank input file and an answers file with all '0' answers.
add-test day name:
    #!/usr/bin/env python3

    from subprocess import run
    from sys import stdin

    day = {{day}}
    name = '{{name}}'
    test_path = f'tests/day{day:02}_{name}.yaml'

    with open(test_path, 'x') as f:
        print('Please paste the input data below:')
        print('Use ctrl+D on a new line to end')
        print('----------------------------------')
        input = ''.join(['  ' + line for line in stdin.readlines()])
        f.write(f'---\ninput: |-\n{input}\nanswers:\n  - 0\n  - 0\n')
    
    run(['code', test_path, '--reuse-window'])
