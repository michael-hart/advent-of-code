use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct GraphNode {
    val: String,
    links: Vec<Rc<Box<GraphNode>>>,
}

impl GraphNode {
    fn boxed_from(val: &str) -> Rc<Box<GraphNode>> {
        Rc::new(Box::new(GraphNode { val: val.to_owned(), links: vec![] }))
    }
}

fn parse_graph(raw: &str) -> HashMap<&str, Rc<Box<GraphNode>>> {
    let mut graph = HashMap::new();

    for line in raw.lines() {
        let arrow_idx = line.find("-").expect("Line does not contain a dash!");
        let left_key = line[..arrow_idx].trim();
        let right_key = line[arrow_idx + 1 ..].trim();

        let mut left_node = graph.entry(left_key).or_insert(GraphNode::boxed_from(left_key)).clone();
        println!("Got left node: {:?}", &left_node);
        let right_node = graph.entry(right_key).or_insert(GraphNode::boxed_from(right_key));

        left_node.as_mut().links.push(right_node.clone());
        // Rc::get_mut(&mut left_node).expect("No data here!").links.push(right_node.clone());
    }

    graph
}

fn main() {
    let graph_links = include_str!("../data/test.txt");
    let mut graph = parse_graph(graph_links);
    println!("{:?}", graph);
}
