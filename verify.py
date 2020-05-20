import subprocess
import sys

problems = [['disjoint_sparse_table', 'https://judge.yosupo.jp/problem/staticrmq'],
            ['disjoint_sparse_table_sum', 'https://judge.yosupo.jp/problem/static_range_sum']]


for P in problems:
	result = subprocess.run('oj d --system ' + P[1], shell=True)

	if result.returncode != 0:
		print('Cannot download testcase or not found oj.', file=sys.stderr)
		sys.exit(1)

	result = subprocess.run('oj t -c ' + '"cargo run --release --bin ' + P[0] + ' "', shell=True)
	if result.returncode != 0:
		print('Verify failed', file=sys.stderr)
		sys.exit(1)

	result = subprocess.run('rm ./test -rf', shell=True)
	if result.returncode != 0:
		print('Unknown Error', file=sys.stderr)
		sys.exit(1)
