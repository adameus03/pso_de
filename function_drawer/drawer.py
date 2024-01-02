import pandas
import plotly
import numpy
import glob
numpy.seterr(divide = 'ignore') 

for file in glob.glob("./*.csv"):
	frame = pandas.read_csv(file, sep=";")
	figure = plotly.graph_objects.Figure(data=plotly.graph_objects.Contour(
		x=frame["x"],
		y=frame["y"],
		z=numpy.log10(frame["z"]),
	))
	print(f"writing {file}")
	figure.update_layout(width = 1000, height = 1000)
	figure.write_image(file.replace("csv", "png"))
