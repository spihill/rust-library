import subprocess
import sys
import os

yosupo_problems = [
                   ['diameter', 'https://judge.yosupo.jp/problem/tree_diameter'],
                   ['segment_tree_composite', 'https://judge.yosupo.jp/problem/point_set_range_composite'],
                   ['dynamic_segment_tree_composite', 'https://judge.yosupo.jp/problem/point_set_range_composite'],
                   ['disjoint_sparse_table', 'https://judge.yosupo.jp/problem/staticrmq'],
                   ['disjoint_sparse_table_sum', 'https://judge.yosupo.jp/problem/static_range_sum'],
                  ]

def exit_if_fail(result, message):
	if result.returncode != 0:
		print(message, file=sys.stderr)
		exit(1)

for P in yosupo_problems:

	if len(sys.argv) > 1 and sys.argv[1] == 'bundle' and os.getenv('GITHUB_ACTIONS', default="-1") != "-1":
		exit_if_fail(subprocess.run('cargo run --bin bundle library ./Cargo.toml ./verify/yosupo/' + P[0] + '.test.rs > src/main.rs', shell=True), 'Bundle failed')
		exit_if_fail(subprocess.run('cargo build --release --bin main', shell=True), 'Bundle verify failed')
		exit_if_fail(subprocess.run('oj t -c ' + '"target/release/main"', shell=True), 'Bundle verify failed')
	else:
		exit_if_fail(subprocess.run('oj d --system ' + P[1], shell=True), 'Cannot download testcase or not found oj.')
		exit_if_fail(subprocess.run('cargo build --release --bin ' + P[0], shell=True), 'Verify failed')
		exit_if_fail(subprocess.run('oj t -c ' + '"target/release/' + P[0] + '"', shell=True), 'Verify failed')

	exit_if_fail(subprocess.run('rm ./test/*.in -rf', shell=True), 'Unknown Error')
	exit_if_fail(subprocess.run('rm ./test/*.out -rf', shell=True), 'Unknown Error')
	exit_if_fail(subprocess.run('rmdir ./test &> /dev/null || :', shell=True), 'Unknown Error')
