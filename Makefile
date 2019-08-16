fmt:
	docker run -t -w /mnt -v ${PWD}:/mnt rucc cargo fmt

check:
    docker run -t -w /mnt -v ${PWD}:/mnt rucc cargo check

build:
	docker run -t -w /mnt -v ${PWD}:/mnt rucc cargo build

run:
	docker run -t -w /mnt -v ${PWD}:/mnt rucc cargo run

tmp.s:
	docker run -i -w /mnt -v ${PWD}:/mnt rucc cargo run>tmp.s

tmp: tmp.s
	docker run -t -w /mnt -v ${PWD}:/mnt rucc gcc -o tmp tmp.s

exec: tmp
	docker run -t -w /mnt -v ${PWD}:/mnt rucc ./tmp
	echo $?

shell:
	docker run -t -w /mnt -v ${PWD}:/mnt rucc bash

test:
	docker run -t -w /mnt -v ${PWD}:/mnt rucc sh test.sh
	make clean

coverage:
	docker run -t -w /mnt -v ${PWD}:/mnt rucc sh coverage.sh

clean:
	rm -fv tmp*
