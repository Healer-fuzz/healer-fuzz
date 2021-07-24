#define _GNU_SOURCE 

#include <dirent.h>
#include <endian.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mount.h>
#include <sys/prctl.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

static void sleep_ms(uint64_t ms)
{
	usleep(ms * 1000);
}

static uint64_t current_time_ms(void)
{
	struct timespec ts;
	if (clock_gettime(CLOCK_MONOTONIC, &ts))
	exit(1);
	return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
}

static void use_temporary_dir(void)
{
	char tmpdir_template[] = "./syzkaller.XXXXXX";
	char* tmpdir = mkdtemp(tmpdir_template);
	if (!tmpdir)
	exit(1);
	if (chmod(tmpdir, 0777))
	exit(1);
	if (chdir(tmpdir))
	exit(1);
}

static bool write_file(const char* file, const char* what, ...)
{
	char buf[1024];
	va_list args;
	va_start(args, what);
	vsnprintf(buf, sizeof(buf), what, args);
	va_end(args);
	buf[sizeof(buf) - 1] = 0;
	int len = strlen(buf);
	int fd = open(file, O_WRONLY | O_CLOEXEC);
	if (fd == -1)
		return false;
	if (write(fd, buf, len) != len) {
		int err = errno;
		close(fd);
		errno = err;
		return false;
	}
	close(fd);
	return true;
}

#define FS_IOC_SETFLAGS _IOW('f', 2, long)
static void remove_dir(const char* dir)
{
	int iter = 0;
	DIR* dp = 0;
retry:
		while (umount2(dir, MNT_DETACH) == 0) {
		}
	dp = opendir(dir);
	if (dp == NULL) {
		if (errno == EMFILE) {
	exit(1);
		}
	exit(1);
	}
	struct dirent* ep = 0;
	while ((ep = readdir(dp))) {
		if (strcmp(ep->d_name, ".") == 0 || strcmp(ep->d_name, "..") == 0)
			continue;
		char filename[FILENAME_MAX];
		snprintf(filename, sizeof(filename), "%s/%s", dir, ep->d_name);
			while (umount2(filename, MNT_DETACH) == 0) {
			}
		struct stat st;
		if (lstat(filename, &st))
	exit(1);
		if (S_ISDIR(st.st_mode)) {
			remove_dir(filename);
			continue;
		}
		int i;
		for (i = 0;; i++) {
			if (unlink(filename) == 0)
				break;
			if (errno == EPERM) {
				int fd = open(filename, O_RDONLY);
				if (fd != -1) {
					long flags = 0;
					if (ioctl(fd, FS_IOC_SETFLAGS, &flags) == 0) {
					}
					close(fd);
					continue;
				}
			}
			if (errno == EROFS) {
				break;
			}
			if (errno != EBUSY || i > 100)
	exit(1);
				if (umount2(filename, MNT_DETACH))
	exit(1);
		}
	}
	closedir(dp);
	for (int i = 0;; i++) {
		if (rmdir(dir) == 0)
			break;
		if (i < 100) {
			if (errno == EPERM) {
				int fd = open(dir, O_RDONLY);
				if (fd != -1) {
					long flags = 0;
					if (ioctl(fd, FS_IOC_SETFLAGS, &flags) == 0) {
					}
					close(fd);
					continue;
				}
			}
			if (errno == EROFS) {
				break;
			}
			if (errno == EBUSY) {
					if (umount2(dir, MNT_DETACH))
	exit(1);
				continue;
			}
			if (errno == ENOTEMPTY) {
				if (iter < 100) {
					iter++;
					goto retry;
				}
			}
		}
	exit(1);
	}
}

static int inject_fault(int nth)
{
	int fd;
	fd = open("/proc/thread-self/fail-nth", O_RDWR);
	if (fd == -1)
	exit(1);
	char buf[16];
	sprintf(buf, "%d", nth + 1);
	if (write(fd, buf, strlen(buf)) != (ssize_t)strlen(buf))
	exit(1);
	return fd;
}

static void kill_and_wait(int pid, int* status)
{
	kill(-pid, SIGKILL);
	kill(pid, SIGKILL);
	for (int i = 0; i < 100; i++) {
		if (waitpid(-1, status, WNOHANG | __WALL) == pid)
			return;
		usleep(1000);
	}
	DIR* dir = opendir("/sys/fs/fuse/connections");
	if (dir) {
		for (;;) {
			struct dirent* ent = readdir(dir);
			if (!ent)
				break;
			if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0)
				continue;
			char abort[300];
			snprintf(abort, sizeof(abort), "/sys/fs/fuse/connections/%s/abort", ent->d_name);
			int fd = open(abort, O_WRONLY);
			if (fd == -1) {
				continue;
			}
			if (write(fd, abort, 1) < 0) {
			}
			close(fd);
		}
		closedir(dir);
	} else {
	}
	while (waitpid(-1, status, __WALL) != pid) {
	}
}

static void setup_test()
{
	prctl(PR_SET_PDEATHSIG, SIGKILL, 0, 0, 0);
	setpgrp();
	write_file("/proc/self/oom_score_adj", "1000");
}

static void setup_fault()
{
	static struct {
		const char* file;
		const char* val;
		bool fatal;
	} files[] = {
	    {"/sys/kernel/debug/failslab/ignore-gfp-wait", "N", true},
	    {"/sys/kernel/debug/fail_futex/ignore-private", "N", false},
	    {"/sys/kernel/debug/fail_page_alloc/ignore-gfp-highmem", "N", false},
	    {"/sys/kernel/debug/fail_page_alloc/ignore-gfp-wait", "N", false},
	    {"/sys/kernel/debug/fail_page_alloc/min-order", "0", false},
	};
	unsigned i;
	for (i = 0; i < sizeof(files) / sizeof(files[0]); i++) {
		if (!write_file(files[i].file, files[i].val)) {
			if (files[i].fatal)
	exit(1);
		}
	}
}

static void execute_one(void);

#define WAIT_FLAGS __WALL

static void loop(void)
{
	int iter = 0;
	for (;; iter++) {
		char cwdbuf[32];
		sprintf(cwdbuf, "./%d", iter);
		if (mkdir(cwdbuf, 0777))
	exit(1);
		int pid = fork();
		if (pid < 0)
	exit(1);
		if (pid == 0) {
			if (chdir(cwdbuf))
	exit(1);
			setup_test();
			execute_one();
			exit(0);
		}
		int status = 0;
		uint64_t start = current_time_ms();
		for (;;) {
			if (waitpid(-1, &status, WNOHANG | WAIT_FLAGS) == pid)
				break;
			sleep_ms(1);
		if (current_time_ms() - start < 5000) {
			continue;
		}
			kill_and_wait(pid, &status);
			break;
		}
		remove_dir(cwdbuf);
	}
}

uint64_t r[2] = {0xffffffffffffffff, 0xffffffffffffffff};

void execute_one(void)
{
		intptr_t res = 0;
memcpy((void*)0x20000740, "/dev/kvm\000", 9);
	res = syscall(__NR_openat, 0xffffffffffffff9cul, 0x20000740ul, 0x481001a000001409ul, 0ul);
	if (res != -1)
		r[0] = res;
	res = syscall(__NR_ioctl, r[0], 0xae01, 0ul);
	if (res != -1)
		r[1] = res;
*(uint64_t*)0x20000240 = 0;
*(uint32_t*)0x20000248 = 0x8000;
*(uint32_t*)0x2000024c = 0;
	syscall(__NR_ioctl, r[1], 0x4010ae67, 0x20000240ul);
*(uint64_t*)0x20000040 = 0;
*(uint32_t*)0x20000048 = 0x3000;
*(uint32_t*)0x2000004c = 0;
	syscall(__NR_ioctl, r[1], 0x4010ae67, 0x20000040ul);
*(uint64_t*)0x20000380 = 0;
*(uint32_t*)0x20000388 = 0x4000;
*(uint32_t*)0x2000038c = 0;
	inject_fault(3);
	syscall(__NR_ioctl, r[1], 0x4010ae68, 0x20000380ul);

}
int main(void)
{
		syscall(__NR_mmap, 0x1ffff000ul, 0x1000ul, 0ul, 0x32ul, -1, 0ul);
	syscall(__NR_mmap, 0x20000000ul, 0x1000000ul, 7ul, 0x32ul, -1, 0ul);
	syscall(__NR_mmap, 0x21000000ul, 0x1000ul, 0ul, 0x32ul, -1, 0ul);
	setup_fault();
			use_temporary_dir();
			loop();
	return 0;
}

