.PHONY: clean

README.md:
	./update-README.bash

clean:
	rm -f README.md
	rm -f examples/sample-files/*.mp3
