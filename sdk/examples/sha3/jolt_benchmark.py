import subprocess
import sys

def run_benchmark():
    # Read hex string from file
    with open('input.txt', 'r') as f:
        hex_string = f.read().strip()
    
    # Construct and run the command
    cmd = ['t-rust', 'benchmark', 'jolt', '-k', f'input:bytearray', hex_string]
    
    try:
        # Run the command and capture output
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        # Get the last line of stdout
        last_line = result.stdout #.strip().split('\n')[-1]
        
        # Save to output file
        with open('output.txt', 'w') as f:
            f.write(last_line)
            
        print(f"Last line saved: {last_line}")
        
    except Exception as e:
        print(f"Error running benchmark: {e}")
        sys.exit(1)

if __name__ == "__main__":
    run_benchmark()
