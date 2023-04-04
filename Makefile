.PHONY: clean

README.md:
	./update-README.bash

clean:
	rm -f README.md
