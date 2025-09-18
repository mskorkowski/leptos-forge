run:
	trunk serve
build:
	trunk build  --cargo-profile release --public-url "https://mskorkowski.github.io/"
	# 404.html trick so we can always get the index page during navigation
	cp dist/index.html dist/404.html
clean:
	trunk clean
