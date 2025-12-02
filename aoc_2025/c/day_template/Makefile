# Day-specific Makefile
CC = gcc
CFLAGS = -Wall -Wextra -std=c11 -O2 -g -I../include
CFLAGS_DEBUG = -Wall -Wextra -std=c11 -g -DDEBUG -I../include
INPUT_PATH = ../../inputs

YEAR = $(shell realpath "../../" | sed 's#.*/##' | sed 's/aoc_//')
DAY = $(shell realpath "./" | sed 's#.*/##' | sed 's/day_//')

# Directories
UTILS_DIR = ../include

# Object files
LIB_OBJ = lib.o
UTILS_OBJ = $(UTILS_DIR)/utils.o

# Binaries
PART1_BIN = part_1
PART2_BIN = part_2

# Default target
all: $(PART1_BIN) $(PART2_BIN)

# Build and run part 1
run1: $(PART1_BIN)
	@echo "Running Part 1..."
	@cat "$(INPUT_PATH)/day_$(DAY).input" | YEAR=$(YEAR) DAY=$(DAY) ./$(PART1_BIN)

# Build and run part 2
run2: $(PART2_BIN)
	@echo "Running Part 2..."
	@cat "$(INPUT_PATH)/day_$(DAY).input" | YEAR=$(YEAR) DAY=$(DAY) ./$(PART2_BIN)

# Convenience aliases
part1: run1
part2: run2

# Test with test input
test1: $(PART1_BIN)
	@echo "Running Part 1 with test input..."
	@YEAR=$(YEAR) DAY=$(DAY) @./$(PART1_BIN) test.txt

test2: $(PART2_BIN)
	@echo "Running Part 2 with test input..."
	@YEAR=$(YEAR) DAY=$(DAY)@./$(PART2_BIN) test.txt

# Build part 1 binary
$(PART1_BIN): part_1.o $(LIB_OBJ) $(UTILS_OBJ)
	$(CC) $(CFLAGS) -o $@ $^

# Build part 2 binary
$(PART2_BIN): part_2.o $(LIB_OBJ) $(UTILS_OBJ)
	$(CC) $(CFLAGS) -o $@ $^

# Compile object files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Compile utils
$(UTILS_OBJ): $(UTILS_DIR)/utils.c $(UTILS_DIR)/utils.h
	$(CC) $(CFLAGS) -c $< -o $@

# Clean build artifacts
clean:
	rm -f *.o $(PART1_BIN) $(PART2_BIN)
	rm -f $(UTILS_OBJ)

# Rebuild everything
rebuild: clean all

# Debug build
debug: CFLAGS = $(CFLAGS_DEBUG)
debug: clean all

# Show input
show-input:
	@cat $(INPUT)

.PHONY: all part1 part2 run1 run2 test1 test2 clean rebuild debug show-input
