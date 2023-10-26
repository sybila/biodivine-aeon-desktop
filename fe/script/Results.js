let Results = {	

	clear() {
		document.getElementById("open-tree-explorer").classList.add("gone");
		document.getElementById("results").innerHTML = "";
	},

	hasResults() {
		return document.getElementById("results").getElementsByClassName("table-head").length > 0;
	},

	show() {
		UI.isLoading(true)
		ComputationEndpoints.getResults()
			.then((resultsObject) => {
				UI.isLoading(false)
				let isPartial = resultsObject["isPartial"]
				let isCancelled = resultsObject["isCancelled"]

				let result = resultsObject.data.sort((a, b) => b.sat_count - a.sat_count);
				if (!result) {
					return false;
				}

				const paramsNum = result.reduce((acc, curr) => acc + curr.sat_count, 0);
				//DomElements.statusPanels.result.innerText = 'total parametrizations: ' + paramsNum;

				var table = '';
				result.forEach(({ sat_count, phenotype })=> {
					var behavior = phenotype.map(x => x[0]).sort().join('');
					let behaviorString = behavior;
					if (behaviorString === 0) {
						behaviorString = "<span style=\"font-family: 'FiraMono'; letter-spacing: normal;\">unclassified</span>";
					}
					table += `
		            	<tr>
		            		<td class="table-behavior">${behaviorString}</td>
		            		<td class="table-sat-count">${sat_count}</td>
		            		<td><span class="inline-button" onclick="Windows.openWitnessWindow('${behavior}');">Witness</span></td>
		            		<td><span class="inline-button" onclick="Windows.openAttractorExplorerWindow('${behavior}');">Attractor</span></td>
		            	</tr>
		            `;
				});

				table = `
		        	<div class="center">Total number of classes: ${result.length}</div>
		        	<table>
		        		<tr class='table-head'>
		        			<td>Behavior<br>class</td>
		        			<td>Witness<br>count</td>
		        			<td></td>
		        		</tr>
		        		${table}
		        	</table>
		        `;
				if (isPartial) {	// if the computation is not finished, add
					table = "<h4 class='orange' style='text-align:center;'>Warning: These are partial results from an unfinished computation.</h4>" + table;
				} else {
					table = "<div class='center'>Elapsed: " + (resultsObject["elapsed"] / 1000) + "s</div>" + table;
				}

				// Show result window
				document.getElementById("results").innerHTML = table;
				document.getElementById("open-tree-explorer").classList.remove("gone");
				document.getElementById("tab-results").classList.remove("gone");

				// Hide "Show partial result" button after it is clicked on cancelled computation
				if (isCancelled) {
					document.getElementById("computation-download").classList.add("gone");
				}
			})
			.catch((errorMessage) => {
				UI.isLoading(false)
				Dialog.errorMessage(errorMessage)
			})
	},

}
