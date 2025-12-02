# Makefile used to generate template code
include ../../.env

YEAR = 2025
TEMPLATE_DIR = day_template
QUERY_SCRIPT = ../../query_input.sh

# Create a new day from template
# Usage: make new DAY=1
new:
ifndef DAY
	$(error DAY is not set. Usage: make new day=1)
endif
ifndef AOC_SESSION_ID
	$(error AOC_SESSION_ID is not set.)
endif
	@if [ -d "day_$(DAY)" ]; then \
		echo "Error: day_$(DAY) already exists!"; \
		exit 1; \
	fi
	@echo "Creating day $(DAY)..."
	@mkdir -p day_$(DAY)
	@cp $(TEMPLATE_DIR)/lib.c day_$(DAY)/
	@cp $(TEMPLATE_DIR)/lib.h day_$(DAY)/
	@cp $(TEMPLATE_DIR)/part_1.c day_$(DAY)/
	@cp $(TEMPLATE_DIR)/part_2.c day_$(DAY)/
	@cp $(TEMPLATE_DIR)/.gitignore day_$(DAY)/
	@ln -s ../$(TEMPLATE_DIR)/Makefile day_$(DAY)/Makefile
	@echo "Fetching input for day $(DAY)..."
	@if [ ! -d "../inputs/day_$(DAY).input" ]; then \
		AOC_SESSION_ID=$(AOC_SESSION_ID) bash $(QUERY_SCRIPT) $(YEAR) $(DAY) "../inputs/day_$(DAY).input"; \
	fi
	@echo "Day $(DAY) created successfully!"
	@echo "Navigate to day_$(DAY) and run 'make part1' or 'make part2'"

# Clean all day directories
clean:
	@echo "Cleaning all day directories..."
	@for dir in day_*/; do \
		if [ -d "$$dir" ]; then \
			$(MAKE) -C $$dir clean 2>/dev/null || true; \
		fi \
	done
	@echo "Cleaning input files..."
		if [ -d "../inputs" ]; then \
			rm -rf ../inputs; \
			mkdir ../inputs; \
		fi
	@echo "Clean complete!"

# Run a specific day's part
# Usage: make run day=1 PART=1
run:
ifndef day
	$(error day is not set. Usage: make run day=1 PART=1)
endif
ifndef PART
	$(error PART is not set. Usage: make run day=1 PART=1)
endif
	@if [ ! -d "day_$(DAY)" ]; then \
		echo "Error: day_$(DAY) does not exist!"; \
		exit 1; \
	fi
	@$(MAKE) -C day_$(DAY) part$(PART)

# List all created days
list:
	@echo "Created days:"
	@ls -d day_*/ 2>/dev/null | sed 's/\///' || echo "No days created yet"

.PHONY: new clean run list
