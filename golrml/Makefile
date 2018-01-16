gol: gol.rml
	rmlc -thread gol.rml
	ocamlc -o gol -I `rmlc -where` -thread unix.cma threads.cma rmllib.cma graphics.cma gol.ml

planets: planets.rml
	rmlc -thread planets.rml
	ocamlc -o planets -I `rmlc -where` -thread unix.cma threads.cma rmllib.cma graphics.cma planets.ml

clean:
	rm -f gol.rzi gol.ml gol.cm? gol
	rm -f planets.rzi planets.ml planets.cm? planets

cleanall: clean
	rm -f gol *~
	rm -f planets *~
