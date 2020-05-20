import subprocess

problems = [['disjoint_sparse_table', 'https://judge.yosupo.jp/problem/staticrmq'],
            ['disjoint_sparse_table_sum', 'https://judge.yosupo.jp/problem/static_range_sum']]


for P in problems:
	subprocess.run('oj d --system ' + P[1], shell=True)
	subprocess.run('oj t -c ' + '"cargo run --release --bin ' + P[0] + ' "', shell=True)
	subprocess.run('rm ./test -rf', shell=True)
