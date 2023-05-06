# [DS 210 Spring 2023] CRN Schaubild Analytica

Xiang Fu

## Introduction

**CRN Schaubild Analytica**, 

## Dataset

We are using [California Road Network](https://snap.stanford.edu/data/roadNet-CA.html) from [Stanford Network Analysis Project](https://snap.stanford.edu), which is a road network of California consisting of intersections and endpoints represented by nodes and the roads connecting these intersections or road endpoints represented by undirected edges. The dataset contains 1,965,206 nodes and 2,766,607 edges. The statistics for the largest weakly connected component (WCC) and strongly connected component (SCC) are shown below:

| Property | Value |
| --- | --- |
| Nodes in largest WCC | 1,957,027 |
| Edges in largest WCC | 2,760,388 |
| Nodes in largest SCC | 1,957,027 |
| Edges in largest SCC | 2,760,388 |
| Average clustering coefficient | 0.0464 |
| Number of triangles | 120,676 |
| Fraction of closed triangles | 0.02097 |
| Diameter (longest shortest path) | 849 |
| 90-percentile effective diameter | 500 |

The largest WCC and SCC represent a significant proportion of the road network, with over 99% of nodes and edges contained within them. The average clustering coefficient is 0.0464, indicating that the road network is moderately clustered. The number of triangles in the network is 120,676, and the fraction of closed triangles is 0.02097, suggesting that the road network has a significant number of interconnected clusters. The diameter of the road network, which is the longest shortest path between any two nodes, is 849, while the 90th percentile effective diameter is 500, indicating that the network has a relatively short average path length. These statistics can be used to guide the analysis and identify areas of the network that may be of particular interest or importance.

## Project Modules

- `main.rs`
- `assort.rs`
- `bed.rs`
- `cca.rs`
- `centrality_analysis.rs`
- `centrality.rs`
- `cpa.rs`
- `data.rs`
- `leiden.rs`
- `network_analysis.rs`
- `pagerank.rs`
- `shortest_path.rs`

## Results

### Assortativity

### PageRank

### Network Density

### Cluserting Coefficient

### Degree Distribution

### Connected Components

### Bridge Edges

### Core Periphery Analysis

### Leiden Community Detection

### Eigenvector Centrality

### Shortest Path with Landmarks

## Limitations

1. The California Road Network dataset only represents a portion of the state's road network, and it may not be fully representative of the transportation system as a whole. Future work could involve collecting additional data to better understand the road network and its properties.
2. The road network may change over time, which could affect the accuracy and relevance of the analysis. Future work could involve updating the analysis with more recent data and comparing the results to previous analyses.
3. The analysis focuses on the topological properties of the road network, but it does not take into account other factors that may affect transportation, such as weather, road conditions, or driver behaviour. Future work could involve integrating other types of data into the analysis to provide a more comprehensive understanding of the transportation system.
4. The analysis is limited to the California Road Network dataset, but the methods and techniques developed in this project could be applied to other road networks with similar properties. Future work could involve applying the same analysis to road networks in other states or countries to compare and contrast their properties.
5. The analysis may not capture the full complexity and nuances of the transportation system, and there may be other factors or variables that are important to consider. Future work could involve collaborating with experts in transportation planning and engineering to identify other factors that should be included in the analysis.

## Future Work

This project is a part of a larger project called "AnalyticaHub"

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details