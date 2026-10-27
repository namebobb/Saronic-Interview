import copy
import json
import subprocess
from pathlib import Path
from pypdf import PdfReader

ROOT = Path(__file__).resolve().parent
PDF = ROOT.parent.parent / 'Saronic - Interview Project - Software Engineer Intern.pdf'
EXE = ROOT / 'target/debug/Saronic-Interview.exe'
pages = [p.extract_text() for p in PdfReader(PDF).pages]
report = ['# PDF command verification', '',
          'Actual executable stdin/stdout, compared as parsed JSON (object key order ignored).',
          'Full commands recovered from embedded PDF text despite visual clipping. Omitted ticks are expanded.',
          'Base example adapted only by adding dock_throughput=[2,4,1] and empty cargo fields required by Follow-on 1.', '']
count = 0
failures = 0

def pairs(text):
    request = None
    for line in text.splitlines():
        line = line.strip()
        if line.startswith('{"command"'):
            request = json.loads(line)
        elif line.startswith('→') and request is not None:
            yield request, json.loads(line[1:].strip())
            request = None

def cargo_fields(response):
    result = copy.deepcopy(response)
    if result['status'] == 'ok':
        result['payload'].setdefault('cargo', [])
        for vessel in result['payload']['vessels'].values():
            vessel.setdefault('cargo', [])
    return result

def expanded(sequence, base=False):
    previous = None
    for request, expected in sequence:
        request, expected = copy.deepcopy(request), copy.deepcopy(expected)
        if base:
            expected = cargo_fields(expected)
            if request['command'] == 'init':
                request['parameters']['dock_throughput'] = [2, 4, 1]
        if request['command'] == 'tick' and previous:
            for tick in range(previous['tick'] + 1, expected['payload']['tick']):
                world = copy.deepcopy(previous)
                world['tick'] = tick
                yield {'command': 'tick', 'parameters': {}}, {'status': 'ok', 'payload': world}, 'Inserted omitted tick'
        yield request, expected, 'PDF example'
        if expected['status'] == 'ok' and request['command'] != 'get_state_at':
            previous = expected['payload']

def run(name, steps):
    global count, failures
    steps = list(steps)
    inputs = [json.dumps(r, separators=(',', ':')) for r, _, _ in steps]
    (ROOT / (name + '.input.jsonl')).write_text('\n'.join(inputs) + '\n', encoding='utf-8')
    proc = subprocess.run([str(EXE)], input='\n'.join(inputs)+'\n', text=True, capture_output=True)
    (ROOT / (name + '.output.jsonl')).write_text(proc.stdout, encoding='utf-8')
    assert proc.returncode == 0, proc.stderr
    lines = proc.stdout.splitlines()
    assert len(lines) == len(steps), (name, len(lines), len(steps))
    report.extend(['## ' + name, ''])
    for index, ((request, expected, note), raw) in enumerate(zip(steps, lines), 1):
        actual = json.loads(raw)
        passed = actual == expected if expected is not None else actual['status'] == 'error'
        count += 1
        failures += not passed
        result = 'PASS' if passed else 'FAIL'
        report.extend([f'### {index}. {request["command"]}: {result}', '', note,
                       '', 'Input:', '```json', inputs[index-1], '```',
                       'Actual output:', '```json', raw, '```',
                       'Expected:', '```json', json.dumps(expected, separators=(',', ':')) if expected is not None else 'Any structured error response', '```', ''])
        print(f'{name} {index:02} {request["command"]}: {result}')

base_text = pages[5].split('\nExample\n', 1)[1] + '\n' + pages[6].split('Follow-on 1:', 1)[0]
run('01-base-example', expanded(pairs(base_text), base=True))
cargo_text = pages[9] + '\n' + pages[10].split('Follow-on 2:', 1)[0]
history_text = pages[10].split('\nExample\n', 1)[1]
cargo_steps = list(expanded(pairs(cargo_text + '\n' + history_text)))
run('02-cargo-and-history', cargo_steps)

# Standalone swap example uses a separate session with its stated inventory.
swap_request = {'command':'swap','parameters':{'vessel_id':'v1','load_destination':1,'unload_destination':2}}
swap_expected = next(expected for request, expected in pairs(pages[8].replace('\n{"status"', '\n→ {"status"')) if request['command']=='swap')
setup = copy.deepcopy(cargo_steps[:5])
run('03-swap-example', setup + [(swap_request, swap_expected, 'Standalone swap example, PDF physical page 9; setup loads one unit at tick 2.')])

# The standalone init includes three manifest entries, unlike the chronological example.
standalone = list(pairs(pages[7].replace('\n{"status"', '\n→ {"status"')))
init_request, init_expected = next((r,e) for r,e in standalone if r['command']=='init')
tick_request, _ = next((r,e) for r,e in standalone if r['command']=='tick')
merged = copy.deepcopy(init_expected)
merged['payload']['tick'] = 1
for entry in merged['payload']['cargo']:
    entry['quantity'] = 2
run('04-manifest-init-and-addition', [(init_request, init_expected, 'Standalone init, PDF physical page 8.'),
    (tick_request, merged, 'Applying the standalone tick after this init must ADD cargo: quantity 2 each. The printed tick response shows quantity 1 and is not a sequential continuation of this init.')])

# Additional stated failures, each exercised through the executable.
init = cargo_steps[0]
error_steps = [({'command':'get_state','parameters':{}}, {'status':'error','payload':{'message':'Not initialized'}}, 'Before init'), init,
    (init[0], {'status':'error','payload':{'message':'Already initialized'}}, 'Second init'),
    ({'command':'get_state_at','parameters':{'tick':-1}}, None, 'Negative historical tick'),
    ({'command':'set_destination','parameters':{'vessel_id':'missing','destination':1}}, {'status':'error','payload':{'message':'Cannot find vessel missing'}}, 'Unknown vessel'),
    ({'command':'set_destination','parameters':{'vessel_id':'v1','destination':3}}, {'status':'error','payload':{'message':'invalid destination'}}, 'Out-of-range destination')]
run('05-rejection-cases', error_steps)
throughput = copy.deepcopy(cargo_steps[:6])
throughput.append(({'command':'unload','parameters':{'vessel_id':'v1','destination':2}}, {'status':'error','payload':{'message':'transfer of 3 operations exceeds dock throughput of 2 at location 0'}}, 'Third cargo operation exceeds throughput; PDF physical page 9.'))
throughput.append(({'command':'get_state','parameters':{}}, cargo_steps[5][1], 'Rejected operation leaves planned state unchanged.'))
run('06-throughput-limit', throughput)
report[2:2] = [f'**Result: {count-failures}/{count} command responses passed; {failures} failures.**', '']
(ROOT / 'PROOF.md').write_text('\n'.join(report), encoding='utf-8')
print(f'TOTAL: {count-failures}/{count} PASS')
raise SystemExit(1 if failures else 0)
