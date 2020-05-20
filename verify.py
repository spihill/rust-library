import subprocess
import sys

problems = [
            ['segment_tree_composite', 'https://judge.yosupo.jp/problem/point_set_range_composite'],
            ['dynamic_segment_tree_composite', 'https://judge.yosupo.jp/problem/point_set_range_composite'],
            ['diameter', 'https://judge.yosupo.jp/problem/tree_diameter'],
            ['disjoint_sparse_table', 'https://judge.yosupo.jp/problem/staticrmq'],
            ['disjoint_sparse_table_sum', 'https://judge.yosupo.jp/problem/static_range_sum'],
           ]

def exit_if_fail(result, message):
	if result.returncode != 0:
		print(message, file=sys.stderr)
		exit(1)

exit_if_fail(subprocess.run('ulimit -s unlimited', shell=True), 'Cannot run "ulimit -s unlimited')

for P in problems:
	exit_if_fail(subprocess.run('oj d --system ' + P[1], shell=True), 'Cannot download testcase or not found oj.')
	exit_if_fail(subprocess.run('oj t -c ' + '"cargo run --release --bin ' + P[0] + ' "', shell=True), 'Verify failed')
	exit_if_fail(subprocess.run('rm ./test/*.in -rf', shell=True), 'Unknown Error')
	exit_if_fail(subprocess.run('rm ./test/*.out -rf', shell=True), 'Unknown Error')
	exit_if_fail(subprocess.run('rmdir ./test &> /dev/null || :', shell=True), 'Unknown Error')
