
.PHONY: all
all: run clean

.PHONY: run
run: host
	./host

host: main.cpp
	g++ $^ -o $@ -ldqcsim -std=c++11

.PHONY: clean
clean:
	rm -f host
