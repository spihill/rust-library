import subprocess
import sys
import os
import toml

def exit_if_fail(result, message):
	if result.returncode != 0:
		print(message, file=sys.stderr)
		exit(1)

args = sys.argv

tom = toml.load('Cargo.toml')
package_name = tom['package']['name']
bin = tom['bin']

for P in bin:
	if not P['path'].endswith(".test.rs"):
		continue
	f = open(P['path'])
	url = f.readline()
	url = url[url.find('"') + 1 : url.rfind('"')]
	f.close()
	if len(sys.argv) > 1 and sys.argv[1] == 'bundle' and os.getenv('GITHUB_ACTIONS', default="-1") != "-1":
		exit_if_fail(subprocess.run('oj d --system ' + url, shell=True), 'Cannot download testcase or not found oj.')
		exit_if_fail(subprocess.run('cargo run --bin bundle library ./Cargo.toml ' + P['path'] + ' > src/main.rs', shell=True), 'Bundle failed')
		exit_if_fail(subprocess.run('cargo build --release --bin main', shell=True), 'Bundle verify failed')
		exit_if_fail(subprocess.run('oj t -c ' + '"target/release/main"', shell=True), 'Bundle verify failed')
	else:
		exit_if_fail(subprocess.run('oj d --system ' + url, shell=True), 'Cannot download testcase or not found oj.')
		exit_if_fail(subprocess.run('cargo build --release --bin ' + P['name'], shell=True), 'Verify failed')
		exit_if_fail(subprocess.run('oj t -c ' + '"target/release/' + P['name'] + '"', shell=True), 'Verify failed')

	exit_if_fail(subprocess.run('rm ./test/*.in -rf', shell=True), 'Unknown Error')
	exit_if_fail(subprocess.run('rm ./test/*.out -rf', shell=True), 'Unknown Error')
	exit_if_fail(subprocess.run('rmdir ./test &> /dev/null || :', shell=True), 'Unknown Error')
