import subprocess
import sys
import os
import json

output_file = "benchmark_result.csv"
with open("bench_config.json", "r") as json_file:
    config = json.load(json_file)

def run_command(cmd):
    try:
        # Run the command and capture output
        process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, bufsize=1)
        stdout_lines = []
        for line in process.stdout:
            print(line, end='')
            stdout_lines.append(line)
        process.stdout.close()
        process.wait()
        result = subprocess.CompletedProcess(cmd, process.returncode, stdout=''.join(stdout_lines))
        return result
        
    except Exception as e:
        print(f"Error running benchmark: {e}")
        sys.exit(1)

def run_benchmark(app, target, input_type, input):
    os.chdir(f"./{app}")
    print(f'Compiling {app} on {target} ...')
    cmd = ['t-rust', 'compile', target]
    run_command(cmd)
    subprocess.run(cmd, capture_output=True, text=True)
    print(f'Running {app} on {target} ...')
    cmd = ['t-rust', 'benchmark', target, '-k', f'input:{input_type}', input]
    result = run_command(cmd)
    os.chdir(f"..")

    result = result.stdout.strip().split('\n')[-1]
    words = result.split()
    time = words[-2]
    return f"{app}, {target}, {time}\n"

def run():
    with open(output_file, 'w') as f:
        f.write("app, target, time (ns)\n")

    for app, app_config in config.items():
        input_value = app_config["input"]
        input_type = app_config["input_type"]
        for target in app_config["targets"]:
            line = run_benchmark(app, target, input_type, str(input_value))
            with open(output_file, 'a') as f:
                f.write(line)
        print('Ok')

if __name__ == "__main__":
    run()

