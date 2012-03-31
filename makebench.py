
import os
import shutil

num_dirs = 0
num_files = 0
num_lines = 0

def make_dir(path, level = 0):
	global num_dirs, num_files, num_lines
	num_dirs += 1
	os.mkdir(path)
	if level < 4:
		for i in xrange(5):
			make_dir(path + "/sub%d"  % i, level + 1)
	for i in xrange(20):
		num_files += 1
		f = open("%s/file%d.txt" % (path, i), "w")
		for j in xrange(10):
			num_lines += 1
			f.write("line %d\n" % j)
		f.close()

shutil.rmtree("bench")
make_dir("bench")

print "Num dirs:", num_dirs
print "Num files:", num_files
print "Num lines:", num_lines
