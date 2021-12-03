#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

////////////////////////////////////////////////////////////////////////////////
/// Typ definitions
struct Robot {
  pthread_t *thread_struct;
  int robot_type;
  int operation_type;
  char *input1;
  char *input2;
  char *work_buf;
  long len1;
  long len2;
  bool done;
};
typedef struct Robot robot_t;

////////////////////////////////////////////////////////////////////////////////
/// Forward declarations
void watcher_thread(void);
void terminate_not_implemented(void);
void do_robot(robot_t *robot);
void do_num_add(robot_t *robot);
void do_str_mul(robot_t *robot);

////////////////////////////////////////////////////////////////////////////////
/// Globals
robot_t *robots[8];

////////////////////////////////////////////////////////////////////////////////
/// Functions
void create_robot(void) {
  char robot_type;
  char operation_type;
  int iVar1;
  robot_t *robot;
  char *string_buffer;
  void *pvVar2;
  int slot;
  int i;
  pthread_t *thread;
  long line_length;

  slot = -1;
  for (i = 0; i < 8; i = i + 1) {
    if (robots[i] == NULL) {
      slot = i;
      break;
    }
  }
  if (slot == -1) {
    puts("Error! No free parts!");
  } else {
    robot = (robot_t *)malloc(0x40);
    robots[slot] = robot;
    robots[slot]->done = false;
    do {
      printf("What kind of robot would you like? (n/s) > ");
      iVar1 = getchar();
      robot_type = (char)iVar1;
      getchar();
      if (robot_type == 'n')
        break;
    } while (robot_type != 's');
    do {
      printf("What kind of operation do you want? (a/s/m) > ");
      iVar1 = getchar();
      operation_type = (char)iVar1;
      getchar();
      if ((operation_type == 'a') || (operation_type == 's'))
        break;
    } while (operation_type != 'm');
    if (robot_type == 's') {
      robots[slot]->robot_type = 1;
      printf("Enter string 1: ");
      robot = robots[slot];
      string_buffer = (char *)malloc(0x100);
      robot->input1 = string_buffer;
      fgets(robots[slot]->input1, 0x100, stdin);
      pvVar2 = memchr(robots[slot]->input1, L'\n', 0x100);
      line_length = (long)pvVar2 - (long)robots[slot]->input1;
      robots[slot]->len1 = line_length;
      if (operation_type == 'a') {
        terminate_not_implemented();
      } else {
        if (operation_type != 'm') {
          terminate_not_implemented();
        }
        printf("Enter size: ");
        scanf("%ld", &robots[slot]->input2);
        getchar();
      }
    } else {
      if (robot_type == 'n') {
        robots[slot]->robot_type = 0;
        printf("Enter number 1: ");
        __isoc99_scanf("%ld", &robots[slot]->input1);
        getchar();
        printf("Enter number 2: ");
        __isoc99_scanf("%ld", &robots[slot]->input2);
        getchar();
      }
    }
    if (operation_type == 's') {
      robots[slot]->operation_type = 1;
    } else {
      if (operation_type < 't') {
        if (operation_type == 'a') {
          robots[slot]->operation_type = 0;
        } else {
          if (operation_type == 'm') {
            robots[slot]->operation_type = 2;
          }
        }
      }
    }
    pthread_create((pthread_t *)&thread, (pthread_attr_t *)0x0, do_robot,
                   robots[slot]);
    robots[slot]->thread_struct = thread;
  }
  return;
}

int main(void) {
  pthread_t thread;

  setvbuf(stdout, (char *)0x0, 2, 0);
  pthread_create(&thread, (pthread_attr_t *)0x0, watcher_thread, (void *)0x0);
  do {
    create_robot();
  } while (true);
}

void do_num_add(robot_t *robot) {
  char buf[8];
  robot->work_buf = buf;
  *robot->work_buf = (long)robot->input2 + (long)robot->input1;
  robot->done = true;
}

void do_str_mul(robot_t *robot) {
  long i;
  long offset;
  char buf[264];
  robot->work_buf = buf;

  if (robot->robot_type == 1) {
    memcpy(robot->work_buf, robot->input1, robot->len1);
    offset = robot->len1;
    for (i = 0; i < (long)robot->input2; i = i + 1) {
      memcpy(robot->work_buf + offset, robot->input1, robot->len1);
      offset = offset + robot->len1;
    }
  }
  robot->done = true;
}

void do_robot(robot_t *robot) {
  int op = robot->operation_type;
  int type = robot->robot_type;
  // Numeric add
  if (type == 0 && op == 0) {
    do_num_add(robot);
  }
  // String multiplication
  else if (type == 1 && op == 2) {
    do_str_mul(robot);
  } else {
    terminate_not_implemented();
  }
}

void watcher_thread(void) {
  int i;
  do {
    for (i = 0; i < 8; i = i + 1) {
      if ((robots[i] != (robot_t *)NULL) && (robots[i]->done != false)) {
        if (robots[i]->robot_type == 0) {
          printf("Result: %ld", robots[i]->work_buf);
        } else {
          if (robots[i]->robot_type == 1) {
            printf("Result: %s", robots[i]->work_buf);
          }
        }
        write(1, "\n", 2);
        free(robots[i]);
        robots[i] = (robot_t *)NULL;
      }
    }
    sleep(1);
  } while (true);
}

void terminate_not_implemented(void) {
  puts("Stripped, use official binary instead");
  exit(EXIT_FAILURE);
}
