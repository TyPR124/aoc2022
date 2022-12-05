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

    day = {{day}}
    name = '{{name}}'
    input_path = f'test_input/day{day:02}_{name}.input'
    answers_path = f'test_input/day{day:02}_{name}.answers'

    with open(input_path, 'x') as f:
        pass
    
    with open(answers_path, 'x') as f:
        f.write('0\n0')
    
    run(['code', input_path, answers_path, '--reuse-window'])
