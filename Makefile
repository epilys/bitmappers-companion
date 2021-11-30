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
	@du -sh build/bitgeom.pdf
	@mpv /usr/share/sounds/freedesktop/stereo/bell.oga > /dev/null
	@notify-send Done


clean:
	latexmk -outdir=build -auxdir=build -CA

gen-samples:
	convert -alpha remove -density 300 -quality 100 "build/bitgeom.pdf[$(pages)]" output.png

# convert news.png  -density 38 -units PixelsPerCentimeter news.png
