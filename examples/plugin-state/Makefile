
.PHONY: all
all: run clean

.PHONY: run
run: plugin
	dqcsim plugin null

plugin: main.cpp
	g++ $^ -o $@ -ldqcsim -std=c++11

.PHONY: clean
clean:
	rm -f plugin
