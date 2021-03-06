export function histogram(svg, histData) {
    const width = svg.attr("width"),
        height = svg.attr("height")

    let xDomain = d3.extent(histData);

    let bins = d3.histogram().domain(xDomain).thresholds(20)(histData);

    let x = d3.scaleLinear()
        .domain(xDomain)
        .range([0, width]);

    let y = d3.scaleLinear()
        .domain([0, d3.max(bins, b => b.length)])
        .range([height, 0]);

    var xAxis = d3.axisBottom(x)
    var yAxis = d3.axisLeft(y)

    svg.append("g")
        .selectAll("rect")
        .data(bins)
        .enter()
        .append("rect")
        .attr("x", d => x(d.x0) + 1)
        .attr("y", d => y(d.length))
        .attr("width", d => x(d.x1) - x(d.x0) - 1)
        .attr("height", d => y(0) - y(d.length))
        .attr("class", "histbars");

    svg.select(".xaxis")
        .append("g")
        .attr("transform", "translate( 0," + height + ")")
        .call(xAxis)

        .selectAll("text")
        .style("text-anchor", "end")
        .attrs({
            dx: "-.8em",
            dy: ".15em"
        })
        .attr("transform", function (d) {
            return "rotate(-55)"
        });

    // never want yaxis on histogram
    // svg.select(".yaxis")
    //     .append("g")
    //     .call(yAxis);
}