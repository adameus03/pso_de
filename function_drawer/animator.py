import pandas
import plotly
import numpy
import glob
import json
numpy.seterr(divide = 'ignore') 

for file in glob.glob("./*.csv"):
	#try:
		frame = pandas.read_csv(file, sep=";")
		frames = None
		with open(f"../output/{file.replace('csv', 'json')}") as json_file:
			frames = json.load(json_file)
		figure = plotly.graph_objects.Figure(
			data=[
				plotly.graph_objects.Contour(
					x=frame["x"],
					y=frame["y"],
					z=numpy.log10(frame["z"]),
				),
			] * 2,
			frames=[
				plotly.graph_objects.Frame(
					data = plotly.graph_objects.Scatter(
						x = [particle["x"] for particle in frame],
						y = [particle["y"] for particle in frame],
						mode="markers",
					),
					traces=[1],
				) for frame in frames
			],
		)
		print(f"writing {file}")
		figure.update_layout(width = 1500, height = 1500)
		figure.write_html(file.replace("csv", "html"))
	#except:
	#	print(f"error for {file}")
