default: run

run:
	npm run build
	./target/release/template_gui.exe

init:
	npm i
