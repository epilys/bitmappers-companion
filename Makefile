# You want latexmk to *always* run, because make does not have all the info.
# Also, include non-file targets in .PHONY so they are run regardless of any
# file of the given name existing.
.PHONY: bitgeom.pdf all clean

# The first rule in a Makefile is the one executed by default ("make"). It
# should always be the "all" rule, so that "make" and "make all" are identical.
all: bitgeom.pdf

# MAIN LATEXMK RULE

# -pdf tells latexmk to generate PDF directly (instead of DVI).
# -pdflatex="" tells latexmk to call a specific backend with specific options.
# -use-make tells latexmk to call make for generating missing files.

bitgeom.pdf: bitgeom.tex *.tex $(OUTFILES)
	latexmk -file-line-error -outdir=build -auxdir=build -e '$$max_repeat=2' -pdfxe  -pdfxelatex="xelatex -output-directory=build --shell-escape %O %S" -use-make bitgeom.tex
	@du -sh build/main.pdf
	@mpv --volume 35 /usr/share/sounds/freedesktop/stereo/bell.oga > /dev/null
	@notify-send Done


clean:
	latexmk -outdir=build -auxdir=build -CA

gen-samples:
	convert -alpha remove -density 300 -quality 100 "build/main.pdf[$(pages)]" output.png

style.pdf:
	pandoc style.md --from markdown+raw_attribute --pdf-engine=xelatex -o build/style.pdf

aoaom: aoaom.pdf

aoaom.pdf:
	#pandoc aoaom.md --toc --from markdown+raw_attribute+hard_line_breaks --pdf-engine=xelatex --pdf-engine-opt="-shell-escape" -o build/aoaom.pdf
	latexmk -file-line-error -outdir=build -auxdir=build -e '$$max_repeat=2' -pdfxe  -pdfxelatex="xelatex -halt-on-error -output-directory=build -interaction=batchmode --shell-escape %O %S" -use-make aoaom.tex
